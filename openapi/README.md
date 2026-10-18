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

Commands use the OpenAPI release pinned in `version.json`. The managed `spec3.sdk.json` cache is downloaded automatically when it is missing, malformed, or tagged with another release. Generation also synchronizes `fixtures.json`. `generate --update-spec latest` or `generate --update-spec v2343` advances the pin after both inputs are available. Use `cargo run -- --help` or a subcommand's `--help` for the complete interface.

CI verifies output compatibility by running `generate` and requiring the entire Git worktree to remain clean. This exercises the same input synchronization, formatting, and installation path used to update generated code.

## Pipeline

Code generation has explicit stages:

1. **Inference** parses schemas and operations, allocates `ComponentId` and `OperationId` identities, resolves crate configuration, and records an immutable dependency fact table.
2. **Retention** removes unreachable components without changing catalog identity.
3. **Component layout** infers model and request crate placement in a mutable `ComponentLayoutDraft`, validates crate-graph invariants, then freezes a `ComponentLayout`.
4. **Model rewrites** apply validated overrides and structural deduplication.
5. **Rust-model lowering** allocates a post-dedup `ItemId` for every declaration, replaces nested declarations and named paths with item references, and computes lifetime/`Copy`/`Eq` properties over the resulting canonical graph. Each item is then assigned an explicit emission site and either local or crate-root addressability under the frozen component layout.
6. **Rendering** combines an emission-site-scoped `RenderContext` with canonical items, their explicit placements, and semantic `RustType`s, then writes Rust syntax directly. Rendering does not infer placement from item provenance. The current path policy deliberately preserves absolute generated-crate aliases; path shortening is a separate output-changing decision.
7. **Materialization** writes generated bindings to the staging tree, runs rustfmt directly to a checked fixed point, and installs them unless `--no-install` was requested. Analysis stops after model construction and writes only its report bundle.

Keeping identity, dependency facts, layout, and rendering separate is deliberate. A future layout or `Expandable` experiment should be expressible as a different projection or policy rather than by rewriting component names or rendered Rust paths early in the pipeline.

Item identities describe declarations produced by the existing component-local deduplication pass. They do not define structural equivalence or decide whether different usage contexts can share a declaration. Those decisions still precede item allocation. Item placement also preserves existing emission groups: ID newtypes follow their model components, and nested declarations stay with their parents. Context-aware deduplication and independent item placement remain follow-up work.

Operation declarations and their generated request specifications are owned together by the operation catalog and keyed by `OperationId`; component model objects contain only model data. This keeps request traversal and lookup independent of where model types are eventually placed.

Inference also carries an explicit borrowing policy. Borrowed strings, slices, maps, and named items all use the same `RustType::Reference` representation; borrowing is not encoded independently in each type variant. The production pipeline currently selects owned request fields.

Types supplied by dependencies are likewise symbolic in the model; only `RenderContext` maps them to source paths such as `stripe_types::Timestamp`. Constructor conversions and field-level serde behavior are selected by small render plans before their syntax is formatted.

## Analysis output

`analyze` writes deterministic, diffable files:

- `summary.md`: totals for components, operations, requests, webhooks, objects, fields, variants, canonical item properties, dependency forms, and each crate.
- `components.csv`: per-component placement, shape, request, and dependency counts.
- `items.csv`: one row per post-dedup Rust declaration, with its run-local `ItemId`, descriptive origin, chosen identifier, kind, usage, and derived lifetime/`Copy`/`Eq` properties.
- `dependencies.csv`: one row per parsed dependency fact, including its source, use, representation, field or variant path, target, and whether each endpoint survived retention.
- `crate-inference-warnings.csv`: every placement that required fallback resolution and its candidates.
- `graphs/crate-dependencies.dot`: the generated crate dependency graph.
- `graphs/component-dependencies.dot`: all component dependencies under the selected expandable policy.
- `graphs/component-model-dependencies.dot`: model-field dependencies under the selected expandable policy.

`--graphs all` additionally emits explicit `with-expandables` and `without-expandables` versions of both component graphs. The latter is an analysis counterfactual: it asks what placement pressure remains if expandable references become opaque. It does not alter generated API types.

Dependency facts distinguish direct fields, container references, expandable references, union members, and references to ID newtypes. They also distinguish model fields, request parameters, request returns, and webhook payloads. ID-only facts are visible in reports but remain opaque to current layout; `--graphs all` emits a `with-id-only` counterfactual. Graphs are projections of this fact table; the fact table itself records what was inferred from the specification and is not rewritten by layout.

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
