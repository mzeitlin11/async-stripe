# Verify generated bindings in isolation

Run from the repository root with Python 3.11 or newer:

```sh
python3 openapi/tests/verify_generated_workspace.py \
  --work-dir /tmp/async-stripe-generated-verification \
  --generator-target-dir /tmp/async-stripe-generator-target \
  --target-dir /tmp/async-stripe-sdk-target
```

The selected Rust toolchain defaults to `1.89.0` for both generation and SDK
validation. Nightly with rustfmt must also be installed. Cargo commands use
`--offline`, so dependencies must be cached; generation uses the pinned spec and
its matching fixtures. The script does not update the pin or install bindings.

The verifier stages runtime sources and generated artifacts outside the checkout,
then formats the isolated workspace and checks every crate listed in
`openapi/gen_crates.toml` together with the webhook crate, enabling all features.
It first checks the runtime with default features disabled and a narrow v1
Customer request configuration. It also runs webhook unit tests, the generated deserialization fixtures, existing
deserialization tests, and parser tests. These checks do not require stripe-mock.

Commands and their output are saved as individual logs. Analysis reports and
`summary.json` remain in the work directory. The all-features check also retains
`compiled-generated-crate-sources.json`, measured from rustc dep-info, with total
and per-package file counts in the summary. These counts cover source files under
`generated/async-stripe-*`, including module indexes and request files; they are
not model counts. Runtime packages, including the webhook package's generated
module, are checked but excluded from this source count.

On success, generated staging and
implicit build caches are removed. Explicit build caches are retained; add
`--keep-workspace` to retain the formatted SDK sources too. Failures retain the
workspace and caches for inspection and print the failing log's tail.
An implicit cache containing an explicitly retained cache is retained as well.

Repository bytes are compared before and after verification, including tracked
files and untracked files outside Git ignore rules. Ignored caches are excluded.
Concurrent source edits are reported as changes, so run against a stable tree.
The work directory must be empty or previously created by this verifier.

## Source measurement smoke check

This regression case checks absolute and workspace-relative rustc dep-info,
including escaped spaces, without running Cargo:

```sh
python3 - <<'PY'
import json
from pathlib import Path
import runpy
import tempfile

measure = runpy.run_path('openapi/tests/verify_generated_workspace.py')['compiled_generated_sources']
with tempfile.TemporaryDirectory(prefix='generated source smoke ') as directory:
    workspace = Path(directory)
    module = workspace / 'generated/async-stripe-shared/src/mod.rs'
    payload = module.parent / 'model with spaces.rs'
    module.parent.mkdir(parents=True)
    for source in [module, payload, module.parent / 'unused.rs']:
        source.write_text('')
    artifact = workspace / 'libstripe_shared-smoke.rmeta'
    dep_info = workspace / 'stripe_shared-smoke.d'
    log = workspace / 'check.log'
    log.write_text('command header\n' + json.dumps({
        'reason': 'compiler-artifact',
        'target': {'src_path': str(module)},
        'filenames': [str(artifact)],
    }) + '\n')
    expected = sorted(str(source.relative_to(workspace)) for source in [module, payload])
    for absolute in [False, True]:
        paths = [str(source if absolute else source.relative_to(workspace))
                 for source in [module, payload]]
        escaped = [path.replace(' ', '\\ ') for path in paths]
        dep_info.write_text(f'{artifact}: ' + ' '.join(escaped) + '\n')
        assert measure(log, workspace) == expected, absolute
print('Source measurement smoke check passed')
PY
```

## Neutral refactor comparison

The structural refactor is compared with `bc737f7c`, the output-handling stage
before canonical item allocation was introduced. Generate into separate staging
directories from that revision and the candidate, using the same cached spec,
fixtures, library version, Rust toolchain, and nightly formatter. Do not update
input pins during the comparison. The recorded input and toolchain hashes are in
[neutral_refactor_validation.json](neutral_refactor_validation.json).

From each checkout's `openapi` directory, use the appropriate output path:

```sh
CARGO_TARGET_DIR=/tmp/stripe-parity-generator cargo +1.89.0 run --locked --offline --release -- \
  generate --no-install --version 1.0.0-rc.9 --out /tmp/stripe-before-output
# Run the same command in the candidate, with --out /tmp/stripe-after-output.
```

Then, from the candidate repository root:

```sh
python3 openapi/tests/compare_generated.py /tmp/stripe-before-output /tmp/stripe-after-output
```

The comparator includes every file, including manifests, module exports, and the
staging marker. It exits unsuccessfully for added, removed, or changed files.
The recorded run matches all 1,757 SDK artifacts and the marker. Both original
`graph` DOT files also match their predecessor byte for byte. The CLI accepts
`--emit-warnings` and retains its Markdown warning output.

The construction regressions cover representation-dependent properties and
request types, validation after a transform, and a request field changed from a
model reference to an independently placed ID. That last probe removes the
model-crate dependency while retaining the inference-side relationship. This is
the ordering needed by the expandable experiment. The feature can transform an
`ItemCatalogDraft`, then call `RustModel::from_draft`; it no longer needs to
recompute properties and canonical requests on a completed generator.

Configuration tests resolve two layouts in one process. A separate full-input
probe constructs two live `CodeGen` instances with different package names, then
renders the first again. Its complete output remains identical. This exercises
the explicit configuration input needed by the v2 layout experiment.

The [evidence archive](neutral_refactor_evidence.tar.gz) retains the exact probe
source, input hashes, comparison results, generator test/Clippy logs, SDK logs,
and compiled-source inventory. These focused probes do not claim that the full
v2 or opaque branches have been rebased. The neutral default continues to select
the legacy representation and placement policy.

## Review regressions

Run the parity comparator's positive and negative command-line cases with:

```sh
python3 -B -m unittest discover -s openapi/tests -p test_compare_generated.py
```

An input tree must contain files; directories alone cannot establish parity.
The generator tests also cover enum metadata refresh after representation
changes, Object identity compatibility, and the distinction between represented
dependencies and legacy package-facade requirements. Optional IDs remain valid
for ordinary untagged structs.

The review pass also exercises nonempty legacy warning output by removing the
explicit Address assignment from a task-owned copy of `gen_crates.toml`, then
comparing `generate --emit-warnings` on the predecessor and candidate. Production
configuration and input pins stay unchanged. Results are retained separately in
[neutral_refactor_review.json](neutral_refactor_review.json) and its
[evidence archive](neutral_refactor_review_evidence.tar.gz); the original
validation record remains historical evidence for the first implementation.

## Second review

The second review adds construction checks for the enum forms currently
supported by rendering and derives union discriminator tags from represented
structs. It also exercises canonical fixture names after rename/relocation and
validated alternate placement tables. Artifact-write tests verify that rejecting
a duplicate leaves the original bytes intact; documentation mapping tests use
explicit files or an empty mapping without network access.

Two inherited derived-trait bugs are corrected in a separate commit: `Box<T>`
cannot be `Copy`, and borrowing a map does not make floating-point values `Eq`.
The conservative policy for owned-map equality remains unchanged. These fixes
are distinguished from the neutral construction changes in
[neutral_refactor_review2.json](neutral_refactor_review2.json), with fresh logs
in the [second review evidence archive](neutral_refactor_review2_evidence.tar.gz).

The simplification passes following `2bcc0189` remove the single-caller pipeline
wrapper, forwarding options, field render plans, transient type-name records,
and duplicate display/reporting adapters. Inferred request parameters carry
`RustType` directly; property analysis reads its existing property slice rather
than accepting an interchangeable lookup callback. Reports retain only model
module counts alongside the complete dependency set, and crate membership is
iterated without an intermediate set. Constructor borrowing is checked through
rendered signatures. Fresh validation passes 86 library tests, one CLI test,
three comparator tests, strict Clippy, and the complete isolated SDK verifier.
All 1,758 staging files still match `bc737f7c` with the input and toolchain hashes
recorded above. Default and counterfactual analysis bundles, CLI help, legacy
graphs, and nonempty warning output also match the preceding candidate byte for
byte. Historical evidence records remain attached to the revisions they tested.
