# OpenAPI code generation

This crate generates async-stripe from Stripe's [OpenAPI specification](https://github.com/stripe/openapi). Run commands from this directory. Use Rust 1.89 or newer; generated Rust is formatted with nightly rustfmt.

## Common commands

Generate, format, and install the result into the repository:

```console
cargo run --release -- generate
```

Generate without installing:

```console
cargo run --release -- generate --no-install
```

Generated files are staged in `out` by default. Use `generate --out <DIR>` to choose another directory. Custom directories must be empty or belong to an earlier generator run; stale generated files are cleared before regeneration. Formatting must succeed before installation.

Advance to the latest Stripe OpenAPI release and regenerate:

```console
cargo run --release -- generate --update-spec latest
```

An exact release such as `v2343` can be selected instead of `latest`.

Write the default analysis bundle to `out/analysis`:

```console
cargo run --release -- analyze
```

Write every graph projection, including the expandable counterfactuals:

```console
cargo run --release -- analyze --graphs all
```

The original `graph` command still writes `out/graphs/components_graph.txt` and
`out/graphs/crate_graph.txt`. `generate --emit-warnings` still writes
`<out>/crate_warnings.md` when inference warnings exist. Analysis reports are additive.

Commands use the OpenAPI release pinned in `version.json`. The managed `spec3.sdk.json` cache is downloaded automatically when it is missing, malformed, or tagged with another release. Generation also synchronizes `fixtures.json`. `generate --update-spec latest` or `generate --update-spec v2343` advances the pin after both inputs are available. Use `cargo run -- --help` or a subcommand's `--help` for the complete interface.

CI verifies output compatibility by running `generate` and requiring the entire Git worktree to remain clean. This exercises the same input synchronization, formatting, and installation path used to update generated code.

## Pipeline

Code generation has explicit stages:

1. **Inference** parses schemas and operations, allocates `ComponentId` and `OperationId` identities, resolves crate configuration, and records an immutable dependency fact table.
2. **Retention** removes unreachable components without changing catalog identity.
3. **Request ownership** infers service crates in a mutable `ComponentLayoutDraft`. Ownership retains the historical inference order and is separate from final model placement.
4. **Model rewrites** apply validated overrides and component-local deduplication. Candidate matching, shared public naming, and usage compatibility are separate decisions. Each shared declaration retains the selected rule and the original locations and usages that contributed to it.
5. **Rust-model construction** allocates a post-dedup `ItemId` for every declaration and replaces nested declarations and named paths with item references in an `ItemCatalogDraft`. Representation changes belong on this draft. `RustModel::from_draft` consumes it, validates the represented declarations and Object identities, refreshes enum rendering metadata from represented declarations, computes lifetime/`Copy`/`Eq` properties, validates the supported enum rendering forms, and builds canonical requests together. Completed models provide read-only access.
6. **Final placement** selects the legacy Shared promotions using the immutable parsed dependency facts, preserving existing output, then freezes component and item placement. Another policy can inspect the finalized representation before choosing placement. Each item has an explicit emission site and either local or crate-root addressability. Crate dependencies follow represented declarations, associated IDs, request signatures and constructor fields. Public export decisions are shared with rendering. The represented dependency calculation adds no implicit Shared edges; default assembly supplies the legacy facade dependencies separately. Local references must stay within their emission site.
7. **Rendering** validates that the lowered Rust crate dependencies are acyclic and uses them for generated Cargo dependencies and feature forwarding. One `ItemEmitter` routes every canonical declaration to its selected file and writes the corresponding module declarations and exports. The existing traversal preserves source order; IDs and nested declarations are emitted independently using their own placements. An emission-site-scoped `RenderContext` resolves references and writes Rust syntax directly. The current path policy deliberately preserves absolute generated-crate aliases; path shortening is a separate output-changing decision.
8. **Materialization** writes generated bindings to the staging tree, runs rustfmt directly to a checked fixed point, and installs them unless `--no-install` was requested. Analysis stops after model construction and writes only its report bundle.

