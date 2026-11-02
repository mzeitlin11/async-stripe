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
MEMBERS = ["async-stripe-types", "async-stripe-client-core", "async-stripe", "async-stripe-webhook", "async-stripe-parser", "generated/*"]


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
    parser.add_argument("--full-v1", action="store_true", help="generate every v1 group and run exhaustive fixtures and parser tests")
    args = parser.parse_args()
    work = (args.work_dir or Path(tempfile.mkdtemp(prefix="async-stripe-opaque-check-"))).resolve()
    work.mkdir(parents=True, exist_ok=True)
    marker = work / ".opaque-groups-verification"
    if not marker.exists() and any(work.iterdir()):
        raise AssertionError(f"verification work directory must be empty or managed: {work}")
    marker.write_text("opaque model group verification\n")
    workspace, output = work / "workspace", work / "generated-output"
    for path in [workspace, output, work / "analysis"]:
        if path.exists():
            shutil.rmtree(path)
    generator_target = (args.generator_target_dir or work / "generator-target").resolve()
    target = (args.target_dir or work / "consumer-target").resolve()
    before = source_snapshot()
    env = dict(os.environ, CARGO_TARGET_DIR=str(generator_target))
    groups = ["--all"] if args.full_v1 else ["balance_transaction", "charge", "invoice", "balance_transaction_source", "application", "deleted_customer", "customer"]
    run(work, "generation", ["cargo", f"+{args.toolchain}", "run", "--locked", "--offline", "--example", "opaque_model_groups", "--", str(output), *groups], REPO / "openapi", env)
    shutil.copytree(output / "analysis", work / "analysis")

    workspace.mkdir()
    members = MEMBERS + (["tests"] if args.full_v1 else [])
    for crate in [member for member in members if member != "generated/*"]:
        shutil.copytree(REPO / crate, workspace / crate, ignore=shutil.ignore_patterns("target"))
    shutil.copytree(output / "crates", workspace / "generated")
    for relative in ["async-stripe/Cargo.toml", "async-stripe/src/generated/mod.rs", "async-stripe-webhook/Cargo.toml"]:
        shutil.copyfile(output / relative, workspace / relative)
    shutil.copyfile(output / "async-stripe-webhook/mod.rs", workspace / "async-stripe-webhook/src/generated/mod.rs")
    if args.full_v1:
        shutil.copyfile(output / "tests/Cargo.toml", workspace / "tests/Cargo.toml")
        shutil.copyfile(output / "tests/mod.rs", workspace / "tests/tests/it/generated/mod.rs")
        (workspace / "openapi").mkdir(exist_ok=True)
        shutil.copyfile(REPO / "openapi/fixtures.json", workspace / "openapi/fixtures.json")
    manifest = (REPO / "Cargo.toml").read_text()
    for key in ["members", "default-members"]:
        manifest = replace_array(manifest, key, members)
    (workspace / "Cargo.toml").write_text(manifest)
    shutil.copyfile(REPO / "Cargo.lock", workspace / "Cargo.lock")
    fixture_relative = Path("openapi/tests/fixtures/opaque-consumer")
    fixture = workspace / fixture_relative
    shutil.copytree(REPO / fixture_relative, fixture, ignore=shutil.ignore_patterns("target", "Cargo.lock"))
    shutil.copyfile(REPO / "Cargo.lock", fixture / "Cargo.lock")
    webhook_relative = Path("openapi/tests/fixtures/opaque-webhook-consumer")
    webhook_fixture = workspace / webhook_relative
    shutil.copytree(REPO / webhook_relative, webhook_fixture, ignore=shutil.ignore_patterns("target", "Cargo.lock"))
    shutil.copyfile(REPO / "Cargo.lock", webhook_fixture / "Cargo.lock")
    env = dict(os.environ, CARGO_TARGET_DIR=str(target), CARGO_PROFILE_DEV_DEBUG="0", CARGO_INCREMENTAL="0", CARGO_BUILD_JOBS="2")
    run(work, "format", ["cargo", "+nightly", "fmt", "--all"], workspace, env)
    run(work, "fixture-format", ["cargo", "+nightly", "fmt"], fixture, env)
    run(work, "webhook-fixture-format", ["cargo", "+nightly", "fmt"], webhook_fixture, env)
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
    card_file = "generated/async-stripe-shared/src/payment_method_details_card.rs"
    ideal_file = "generated/async-stripe-shared/src/payment_method_details_ideal.rs"
    assert card_file not in charge_sources and ideal_file not in charge_sources, "Charge compiled optional detail decoders"
    (work / "charge-compiled-sources.json").write_text(json.dumps(charge_sources, indent=2) + "\n")
    run(work, "charge-available", cargo + ["check", "--offline", "--bin", "charge-compile-probe", "--features", "charge-compile-probe,charge"], fixture, env)
    no_card = run(work, "record-card-unavailable", cargo + ["check", "--offline", "--no-default-features", "--bin", "record-card-compile-probe", "--features", "charge,record-card-compile-probe"], fixture, env, expect_success=False)
    assert "no method named" in no_card.stderr and "`card`" in no_card.stderr, no_card.stderr
    card = run(work, "record-card", cargo + ["test", "--offline", "--no-default-features", "--features", "charge,payment_method_details_card", "--message-format=json"], fixture, env)
    card_sources = compiled_shared_sources(card.stdout, workspace)
    assert card_file in card_sources and ideal_file not in card_sources
    run(work, "record-card-available", cargo + ["check", "--offline", "--no-default-features", "--bin", "record-card-compile-probe", "--features", "record-card-compile-probe,payment_method_details_card"], fixture, env)
    card_only = run(work, "record-card-only", cargo + ["test", "--offline", "--no-default-features", "--features", "payment_method_details_card", "--message-format=json"], fixture, env)
    assert charge_file not in compiled_shared_sources(card_only.stdout, workspace), "standalone card decoder compiled Charge"
    for line in card_only.stdout.splitlines():
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        assert message.get("target", {}).get("name") != "stripe_core", "standalone card decoder activated HTTP request package"
    ideal = run(work, "record-ideal", cargo + ["test", "--offline", "--no-default-features", "--features", "charge,payment_method_details_ideal", "--message-format=json"], fixture, env)
    ideal_sources = compiled_shared_sources(ideal.stdout, workspace)
    assert ideal_file in ideal_sources and card_file not in ideal_sources
    charge_only = run(work, "charge-only", cargo + ["test", "--offline", "--no-default-features", "--features", "charge", "--message-format=json"], fixture, env)
    charge_only_sources = compiled_shared_sources(charge_only.stdout, workspace)
    assert charge_file in charge_only_sources
    assert "generated/async-stripe-shared/src/balance_transaction.rs" not in charge_only_sources, "charge alone compiled the balance transaction model"
    run(work, "model-only", cargo + ["test", "--offline", "--no-default-features", "--features", "balance_transaction_source"], fixture, env)
    run(work, "typed-model-only", cargo + ["test", "--offline", "--no-default-features", "--features", "application"], fixture, env)
    run(work, "shared-tag-models", cargo + ["test", "--offline", "--no-default-features", "--features", "application,deleted_customer,customer,deserialize"], fixture, env)
    run(work, "combined-groups", cargo + ["test", "--offline", "--no-default-features", "--features", "charge,invoice"], fixture, env)
    run(work, "all-features", cargo + ["test", "--offline", "--all-features"], fixture, env)
    run(work, "http-client", cargo + ["check", "--offline", "--features", "stripe/default-tls"], fixture, env)
    bare = run(work, "bare-serialize", cargo + ["check", "--offline", "--manifest-path", str(workspace / "async-stripe/Cargo.toml"), "--no-default-features", "--features", "serialize", "--message-format=json"], workspace, env)
    for line in bare.stdout.splitlines():
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        assert message.get("target", {}).get("name") not in {"stripe_core", "stripe_billing"}, "serialization forwarding activated an optional API crate"

    webhook_base = run(work, "webhook-base", cargo + ["test", "--offline", "--message-format=json"], webhook_fixture, env)
    webhook_base_sources = compiled_shared_sources(webhook_base.stdout, workspace)
    assert charge_file not in webhook_base_sources, "base webhook compiled Charge"
    webhook_charge = run(work, "webhook-charge", cargo + ["test", "--offline", "--features", "charge,deserialize,serialize", "--message-format=json"], webhook_fixture, env)
    webhook_charge_sources = compiled_shared_sources(webhook_charge.stdout, workspace)
    assert charge_file in webhook_charge_sources
    for line in webhook_charge.stdout.splitlines():
        try:
            message = json.loads(line)
        except json.JSONDecodeError:
            continue
        assert message.get("target", {}).get("name") != "stripe_core", "webhook model feature activated HTTP request package"
    run(work, "webhook-disabled-serde", cargo + ["test", "--offline", "--features", "deserialize,serialize"], webhook_fixture, env)
    run(work, "webhook-all-features", cargo + ["test", "--offline", "--all-features"], webhook_fixture, env)
    if args.full_v1:
        run(work, "webhook-unit-tests", cargo + ["test", "--offline", "-p", "async-stripe-webhook", "--all-features"], workspace, env)
        run(work, "facade-full", cargo + ["check", "--offline", "-p", "async-stripe", "--no-default-features", "--features", "full,serialize,deserialize"], workspace, env)
        run(work, "exhaustive-fixtures", cargo + ["test", "--offline", "-p", "async-stripe-tests", "--test", "it", "deserialization_fixture::fixtures", "--", "--nocapture"], workspace, env)
        run(work, "parser", cargo + ["test", "--offline", "-p", "async-stripe-parser"], workspace, env)
    assert source_snapshot() == before, "verification modified product source"
    summary = {"full_v1": args.full_v1, "base_shared_source_files": len(base_sources), "charge_shared_source_files": len(charge_sources), "charge_only_shared_source_files": len(charge_only_sources), "card_decoder_shared_source_files": len(card_sources), "ideal_decoder_shared_source_files": len(ideal_sources), "webhook_base_shared_source_files": len(webhook_base_sources), "webhook_charge_shared_source_files": len(webhook_charge_sources), "charge_absent_from_base": True, "detail_decoders_compile_independently": True, "webhook_charge_does_not_activate_core": True, "source_unchanged": True}
    (work / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    for path in [workspace, output]:
        shutil.rmtree(path)
    for explicit, path in [(args.target_dir, target), (args.generator_target_dir, generator_target)]:
        if explicit is None and path.exists():
            shutil.rmtree(path)
    print(f"Verified opaque model groups. Logs and results: {work}", flush=True)


if __name__ == "__main__":
    main()
