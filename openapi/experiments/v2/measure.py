#!/usr/bin/env python3
"""Compare one identical Event Destination consumer against two verified slices.

Uses the existing compile-bench binary for sizes. LLVM diagnostics select the
consumer and v2 library separately, avoiding its hardcoded Shared diagnostic.
No benchmark executables are run; there are no timing samples.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess


MAIN = '''use std::hint::black_box;
fn main() {
    let value: stripe_v2::core::EventDestination =
        miniserde::json::from_str(black_box(include_str!("fixture.json"))).unwrap();
    black_box(value);
}
'''
MANIFEST = '''[package]
name = "bench-minimal"
version = "0.0.0"
edition.workspace = true
rust-version.workspace = true
publish = false

[dependencies]
miniserde.workspace = true
async-stripe-v2 = { path = "../../../../generated/async-stripe-v2" }
'''


def run(command, cwd, env, log):
    print(log.name, flush=True)
    with log.open("w") as stream:
        stream.write(f"cwd: {cwd}\ncommand: {json.dumps(list(map(str, command)))}\n")
        stream.flush()
        subprocess.run(command, cwd=cwd, env=env, stdout=stream, stderr=subprocess.STDOUT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--verified", type=Path, required=True)
    parser.add_argument("--collector", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    verified = args.verified.resolve()
    validation = json.loads((verified / "summary.json").read_text())
    if not validation["success"] or not validation["source_unchanged"]:
        parser.error("requires a successful isolated v2 verification")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    env = dict(os.environ, RUSTUP_TOOLCHAIN="1.89.0", CARGO_BUILD_JOBS="2")
    rustc = subprocess.check_output(["rustup", "which", "--toolchain", "1.89.0", "rustc"], text=True).strip()
    env["RUSTC"] = rustc
    results = {"input_sha256": validation["input_sha256"], "cases": {}}
    for name in ["events", "combined"]:
        source = verified / name / "workspace"
        case = output / name
        case.mkdir()
        workspace = case / "workspace"
        workspace.mkdir()
        for package in ["async-stripe-types", "async-stripe-client-core", "async-stripe", "generated"]:
            shutil.copytree(source / package, workspace / package)
        for filename in ["Cargo.lock", "README.md", ".rustfmt.toml"]:
            shutil.copyfile(source / filename, workspace / filename)
        manifest = (source / "Cargo.toml").read_text()
        members = ["async-stripe", "async-stripe-client-core", "async-stripe-types", "generated/*", "bench/compile/slices/minimal"]
        for key in ["members", "default-members"]:
            manifest = re.sub(rf"(?m)^{key}\s*=\s*\[[^\]]*\]", f"{key} = {json.dumps(members)}", manifest, count=1)
        (workspace / "Cargo.toml").write_text(manifest)
        fixture = workspace / "bench/compile/slices/minimal"
        (fixture / "src").mkdir(parents=True)
        (fixture / "Cargo.toml").write_text(MANIFEST)
        (fixture / "src/main.rs").write_text(MAIN)
        shutil.copyfile(source / "v2-experiment/fixtures/event_destination.json", fixture / "src/fixture.json")
        run(["cargo", "+1.89.0", "generate-lockfile", "--offline"], workspace, env, case / "lock.log")
        # The collector records git provenance. This commit describes the exact
        # disposable SDK inputs, independently of the implementation branch.
        run(["git", "init", "-q"], workspace, env, case / "git-init.log")
        run(["git", "add", "."], workspace, env, case / "git-add.log")
        run(["git", "-c", "commit.gpgsign=false", "commit", "-qm", "Record generated v2 measurement inputs"],
            workspace, env, case / "git-commit.log")
        run([args.collector.resolve(), "collect", "--repo", workspace, "--cases", "minimal",
             "--profiles", "dev,release", "--offline", "--keep-artifacts", "executable",
             "--output", case / "sizes"], workspace, env, case / "collector.log")
        report = json.loads((case / "sizes/results.json").read_text())
        for row in report["measurements"]:
            if row["timing_samples"]:
                raise RuntimeError("unexpected timing measurement")
            if any(lib["package"] == "async-stripe-shared" for lib in row["sizes"]["libraries"]):
                raise RuntimeError("v2 measurement compiled v1 Shared")
        totals = {}
        for package, kind in [("bench-minimal", "consumer"), ("async-stripe-v2", "library")]:
            target = case / f"ir-{kind}-target"
            flags = report["environment"]["rustflags"] + [
                f"--remap-path-prefix={workspace}=/workspace", f"--remap-path-prefix={target}=/build",
                f"--remap-path-prefix={Path(os.environ.get('CARGO_HOME', str(Path.home() / '.cargo')))}=/cargo",
            ]
            ir_env = dict(env, CARGO_TARGET_DIR=str(target), CARGO_INCREMENTAL="0", RUSTC_WRAPPER="",
                          RUSTC_WORKSPACE_WRAPPER="", CARGO_ENCODED_RUSTFLAGS="\x1f".join(flags),
                          CARGO_PROFILE_RELEASE_SPLIT_DEBUGINFO="off", CARGO_PROFILE_RELEASE_STRIP="none")
            ir_env.pop("RUSTFLAGS", None)
            command = ["cargo", "+1.89.0", "llvm-lines", "--locked", "--offline", "--profile", "release",
                       "--target", report["environment"]["target"], "-p", package]
            command += ["--lib"] if kind == "library" else ["--bin", package]
            log = case / f"{kind}-ir.log"
            run(command, workspace, ir_env, log)
            for line in log.read_text().splitlines():
                fields = line.split()
                if len(fields) > 2 and fields[2] == "(TOTAL)":
                    totals[kind] = {"lines": int(fields[0]), "copies": int(fields[1])}
                    break
            if kind not in totals:
                raise RuntimeError(f"LLVM diagnostic has no TOTAL row: {log}")
            shutil.rmtree(target)
        results["cases"][name] = {"sizes": report, "release_ir": totals,
                                   "consumer_fixture_sha256": hashlib.sha256((fixture / "src/fixture.json").read_bytes()).hexdigest()}
        (output / "results.json").write_text(json.dumps(results, indent=2) + "\n")
    run([args.collector.resolve(), "compare", output / "events/sizes", output / "combined/sizes",
         "--output", output / "comparison.md"], output, env, output / "compare.log")
    print(f"Artifact and IR measurements: {output}")


if __name__ == "__main__":
    main()