Alternative item-placement policies can pass a complete table to `ItemLayout::from_placements`, which validates it before returning a layout. Fixture references use canonical declaration names through the public component facades, preserving existing output even when a model is relocated.

Each `CodeGen` owns its `CrateConfig`. The CLI loads `gen_crates.toml`; library callers can pass another owned configuration to `CodeGen::with_config`. Crate handles are run-local and resolve names through their configuration. Multiple configurations can coexist without global initialization or changing the process working directory. `UrlFinder::load` accepts an explicit documentation mapping path; `UrlFinder::default()` supplies an empty mapping for generation without documentation links. The CLI keeps loading `doc_urls.json`.

Keeping identity, dependency facts, layout, and rendering separate is deliberate. A future layout or `Expandable` experiment should be expressible as a different projection or policy rather than by rewriting component names or rendered Rust paths early in the pipeline.

Item identities describe declarations produced by component-local deduplication; they are allocated after sharing decisions. The current sharing policy preserves existing output. Fieldless enums are matched by their variants, independently of occurrence names and documentation. The model-field rule can combine model, request-parameter, and return usages: these enums have no field-level serialization or borrowing behavior to reconcile. Other candidates require identical usage. Shared names still use the existing field-name, schema-title, or description cues; equal representation alone does not imply shared public identity.

Composite candidate matching remains conservative: field documentation and nested object metadata still participate in equality. Broader structural matching and more flexible usage compatibility need concrete cases and explicit rules for the resulting public API and documentation.

The default placement policy keeps ID newtypes with their models and nested declarations with their parents. Emission itself no longer requires that grouping: a public declaration can occupy a standalone module in a selected generated crate, with local children sharing that site or independently exported children elsewhere. Placement does not change Rust visibility. Existing component model/ID facades and shared override exports follow the selected addresses. Webhook references and feature gates follow item addresses too; its handwritten manifest still needs the corresponding dependency and feature. Selecting a new production placement policy remains follow-up work.

Operation declarations and their generated request specifications are owned together by the operation catalog and keyed by `OperationId`; component model objects contain only model data. This keeps request traversal and lookup independent of where model types are eventually placed.

Inference also carries an explicit borrowing policy. Borrowed strings, slices, maps, and named items all use the same `RustType::Reference` representation; borrowing is not encoded independently in each type variant. The production pipeline currently selects owned request fields.

Types supplied by dependencies are likewise symbolic in the model; rendering maps them to source paths such as `stripe_types::Timestamp`. Constructor conversions and field-level serde attributes are rendered directly from the represented types and usage.

## Analysis output

`analyze` writes deterministic, diffable files:

- `summary.md`: totals for components, operations, requests, webhooks, objects, fields, variants, canonical item properties, dependency forms, and each crate.
- `components.csv`: per-component placement, shape, request, and dependency counts.
- `items.csv`: one row per post-dedup Rust declaration, with its run-local `ItemId`, canonical declaration origin, chosen identifier, kind, usage, derived lifetime/`Copy`/`Eq` properties, the sharing rule when selected directly by deduplication, and the actual emission path and public crate (blank for local declarations).
- `item-occurrences.csv`: original object locations and usages contributing to each declaration, including nested declarations inside shared parents. These are distinct from canonical declaration origins used by the default placement policy. Synthetic ID newtypes have no inferred object occurrences. Locations describe the inferred model after overrides, rather than raw OpenAPI nodes.
- `dependencies.csv`: one row per parsed dependency fact, including its source, use, representation, field or variant path, target, and whether each endpoint survived retention.
- `crate-inference-warnings.csv`: every placement that required fallback resolution and its candidates.
- `model-module-dependencies.csv`: represented type uses between emitted source modules, including request signatures, constructor/setter fields, associated IDs, and references within one module.
- `model-module-sccs.csv`: sorted strongly connected components of model-bearing modules, with canonical declaration, struct-field, and enum-variant counts. Request modules, webhook modules, ID-only modules, and references within one module are excluded from this projection; isolated model modules are retained.
- `graphs/model-module-dependencies.dot`: that model-module projection, showing dependencies that constrain independently compiled model boundaries.
- `graphs/crate-dependencies.dot`: crate dependencies required by the lowered Rust model, including mandatory domain-to-shared facade dependencies. This graph drives generated manifests. The handwritten webhook crate retains its own manifest.
- `graphs/inferred-crate-dependencies.dot`: the spec dependency projection used to validate placement inference, before overrides and Rust-model lowering.
- `graphs/component-dependencies.dot`: all component dependencies under the selected expandable policy.
- `graphs/component-model-dependencies.dot`: model-field dependencies under the selected expandable policy.

