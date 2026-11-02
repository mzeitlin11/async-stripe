#!/usr/bin/env python3
"""Verify opaque model groups in an isolated consumer workspace.

Logs and source-closure results are retained in --work-dir. Private output/build
trees are removed after success. Explicit target directories are caller-owned
caches and are retained. No product files are installed.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile


REPO = Path(__file__).resolve().parents[2]
MEMBERS = ["async-stripe-types", "async-stripe-client-core", "async-stripe", "generated/*"]


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
    start = re.search(rf"(?m)^{re.escape(key)}\s*=\s*\[", manifest)
    if start is None:
        raise AssertionError(f"workspace manifest lacks {key}")
    end = manifest.index("]", start.end()) + 1
    return manifest[: start.start()] + f"{key} = {json.dumps(values)}" + manifest[end:]


def run(work, name, command, cwd, env, expect_success=True):
    print(f"{name}: {' '.join(map(str, command))}", flush=True)
    result = subprocess.run(command, cwd=cwd, env=env, text=True, capture_output=True)
    (work / f"{name}.log").write_text(result.stdout + result.stderr)
    if (result.returncode == 0) != expect_success:
        raise AssertionError(f"{name}: unexpected exit {result.returncode}\n{result.stderr[-6000:]}")
    return result


def compiled_shared_sources(stdout, workspace):
    artifacts = []
    for line in stdout.splitlines():
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        if message.get("reason") == "compiler-artifact" and message.get("target", {}).get("name") == "stripe_shared":
            artifacts.extend(message["filenames"])
    assert artifacts, "Cargo reported no compiled shared artifact"
    info = set()
    for filename in artifacts:
        artifact = Path(filename)
        dep_info = artifact.with_name(artifact.name.removeprefix("lib").rsplit(".", 1)[0] + ".d")
        if dep_info.is_file():
            info.add(dep_info.read_text())
    assert info, "shared crate dependency information was not found"
    contents = "\n".join(info)
    sources = []
    for source in (workspace / "generated/async-stripe-shared/src").rglob("*.rs"):
        escaped = str(source).replace(" ", "\\ ")
        if str(source) in contents or escaped in contents:
            sources.append(str(source.relative_to(workspace)))
    assert sources, "shared dependency information contains no generated sources"
    return sorted(sources)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work-dir", type=Path)
    parser.add_argument("--target-dir", type=Path, help="reusable consumer build cache")
    parser.add_argument("--generator-target-dir", type=Path, help="reusable generator build cache")
    parser.add_argument("--toolchain", default="1.89.0")
    args = parser.parse_args()
    work = (args.work_dir or Path(tempfile.mkdtemp(prefix="async-stripe-opaque-check-"))).resolve()
    work.mkdir(parents=True, exist_ok=True)
    marker = work / ".opaque-groups-verification"
    if not marker.exists() and any(work.iterdir()):
        raise AssertionError(f"verification work directory must be empty or managed: {work}")
    marker.write_text("opaque model group verification\n")
    workspace, output = work / "workspace", work / "generated-output"
    for path in [workspace, output]:
        if path.exists():
            shutil.rmtree(path)
    generator_target = (args.generator_target_dir or work / "generator-target").resolve()
    target = (args.target_dir or work / "consumer-target").resolve()
    before = source_snapshot()
    env = dict(os.environ, CARGO_TARGET_DIR=str(generator_target))
    groups = ["balance_transaction", "charge", "invoice", "balance_transaction_source"]
    run(work, "generation", ["cargo", f"+{args.toolchain}", "run", "--locked", "--offline", "--example", "opaque_model_groups", "--", str(output), *groups], REPO / "openapi", env)

    workspace.mkdir()
    for crate in MEMBERS[:-1]:
        shutil.copytree(REPO / crate, workspace / crate, ignore=shutil.ignore_patterns("target"))
    shutil.copytree(output / "crates", workspace / "generated")
    for relative in ["async-stripe/Cargo.toml", "async-stripe/src/generated/mod.rs"]:
        shutil.copyfile(output / relative, workspace / relative)
    manifest = (REPO / "Cargo.toml").read_text()
    for key in ["members", "default-members"]:
        manifest = replace_array(manifest, key, MEMBERS)
    (workspace / "Cargo.toml").write_text(manifest)
    shutil.copyfile(REPO / "Cargo.lock", workspace / "Cargo.lock")
    fixture_relative = Path("openapi/tests/fixtures/opaque-consumer")
    fixture = workspace / fixture_relative
    shutil.copytree(REPO / fixture_relative, fixture, ignore=shutil.ignore_patterns("target", "Cargo.lock"))
    shutil.copyfile(REPO / "Cargo.lock", fixture / "Cargo.lock")
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), CARGO_PROFILE_DEV_DEBUG="0", CARGO_INCREMENTAL="0", CARGO_BUILD_JOBS="2")
    run(work, "format", ["cargo", "+nightly", "fmt", "--all"], workspace, env)
    run(work, "fixture-format", ["cargo", "+nightly", "fmt"], fixture, env)
    cargo = ["cargo", f"+{args.toolchain}"]
    base = run(work, "base", cargo + ["test", "--offline", "--message-format=json"], fixture, env)
    base_sources = compiled_shared_sources(base.stdout, workspace)
    charge_file = "generated/async-stripe-shared/src/charge.rs"
    assert charge_file not in base_sources, "base feature compiled the full Charge model"
    assert "generated/async-stripe-shared/src/balance_transaction.rs" in base_sources
    (work / "base-compiled-sources.json").write_text(json.dumps(base_sources, indent=2) + "\n")
    negative = run(work, "charge-unavailable", cargo + ["check", "--offline", "--bin", "charge-compile-probe", "--features", "charge-compile-probe"], fixture, env, expect_success=False)
    assert "unresolved import" in negative.stderr and "stripe::Charge" in negative.stderr, negative.stderr
    charge = run(work, "charge", cargo + ["test", "--offline", "--features", "charge", "--message-format=json"], fixture, env)
    charge_sources = compiled_shared_sources(charge.stdout, workspace)
    assert charge_file in charge_sources, "charge feature did not compile the Charge model"
    (work / "charge-compiled-sources.json").write_text(json.dumps(charge_sources, indent=2) + "\n")
    run(work, "charge-available", cargo + ["check", "--offline", "--bin", "charge-compile-probe", "--features", "charge-compile-probe,charge"], fixture, env)
    charge_only = run(work, "charge-only", cargo + ["test", "--offline", "--no-default-features", "--features", "charge", "--message-format=json"], fixture, env)
    charge_only_sources = compiled_shared_sources(charge_only.stdout, workspace)
    assert charge_file in charge_only_sources
    assert "generated/async-stripe-shared/src/balance_transaction.rs" not in charge_only_sources, "charge alone compiled the balance transaction model"
    run(work, "model-only", cargo + ["test", "--offline", "--no-default-features", "--features", "balance_transaction_source"], fixture, env)
    run(work, "combined-groups", cargo + ["test", "--offline", "--no-default-features", "--features", "charge,invoice"], fixture, env)
    run(work, "all-features", cargo + ["test", "--offline", "--all-features"], fixture, env)
    run(work, "http-client", cargo + ["check", "--offline", "--features", "stripe/default-tls"], fixture, env)
    bare = run(work, "bare-serialize", cargo + ["check", "--offline", "--manifest-path", str(workspace / "async-stripe/Cargo.toml"), "--no-default-features", "--features", "serialize", "--message-format=json"], workspace, env)
    for line in bare.stdout.splitlines():
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        assert message.get("target", {}).get("name") != "stripe_core", "serialization forwarding activated the optional API crate"
    assert source_snapshot() == before, "verification modified product source"
    summary = {"base_shared_source_files": len(base_sources), "charge_shared_source_files": len(charge_sources), "charge_only_shared_source_files": len(charge_only_sources), "charge_absent_from_base": True, "source_unchanged": True}
    (work / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    for path in [workspace, output]:
        shutil.rmtree(path)
    for explicit, path in [(args.target_dir, target), (args.generator_target_dir, generator_target)]:
        if explicit is None and path.exists():
            shutil.rmtree(path)
    print(f"Verified opaque model groups. Logs and results: {work}", flush=True)


if __name__ == "__main__":
    main()
