#!/usr/bin/env python3
"""Generate bindings and verify them in an isolated SDK workspace.

Requires Python 3.11+, the selected Rust toolchain, nightly rustfmt, and cached
Cargo dependencies/spec data. Logs and analysis reports survive successful runs.
Source files are never installed. Failed workspaces remain available for review.
"""

import argparse
from collections import Counter
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import tomllib


REPO = Path(__file__).resolve().parents[2]
RUNTIME_PACKAGES = [
    "async-stripe-types",
    "async-stripe-client-core",
    "async-stripe",
    "async-stripe-webhook",
    "async-stripe-parser",
    "tests",
]
MARKER = ".generated-workspace-verification"


def source_snapshot():
    names = subprocess.check_output(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], cwd=REPO
    ).split(b"\0")
    return {
        os.fsdecode(name): hashlib.sha256((REPO / os.fsdecode(name)).read_bytes()).hexdigest()
        for name in names
        if name and (REPO / os.fsdecode(name)).is_file()
    }


def replace_array(manifest, key, values):
    pattern = rf"(?m)^{re.escape(key)}\s*=\s*\[[^\]]*\]"
    manifest, count = re.subn(pattern, lambda _: f"{key} = {json.dumps(values)}", manifest)
    if count != 1:
        raise RuntimeError(f"workspace manifest must contain one {key} array")
    return manifest


def run(work, name, command, cwd, env):
    display = shlex.join(map(str, command))
    print(f"{name}: {display}", flush=True)
    log = work / f"{name}.log"
    with log.open("w") as stream:
        stream.write(f"cwd: {cwd}\n$ {display}\n\n")
        stream.flush()
        result = subprocess.run(command, cwd=cwd, env=env, stdout=stream, stderr=subprocess.STDOUT)
    if result.returncode:
        tail = log.read_text(errors="replace")[-6000:]
        raise RuntimeError(f"{name} failed with exit {result.returncode}; log: {log}\n{tail}")


def compiled_generated_sources(log, workspace):
    """Read rustc dep-info for generated packages, excluding runtime packages."""
    artifacts = []
    for line in log.read_text(errors="replace").splitlines():
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not isinstance(message, dict):
            continue
        source = message.get("target", {}).get("src_path", "")
        if message.get("reason") == "compiler-artifact" and source.startswith(str(workspace / "generated") + "/"):
            artifacts.extend(message["filenames"])
    dependency_files = set()
    for filename in artifacts:
        artifact = Path(filename)
        dep_info = artifact.with_name(artifact.name.removeprefix("lib").rsplit(".", 1)[0] + ".d")
        if dep_info.is_file():
            dependency_files.add(dep_info)
    if not dependency_files:
        raise RuntimeError(f"no generated-package rustc dep-info found from {log}")
    dependencies = "\n".join(path.read_text() for path in dependency_files)
    sources = []
    for source in (workspace / "generated").rglob("*.rs"):
        relative = source.relative_to(workspace)
        # Selected workspace packages use relative paths; dependency builds can
        # use absolute paths. Dep-info escapes spaces in either representation.
        paths = [str(source), str(relative)]
        if any(path in dependencies or path.replace(" ", "\\ ") in dependencies for path in paths):
            sources.append(str(relative))
    if not sources:
        raise RuntimeError(f"rustc dep-info contains no generated source files: {log}")
    return sorted(sources)


def remove_directory(path):
    if path.is_symlink():
        raise RuntimeError(f"refusing to remove a symlink: {path}")
    if path.exists():
        shutil.rmtree(path)


