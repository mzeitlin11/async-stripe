#!/usr/bin/env python3
"""Generate the pinned v2 experiment and verify disposable SDK workspaces.

No output is installed into the source checkout. All build caches stay outside
the workspaces. Optional --stage-only prepares compiler/benchmark inputs.
"""

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys

sys.dont_write_bytecode = True


REPO = Path(__file__).resolve().parents[2]
EVENTS = ["v2.core.event_destination", "v2.core.event"]
ACCOUNTS = ["v2.core.account", "v2.core.account_person", "v2.core.account_token",
            "v2.core.account_person_token", "v2.core.account_link"]


def run(command, cwd, env, log):
    print(log.name, flush=True)
    with log.open("w") as stream:
        stream.write(f"cwd: {cwd}\ncommand: {json.dumps(list(map(str, command)))}\n")
        stream.flush()
        result = subprocess.run(command, cwd=cwd, env=env, stdout=stream, stderr=subprocess.STDOUT)
    if result.returncode:
        raise RuntimeError(f"{log}:\n{log.read_text(errors='replace')[-6000:]}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--spec", type=Path, required=True)
    parser.add_argument("--work-dir", type=Path, required=True)
    parser.add_argument("--generator-target-dir", type=Path, required=True)
    parser.add_argument("--target-dir", type=Path, required=True)
    parser.add_argument("--stage-only", action="store_true")
    args = parser.parse_args()
    work = args.work_dir.resolve()
    if work.is_relative_to(REPO) or REPO.is_relative_to(work):
        parser.error("work directory must be outside the source repository")
    work.mkdir(parents=True, exist_ok=True)
    marker = work / ".v2-verification"
    if not marker.is_file() and any(work.iterdir()):
        parser.error("work directory must be empty or owned by this checker")
    marker.write_text("async-stripe v2 experimental verification\n")
    for target in [args.target_dir.resolve(), args.generator_target_dir.resolve()]:
        if target.is_relative_to(REPO) or any(target.is_relative_to(work / name / "workspace")
                                           for name in ["events", "combined"]):
            parser.error("target directory must be outside source workspaces")
    spec_bytes = args.spec.read_bytes()
    inputs = json.loads((REPO / "openapi/research/v2-architecture/inputs.json").read_text())
    expected = inputs["files"]["openapi.spec3.sdk.json"]["sha256"]
    if hashlib.sha256(spec_bytes).hexdigest() != expected:
        parser.error("experimental SDK input hash differs from the recorded pin")
    document = json.loads(spec_bytes)
    helper_spec = importlib.util.spec_from_file_location("normal_verifier", REPO / "openapi/tests/verify_generated_workspace.py")
    helper = importlib.util.module_from_spec(helper_spec)
    helper_spec.loader.exec_module(helper)
    before = helper.source_snapshot()
    summary = {"input_sha256": expected, "source_unchanged": False, "success": False, "cases": {}}
    try:
        for name, resources in [("events", EVENTS), ("combined", EVENTS + ACCOUNTS)]:
            case = work / name
            case.mkdir(exist_ok=True)
            output, workspace = case / "output", case / "workspace"
            gen_env = dict(os.environ, CARGO_TARGET_DIR=str(args.generator_target_dir.resolve()))
            run(["cargo", "+1.89.0", "run", "--locked", "--offline", "--example", "v2_slice", "--",
                 "--spec", args.spec.resolve(), "--out", output, "--resources", ",".join(resources)],
                REPO / "openapi", gen_env, case / "generation.log")
            if workspace.exists():
                shutil.rmtree(workspace)
            workspace.mkdir()
            for package in ["async-stripe-types", "async-stripe-client-core", "async-stripe"]:
                shutil.copytree(REPO / package, workspace / package)
            shutil.copytree(REPO / "generated", workspace / "generated")
            for package in (output / "crates").iterdir():
                # Empty reserved Shared output must not replace the v1 crate used for coexistence.
                if package.name != "async-stripe-shared":
                    shutil.copytree(package, workspace / "generated" / package.name)
                elif any(path.name != "mod.rs" for path in (package / "src").rglob("*.rs")):
                    raise RuntimeError("v2 models reached reserved legacy Shared; choose explicit v2 ownership before staging")
            shutil.copytree(REPO / "openapi/experiments/v2/consumer", workspace / "v2-experiment")
            fixtures = workspace / "v2-experiment/fixtures"
            fixtures.mkdir()
            for resource in EVENTS + ACCOUNTS:
                samples = document["components"]["schemas"][resource].get("x-stripeDocsSamples", [])
                if resource == "v2.core.account":
                    # The resource-level example omits required fields, while
                    # create/retrieve examples use timestamps for boolean applied.
                    # The pinned update example satisfies the pinned schema.
                    samples = document["paths"]["/v2/core/accounts/{id}"]["post"]["x-stripeDocsSamples"]
                if samples:
                    (fixtures / f"{resource.rsplit('.', 1)[-1]}.json").write_text(json.dumps(samples[0]["response"]))
            legacy = json.loads((REPO / "openapi/fixtures.json").read_text())["resources"]["account"]
            legacy["id"] = document["components"]["schemas"]["v2.core.account"]["x-stripeDocsSamples"][0]["response"]["id"]
            (fixtures / "v1-account.json").write_text(json.dumps(legacy))
            manifest = (REPO / "Cargo.toml").read_text()
            members = ["async-stripe", "async-stripe-client-core", "async-stripe-types", "generated/*", "v2-experiment"]
            for key in ["members", "default-members"]:
                manifest = helper.replace_array(manifest, key, members)
            (workspace / "Cargo.toml").write_text(manifest)
            for filename in ["Cargo.lock", ".rustfmt.toml", "README.md"]:
                shutil.copyfile(REPO / filename, workspace / filename)
            env = dict(os.environ, CARGO_TARGET_DIR=str(args.target_dir.resolve()), CARGO_BUILD_JOBS="2")
            run(["cargo", "+nightly", "fmt", "--all"], workspace, env, case / "format.log")
            result = {"workspace": str(workspace)}
            summary["cases"][name] = result
            if args.stage_only:
                continue
            features = ["--features", "accounts"] if name == "combined" else []
            run(["cargo", "+1.89.0", "check", "--offline", "-p", "v2-experiment", *features, "--message-format=json"],
                workspace, env, case / "check.log")
            sources = helper.compiled_generated_sources(case / "check.log", workspace)
            if any("async-stripe-shared/" in source for source in sources):
                raise RuntimeError("v2-only compilation pulled in v1 Shared")
            result["generated_compiler_sources"] = sources
            run(["cargo", "+1.89.0", "test", "--offline", "-p", "v2-experiment", *features],
                workspace, env, case / "wire.log")
            if name == "combined":
                run(["cargo", "+1.89.0", "test", "--offline", "-p", "v2-experiment", "--features", "v1"],
                    workspace, env, case / "coexistence.log")
            result["success"] = True
        summary["success"] = True
    finally:
        after = helper.source_snapshot()
        summary["source_unchanged"] = before == after
        if before != after:
            summary["success"] = False
        (work / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    if not summary["success"]:
        raise RuntimeError("experimental validation did not complete")
    print(f"V2 validation complete: {work}")


if __name__ == "__main__":
    main()
