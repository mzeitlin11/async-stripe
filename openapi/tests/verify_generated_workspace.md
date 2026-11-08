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

Commands and their output are saved as individual logs. `summary.json` remains in the work directory. The all-features check also retains
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

## Landing comparison

The landing base is upstream `e71ca4a8`, with its pinned v2578 schema, matching
fixtures, and library version 1.0.0-rc.10. Both sides use Rust 1.89.0 and the same
nightly formatter. Upstream's `--no-install` leaves output unformatted; format
its staging tree explicitly before comparing it with this generator's formatted
output. Never install either side while establishing parity.

Use `compare_generated.py` to compare the complete SDK artifact sets, including
manifests, exports, webhook code, and generated fixture tests. The refactor adds
one staging ownership marker, `.async-stripe-codegen-output`. Record that sole
metadata addition separately; do not omit other files or silently exclude hidden
files. Compare SDK artifacts in task-owned copies after removing only that marker
from the candidate copy. Input provenance and results are recorded in
`codegen_landing_validation.json`.

The existing tests exercise finalization after representation changes, canonical
requests, independently placed IDs and nested declarations, constructor
references, facade exports, local visibility, and dependency cycles. Upstream's
union naming, incomplete-object rejection, optional JSON defaults, and webhook
fixes remain part of the landing base.

Broader reports and the additive `analyze` command are a separate follow-up.
Historical v2494 validation records and archives remain on fork commit
`1fa2b9c0850d43b8f5e692b1f57afdb64fa1a624`; their original provenance is preserved.
