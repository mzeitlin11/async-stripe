#!/usr/bin/env python3
"""Verify the one-field opaque storage probe outside the source checkout.

Usage: python3 openapi/tests/verify_opaque_landing_slice.py WORK GENERATOR_TARGET SDK_TARGET
The WORK directory must be empty on the first run; logs and staged SDK are retained.
"""
import json
import os
from pathlib import Path
import runpy
import shutil
import sys

helper = runpy.run_path(str(Path(__file__).with_name("verify_generated_workspace.py")))
repo = helper["REPO"]
work, generator_target, sdk_target = map(lambda value: Path(value).resolve(), sys.argv[1:])
assert not work.is_relative_to(repo) and not repo.is_relative_to(work)
work.mkdir(parents=True, exist_ok=True)
marker = work / ".opaque-landing-probe"
assert marker.is_file() or not any(work.iterdir()), "work directory is not owned by this probe"
marker.write_text("one-field opaque storage probe\n")
workspace, output = work / "workspace", work / "output"
helper["remove_directory"](workspace)
before = helper["source_snapshot"]()
env = dict(os.environ, CARGO_TARGET_DIR=str(generator_target))
helper["run"](work, "generation", ["cargo", "+1.89.0", "run", "--locked", "--offline", "--", "generate", "--no-install", "--out", str(output)], repo / "openapi", env)
helper["stage_workspace"](workspace, output)
package = workspace / "opaque-landing-consumer"
(package / "src").mkdir(parents=True)
(package / "Cargo.toml").write_text('''[package]
name = "opaque-landing-consumer"
version = "0.0.0"
edition = "2024"
publish = false
[dependencies]
async-stripe-shared = { path = "../generated/async-stripe-shared", features = ["serialize", "deserialize"] }
async-stripe-types = { path = "../async-stripe-types", features = ["serialize", "deserialize"] }
serde.workspace = true
serde_json.workspace = true
miniserde.workspace = true
''')
shutil.copyfile(Path(__file__).parent / "fixtures/opaque_landing_slice.rs", package / "src/lib.rs")
manifest = workspace / "Cargo.toml"
manifest.write_text(helper["replace_array"](manifest.read_text(), "members", helper["RUNTIME_PACKAGES"] + ["generated/*", "opaque-landing-consumer"]))
env = dict(os.environ, CARGO_TARGET_DIR=str(sdk_target), CARGO_PROFILE_DEV_DEBUG="0", CARGO_INCREMENTAL="0", CARGO_BUILD_JOBS="2")
helper["run"](work, "consumer-tests", ["cargo", "+1.89.0", "test", "--offline", "-p", "opaque-landing-consumer"], workspace, env)
helper["run"](work, "consumer-clippy", ["cargo", "+1.89.0", "clippy", "--offline", "-p", "opaque-landing-consumer", "--tests", "--", "-D", "warnings"], workspace, env)
assert before == helper["source_snapshot"](), "source changed during verification"
(work / "summary.json").write_text(json.dumps({"success": True, "source_unchanged": True, "field": "charge.customer", "tests": 4, "runtime_source_commit": "176c3f588c4e1553cad274c90f2f1a4a28e41772"}, indent=2) + "\n")
print(f"Verified opaque Charge.customer storage and optional local decoding: {work}")