def stage_workspace(workspace, output):
    workspace.mkdir()
    for package in RUNTIME_PACKAGES:
        shutil.copytree(REPO / package, workspace / package, ignore=shutil.ignore_patterns("target"))
    shutil.copytree(output / "crates", workspace / "generated")
    shutil.copyfile(
        output / "async-stripe-webhook/mod.rs", workspace / "async-stripe-webhook/src/generated/mod.rs"
    )
    shutil.copyfile(output / "tests/mod.rs", workspace / "tests/tests/it/generated/mod.rs")
    (workspace / "openapi").mkdir()
    shutil.copyfile(REPO / "openapi/fixtures.json", workspace / "openapi/fixtures.json")
    members = RUNTIME_PACKAGES + ["generated/*"]
    manifest = (REPO / "Cargo.toml").read_text()
    for key in ["members", "default-members"]:
        manifest = replace_array(manifest, key, members)
    (workspace / "Cargo.toml").write_text(manifest)
    for name in ["Cargo.lock", ".rustfmt.toml", "README.md"]:
        shutil.copyfile(REPO / name, workspace / name)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work-dir", type=Path, help="empty or previously managed directory outside the repository")
    parser.add_argument("--target-dir", type=Path, help="reusable SDK build cache; retained after success")
    parser.add_argument("--generator-target-dir", type=Path, help="reusable generator build cache; retained after success")
    parser.add_argument("--toolchain", default="1.89.0", help="Rust toolchain for generation and SDK validation (default: %(default)s)")
    parser.add_argument("--keep-workspace", action="store_true", help="retain formatted SDK sources after success")
    args = parser.parse_args()
    work = (args.work_dir or Path(tempfile.mkdtemp(prefix="async-stripe-generated-check-"))).resolve()
    if work.is_relative_to(REPO) or REPO.is_relative_to(work):
        parser.error("--work-dir must be outside the repository and cannot contain it")
    work.mkdir(parents=True, exist_ok=True)
    marker = work / MARKER
    if not marker.is_file() and any(work.iterdir()):
        parser.error(f"verification work directory must be empty or managed: {work}")
    marker.write_text("async-stripe generated workspace verification\n")
    workspace, output, reports = work / "workspace", work / "generated-output", work / "analysis"
    target = (args.target_dir or work / "sdk-target").resolve()
    generator_target = (args.generator_target_dir or work / "generator-target").resolve()
    for cache in [target, generator_target]:
        for owned in [workspace, output, reports]:
            if cache.is_relative_to(owned) or owned.is_relative_to(cache):
                parser.error(f"build cache overlaps disposable source/report directory: {cache}")
    # Only our marked work directory owns disposable SDK sources. Codegen
    # independently checks its staging marker before replacing generated output.
    remove_directory(workspace)
    before = source_snapshot()
    summary = {"toolchain": args.toolchain, "completed_checks": [], "source_unchanged": False, "success": False}
    error = None
    try:
        cargo = ["cargo", f"+{args.toolchain}"]
        generator_env = dict(os.environ, CARGO_TARGET_DIR=str(generator_target))
        run(work, "generation", cargo + ["run", "--locked", "--offline", "--", "generate", "--no-install", "--out", str(output), "--report-dir", str(reports)], REPO / "openapi", generator_env)
        summary["completed_checks"].append("generation")
        configuration = tomllib.loads((REPO / "openapi/gen_crates.toml").read_text())
        packages = sorted(f"async-stripe-{crate['name']}" for crate in configuration["crates"])
        actual = sorted(path.name for path in (output / "crates").iterdir() if path.is_dir())
        if packages != actual:
            raise RuntimeError(f"generated packages differ from gen_crates.toml: expected {packages}, found {actual}")
        for package in packages:
            for relative in ["Cargo.toml", "src/mod.rs"]:
                if not (output / "crates" / package / relative).is_file():
                    raise RuntimeError(f"generated package is incomplete: {package}/{relative}")
        summary["generated_packages"] = packages
        stage_workspace(workspace, output)
        env = dict(os.environ, CARGO_TARGET_DIR=str(target), CARGO_PROFILE_DEV_DEBUG="0", CARGO_INCREMENTAL="0", CARGO_BUILD_JOBS="2")
        selected = [argument for package in packages + ["async-stripe-webhook"] for argument in ["-p", package]]
        commands = [
            ("format", ["cargo", "+nightly", "fmt", "--all"]),
            ("format-check", ["cargo", "+nightly", "fmt", "--all", "--check"]),
            ("generated-and-webhook-check", cargo + ["check", "--offline", *selected, "--all-features", "--message-format=json"]),
            ("webhook-tests", cargo + ["test", "--offline", "-p", "async-stripe-webhook", "--all-features"]),
            ("fixtures", cargo + ["test", "--offline", "-p", "async-stripe-tests", "--test", "it", "deserialization_fixture::fixtures", "--", "--exact", "--nocapture"]),
            ("deserialization-tests", cargo + ["test", "--offline", "-p", "async-stripe-tests", "--test", "it", "deser::", "--", "--nocapture"]),
            ("parser-tests", cargo + ["test", "--offline", "-p", "async-stripe-parser"]),
        ]
        for name, command in commands:
            run(work, name, command, workspace, env)
            summary["completed_checks"].append(name)
            if name == "generated-and-webhook-check":
                sources = compiled_generated_sources(work / f"{name}.log", workspace)
                counts = dict(sorted(Counter(Path(source).parts[1] for source in sources).items()))
                if set(counts) != set(packages):
                    raise RuntimeError(f"compiled source measurement missed generated packages: {counts}")
                (work / "compiled-generated-crate-sources.json").write_text(json.dumps(sources, indent=2) + "\n")
                summary["compiled_generated_crate_source_files"] = len(sources)
                summary["compiled_generated_crate_source_files_by_package"] = counts
        summary["success"] = True
    except Exception as exc:
        error = str(exc)
    finally:
        after = source_snapshot()
        changed = sorted(name for name in before.keys() | after.keys() if before.get(name) != after.get(name))
        summary["source_unchanged"] = not changed
        if changed:
            summary["success"] = False
            summary["changed_source_files"] = changed
            error = (error + "\n" if error else "") + "verification changed repository files: " + ", ".join(changed)
        if error:
            summary["error"] = error
        summary["workspace_retained"] = args.keep_workspace or not summary["success"]
        (work / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    if not summary["success"]:
        print(f"Verification failed. Workspace and logs retained: {work}\n{error}", file=sys.stderr)
        return 1
    remove_directory(output)
    if not args.keep_workspace:
        remove_directory(workspace)
    retained = {path.resolve() for path in [args.target_dir, args.generator_target_dir] if path is not None}
    for explicit, cache in [(args.target_dir, target), (args.generator_target_dir, generator_target)]:
        if explicit is None and not any(path.is_relative_to(cache) for path in retained):
            remove_directory(cache)
    print(f"Verified {len(packages)} generated crates, webhook, fixtures, and parser. Reports and logs: {work}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
