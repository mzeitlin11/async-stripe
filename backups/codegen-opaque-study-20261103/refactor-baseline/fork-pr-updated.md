Codegen currently couples inferred objects, crate placement, declaration traversal, and rendered paths. This separates those concerns so crate-organization experiments can be analyzed while preserving the generated bindings.

Based on the [output-handling draft (#4)](https://github.com/mzeitlin11/async-stripe/pull/4), which includes the CI verification and CLI/spec-cache changes. This comparison shows the remaining refactor.

## Changes

- Give components, operations, and generated declarations explicit identities; keep names and output locations separate.
- Record dependency facts with their usage and reference kind, then infer and freeze crate placement from a selected policy.
- Lower declarations and references into an item catalog used by rendering and lifetime/`Copy`/`Eq` analysis.
- Route canonical declarations through their selected item placements with one declaration writer. IDs, request declarations, nested types, and webhook unions follow their own placement; module declarations, exports, references, and webhook feature gates follow those decisions.
- Derive generated Cargo dependencies and feature forwarding from lowered Rust declarations, request signatures and constructor fields, associated ID types, and public facade reexports. Resolve references through item placement; validate local visibility and reject crate cycles before emission. Report the actual Rust crate graph separately from the spec projection used for placement inference.
- Use one represented-module dependency builder for crate dependencies and deterministic module/SCC reports. Include isolated modules and count canonical declarations, struct fields, and explicit enum variants; keep feature activation separate.
- Separate candidate matching, shared naming, and usage compatibility. Retain sharing rules and original occurrence locations/usages; expose them in `items.csv` and `item-occurrences.csv`.
- Replace `graph` with `analyze`, producing deterministic CSV reports and DOT graphs. `generate --report-dir` writes reports alongside generation and replaces `--emit-warnings`; the update workflow reads the warning CSV.
- Materialize bindings and reports from file sets, preserving the prerequisite's staging guards, formatter diagnostics, and installation preflight.
- Verify normal generation in an isolated SDK workspace, retaining analysis, logs, and actual compiler source lists without installing product files.

## Current limits

Sharing remains component-local and preserves the existing decisions before item allocation. Fieldless enum matching is independent of occurrence names and documentation; the model-field rule can combine model and request usages. Composite matching still includes field documentation and nested metadata, and other candidates require identical usage. Broader structural matching and usage compatibility remain follow-up work.

The default placement policy keeps IDs with their models and nested declarations with their parents. Emission now supports independently placed public declarations, with visibility, local-reference, duplicate-name/module, and exactly-once checks. No new production placement policy is selected. The handwritten webhook manifest still requires the corresponding dependency and feature when a payload moves. `analyze --without-expandables` changes placement analysis only; it does not implement a different generated API representation. Analysis reports the remaining Rust dependencies and cycles for that hypothetical placement.

## Validation

- Rust 1.89: 64 generator tests, strict Clippy across all targets, release build, and nightly formatting pass.
- Full normal generation is byte-identical to the preceding refactor baseline across all 1,758 artifacts on pinned spec `v2494`.
- The isolated checker compiles all 13 generated crates and webhooks with all features, runs standalone webhook tests, generated fixtures, existing deserialization tests, and parser tests, and verifies that source files remain unchanged.
- Repeated `analyze --graphs all` runs produce 20 identical files, including the ownership manifest. Selecting no graphs removes stale owned graph files and preserves unrelated review notes.
- The existing relocation, facade export, constructor-field visibility, associated-ID, and crate-cycle regression tests continue to pass. The module reports add coverage for cycle projection, canonical shape accounting, and deterministic ordering.

The branch also backports the two-line Reserve serde feature-forwarding fix from upstream commit `a2a02130`, required for standalone webhook verification on this older base. The opaque-model representation and its model-feature/facade planner remain a separate experiment.
