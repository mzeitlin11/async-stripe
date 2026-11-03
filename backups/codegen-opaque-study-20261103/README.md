# Codegen refactor backup — 2026-11-03

The implementation is on [`expt/codegen-opaque-slice`](https://github.com/mzeitlin11/async-stripe/tree/expt/codegen-opaque-slice), saved at `951e1c207e380839686474bc039f12da6e531b7a`. Its commit history includes the pipeline refactor, value-backed expandable and polymorphic responses, independent model availability, consumer-based placement, facade extension traits, and removal of the six payment-option-bag exceptions.

This backup branch preserves two additional things:

- The parent commit `66768145965964acae27e71cc5ca1097b1936375` captures the original `expt/id-typed-expandable` worktree's 11 uncommitted files exactly, on top of `785bc2ef47fc618d1e48bed9c8607e0995cf6e8e`. The original worktree was not modified. `original-worktree.patch` provides the same changes separately.
- `artifacts.tar.gz` contains every retained non-build artifact from the experiment directory: historical and current analysis, validation logs, timing reports and HTML, comparison scripts, the Rust shape-analysis parser, and early prototype/simulation sources. `manifest.json` records the size and SHA-256 of every archived file; the archive contents were checked against it before committing. Key reports are also available uncompressed under `reports/`.

Build caches and generated SDK workspaces are omitted. Generate and validate the current SDK from the implementation branch:

```console
cd openapi
python3 tests/verify_opaque_groups.py --full-v1 --consumer-layout --keep-workspace
```

The reports retain machine-specific paths and reflect several historical designs. Use `typed-options-impact/summary.json` for the latest physical crate counts and `typed-options-impact/compile-comparison.json` for the latest paired timing comparison. Do not combine timings from separate experiments as a controlled comparison.

Current behavior and conclusions:

- Expandable responses retain raw JSON plus typed identity. Polymorphic responses defer branch payloads while preserving typed common fields.
- Optional named decoders live in one facade extension trait per record, without creating dependencies between model crates.
- Ordinary payment-method option bags remain typed. The six special-case opacity selectors were removed because independently present settings do not imply polymorphism.
- Removing those exceptions increased Shared from 108 to 166 model modules, and from 541 to 655 struct fields. A PaymentIntent feature closure increased from 23 to 95 model modules.
- In two alternating local checks with four jobs and cached external dependencies, median full SDK check time changed from 12.49s to 13.86s, and Shared from 1.60s to 2.19s. These are exploratory measurements.
- The latest revision passed 117 generator tests, strict generator Clippy, formatting, and full and narrow generated-consumer verification. Normal generation retains its existing concrete representation; the opaque candidate remains experimental and staged.

The original refactor review branches already on this fork remain available: `review/codegen-ci-verification`, `review/codegen-cli-spec-cache`, `review/codegen-output-handling`, and `review/codegen-pipeline`.


## Live/deleted enum restoration

The implementation branch now includes `176c3f588c4e1553cad274c90f2f1a4a28e41772`, restoring all 11 live/deleted response pairs to concrete matchable enums. General polymorphic responses and expandable storage stay deferred. Crate/model dependency graphs, all model feature closures, and Shared's model placement are unchanged. The original enum decoders are retained, including their existing serde/miniserde distinction for overlapping shapes.

The [live/deleted restoration notes](live-deleted-restoration/README.md) and [summary](live-deleted-restoration/summary.json) supersede the original archive's discussion of opaque live/deleted pairs. That directory also contains the comparison script, normal-generation artifact hashes, and a verified archive of the new full/narrow validation logs and reports. All 117 generator tests, strict Clippy, full/narrow consumers, fixtures, and parser checks passed; normal generation remains byte-identical across 1,758 artifacts.