`--graphs all` additionally emits explicit `with-expandables` and `without-expandables` versions of both component graphs. `analyze --without-expandables` also computes placement under that counterfactual: it asks what placement pressure remains if expandable references become opaque. It does not alter generated API types. The lowered Rust crate graph still includes those references and may therefore contain cycles; the summary reports this, and generation rejects cyclic crate dependencies.

The module reports follow the same represented type uses used to derive Cargo dependencies, after sharing and item placement. Their shape counts come from canonical declarations rather than repeated inferred occurrences. These reports describe generated type dependencies across the complete model; they do not evaluate feature activation or include handwritten code and module index files. Public facade reexports remain separate consumers in the crate graph. Actual compiler source closures require Cargo dependency information.

Dependency facts distinguish direct fields, container references, expandable references, union members, and references to ID newtypes. They also distinguish model fields, request parameters, request returns, and webhook payloads. ID-only facts are visible in reports but remain opaque to current layout; `--graphs all` emits a `with-id-only` counterfactual. Component graphs and the inferred crate graph project this fact table, which is not rewritten by layout. The lowered Rust crate graph instead follows actual item references after rewrites, including references to ID items regardless of the layout edge policy.

Analysis directories contain an ownership manifest. Subsequent runs remove only stale files listed in that manifest, preserving unrelated files in a custom report directory.

## Generated artifacts

The staging workspace contains:

- `crates/async-stripe-*/`: generated domain and shared crates.
- `async-stripe-webhook/mod.rs`: webhook event deserialization.
- `tests/mod.rs`: generated API-shape tests.
- `crate_info.md`: the component-to-crate table.

The facade re-exports model types from their generated placement, so consumers generally do not need to know whether a type is physically emitted in a domain crate or `async-stripe-shared`.

## Crate organization

Stripe model types contain many cross-resource references. Direct containment and polymorphic unions can form cycles, so the current layout promotes types as needed into `async-stripe-shared`; request definitions remain in domain crates and can be feature-gated per resource.

Expandable references are recorded separately because an ID-typed, opaque representation could remove those model edges. `analyze --graphs all` makes that counterfactual visible before generated API behavior changes. Similar lazy treatment of API errors and other polymorphic payloads can later be evaluated using the same dependency index.

`gen_crates.toml` declares generated crates and the explicit package or component assignments that seed inference. It is resolved once against the full component catalog. Duplicate crate names, package owners, component owners, missing `shared`, and unknown component names are errors; unknown names include nearby candidates. This file should state only intentional seed assignments—the remaining placement is derived from the dependency graph.

Rare model exceptions live in `src/overrides.rs`. Each override selects one generated operation and request field, declares its output name, and includes a reason. Selectors, fields, and output names are validated before any rewrite is applied.

## Isolated generated-workspace verification

Validate normal generation and the resulting SDK in a disposable workspace:

```console
python3 tests/verify_generated_workspace.py
```

This generates without installing, checks the generated crates and webhooks, and runs the existing fixture and parser tests. Logs and analysis are retained. Add `--keep-workspace` to inspect formatted output; explicit `--target-dir` and `--generator-target-dir` paths retain caller-owned build caches. The source worktree must remain unchanged. This checks the current concrete API without requiring any experimental representation or feature planner.
