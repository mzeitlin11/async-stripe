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
4. **Model rewrites** apply validated overrides and component-local deduplication. Candidate matching, shared public naming, and usage compatibility are separate decisions. Each shared declaration retains the selected rule and the original locations and usages that contributed to it.
5. **Rust-model lowering** allocates a post-dedup `ItemId` for every declaration, replaces nested declarations and named paths with item references, and computes lifetime/`Copy`/`Eq` properties over the resulting canonical graph. Each item is then assigned an explicit emission site and either local or crate-root addressability under the frozen component layout. Crate dependencies follow these placements, including declarations, associated ID types, request signatures and constructor fields, and public facade reexports. Local references must stay within their emission site.
6. **Rendering** validates that the lowered Rust crate dependencies are acyclic and uses them for generated Cargo dependencies and feature forwarding. One `ItemEmitter` routes every canonical declaration to its selected file and writes the corresponding module declarations and exports. The existing traversal preserves source order; IDs and nested declarations are emitted independently using their own placements. An emission-site-scoped `RenderContext` resolves references and writes Rust syntax directly. The current path policy deliberately preserves absolute generated-crate aliases; path shortening is a separate output-changing decision.
7. **Materialization** writes generated bindings to the staging tree, runs rustfmt directly to a checked fixed point, and installs them unless `--no-install` was requested. Analysis stops after model construction and writes only its report bundle.

Keeping identity, dependency facts, layout, and rendering separate is deliberate. A future layout or `Expandable` experiment should be expressible as a different projection or policy rather than by rewriting component names or rendered Rust paths early in the pipeline.

Item identities describe declarations produced by component-local deduplication; they are allocated after sharing decisions. The current sharing policy preserves existing output. Fieldless enums are matched by their variants, independently of occurrence names and documentation. The model-field rule can combine model, request-parameter, and return usages: these enums have no field-level serialization or borrowing behavior to reconcile. Other candidates require identical usage. Shared names still use the existing field-name, schema-title, or description cues; equal representation alone does not imply shared public identity.

Composite candidate matching remains conservative: field documentation and nested object metadata still participate in equality. Broader structural matching and more flexible usage compatibility need concrete cases and explicit rules for the resulting public API and documentation.

The default placement policy keeps ID newtypes with their models and nested declarations with their parents. Emission itself no longer requires that grouping: a public declaration can occupy a standalone module in a selected generated crate, with local children sharing that site or independently exported children elsewhere. Placement does not change Rust visibility. Existing component model/ID facades and shared override exports follow the selected addresses. Webhook references and feature gates follow item addresses too; its handwritten manifest still needs the corresponding dependency and feature. Selecting a new production placement policy remains follow-up work.

Operation declarations and their generated request specifications are owned together by the operation catalog and keyed by `OperationId`; component model objects contain only model data. This keeps request traversal and lookup independent of where model types are eventually placed.

Inference also carries an explicit borrowing policy. Borrowed strings, slices, maps, and named items all use the same `RustType::Reference` representation; borrowing is not encoded independently in each type variant. The production pipeline currently selects owned request fields.

Types supplied by dependencies are likewise symbolic in the model; only `RenderContext` maps them to source paths such as `stripe_types::Timestamp`. Constructor conversions and field-level serde behavior are selected by small render plans before their syntax is formatted.

## Analysis output

`analyze` writes deterministic, diffable files:

- `summary.md`: totals for components, operations, requests, webhooks, objects, fields, variants, canonical item properties, dependency forms, and each crate.
- `components.csv`: per-component placement, shape, request, and dependency counts.
- `items.csv`: one row per post-dedup Rust declaration, with its run-local `ItemId`, canonical declaration origin, chosen identifier, kind, usage, derived lifetime/`Copy`/`Eq` properties, the sharing rule when selected directly by deduplication, and the actual emission path and public crate (blank for local declarations).
- `item-occurrences.csv`: original object locations and usages contributing to each declaration, including nested declarations inside shared parents. These are distinct from canonical declaration origins used by the default placement policy. Synthetic ID newtypes have no inferred object occurrences. Locations describe the inferred model after overrides, rather than raw OpenAPI nodes.
- `dependencies.csv`: one row per parsed dependency fact, including its source, use, representation, field or variant path, target, and whether each endpoint survived retention.
- `crate-inference-warnings.csv`: every placement that required fallback resolution and its candidates.
- `graphs/crate-dependencies.dot`: crate dependencies required by the lowered Rust model, including mandatory domain-to-shared facade dependencies. This graph drives generated manifests. The handwritten webhook crate retains its own manifest.
- `graphs/inferred-crate-dependencies.dot`: the spec dependency projection used to validate placement inference, before overrides and Rust-model lowering.
- `graphs/component-dependencies.dot`: all component dependencies under the selected expandable policy.
- `graphs/component-model-dependencies.dot`: model-field dependencies under the selected expandable policy.

`--graphs all` additionally emits explicit `with-expandables` and `without-expandables` versions of both component graphs. `analyze --without-expandables` also computes placement under that counterfactual: it asks what placement pressure remains if expandable references become opaque. It does not alter generated API types. The lowered Rust crate graph still includes those references and may therefore contain cycles; the summary reports this, and generation rejects cyclic crate dependencies.

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

## Experimental opaque model groups

The v1 model-group experiment exercises representation, dependency closure, module availability, and facade exports together. Normal `generate` retains the concrete representation. Model discriminator metadata and live/deleted value conversion are available in both representations. Run the isolated consumer check from this directory:

```console
python3 tests/verify_opaque_groups.py
```

The check builds a private workspace and tests real generated retrieval through a mock client. It verifies that the base group preserves source IDs and unknown expanded objects without compiling `Charge`, that enabling `charge` adds typed decoding, and that `charge` also works independently. Model-only Application, live/deleted Customer, and combined charge/invoice groups exercise facade exports and decoding boundaries. Webhook consumers check disabled/raw and enabled/typed payloads on both decoding paths, including that webhook model features do not activate HTTP request packages. Logs, compiled-source lists, and analysis reports are retained; disposable generated workspaces and build caches are removed after success. Explicit `--target-dir` and `--generator-target-dir` caches are retained for reuse.

For the complete supported v1 surface, including the existing generated fixture suite and standalone parser tests:

```console
python3 tests/verify_opaque_groups.py --full-v1
```

Full v1 generation selects every resource with a wire object tag, named polymorphic resource family, and component with HTTP operations. Supporting schemas follow their actual dependencies rather than receiving an individual public feature. Fixture validation deliberately enables all selected groups and uses actual model addresses. The staged test manifest declares its representation-specific test feature; normal tests keep their concrete behavior, including under `--all-features`.

For inspecting the unformatted output and analysis directly:

```console
cargo run --example opaque_model_groups -- /tmp/stripe-opaque-output balance_transaction charge
# Or generate every supported v1 resource/endpoint group:
cargo run --example opaque_model_groups -- /tmp/stripe-opaque-output --all
```

`analysis/model-availability.csv` records each canonical item's emission path and availability (`always`, `disabled`, or its enabling groups). Availability closes actual represented dependencies at source-module granularity: sibling declarations in the same file compile together. The crate dependency and availability passes share their definition of type consumers.

`model-module-dependencies.csv` and `graphs/model-module-dependencies.dot` expose that same dependency graph after representation and placement. `model-module-sccs.csv` identifies the remaining model cycles; `model-group-closures.csv` separates always-on support from conditional group closures. `model-colocations.csv` records inherent-accessor constraints that require a common crate even without a dependency cycle. These provide evidence for choosing crate boundaries without changing the current crate map. Overlapping group counts must not be summed.

Representation changes occur after sharing and canonical allocation. `ExpandableValue<I>` stores an ID or a complete JSON object without naming its concrete model. Named polymorphic families use distinct opaque wrappers; their typed conversion checks the wire discriminator. IDs and component-level opaque families are lightweight unconditional support. Error scalars remain typed, while resource objects embedded as diagnostic context become `ObjectValue`. The original semantic relationships and occurrence provenance are retained.

The JSON tree is parsed eagerly; concrete model construction is deferred. A consuming decode such as `source.into_expanded_as::<stripe::Charge>()` distinguishes an ID-only value from invalid expanded data. Enabling a model does not change base parsing or discard unknown payloads.

The `PaymentMethodDetails` prototype applies the same separation to an ID-less record. `RecordValue` validates its string `type` and retains every field; generated methods such as `details.card()?` borrow and decode only that field. The `payment_method_details_card` feature enables that decoder, independently of saved Card resources or Charge requests. Missing/null fields return `None`; malformed present fields fail on access. Field access does not impose a stricter relationship with `type` than the existing schema. Repeated calls decode again without caching.

Each accessor is emitted beside its payload model, so its module has an ordinary dependency on the raw owner. Charge depends only on the raw record. Decoder groups are added when this record is part of the selected surface; full generation includes them all. Inherent methods require owner and payload to share a crate, which placement validation checks explicitly. This prototype keeps both in their existing backing crate rather than adding conditional dependency machinery or a decoder registry.

The wire tag is not a unique schema identity: live and deleted models can share it. Deferred conversion follows the existing miniserde model decoder's structural acceptance. A live/deleted union follows its existing `deleted` flag routing. Serde's existing untagged union decoding can choose a different branch for payloads that structurally satisfy both models; this experiment does not impose stricter validation on either individual model.

Selected models and endpoint builders are available at the root (`stripe::Charge`, `stripe::CreateCharge`). Request helpers live under their resource module (`stripe::charge::OptionalFieldsAddress`), so different groups can use the same helper name. Common runtime types such as `stripe::Currency` and `stripe::ObjectValue` are available without another dependency.
Available model modules and their public helpers are reexported according to actual availability, including `stripe::PaymentMethodDetails` and the independently enabled `stripe::PaymentMethodDetailsCard`.

Webhooks gate typed payloads using the same model availability plan. Unknown and disabled payloads preserve their complete data in `EventObject::Unknown` on both serde and miniserde paths; malformed enabled models still fail. The standalone parser enables all model groups and rejects unknown event variants. Opaque expandable/polymorphic subtrees remain opaque there too, so this is not recursive validation of every retained subtree.

This is an experimental v1 SDK representation, not a completed crate organization policy. It keeps the current crate map and gates heavy model modules within it. Repository installation remains rejected while the normal installer lacks ownership of the additional facade, webhook, and test manifests. Those manifests record generated model groups so regeneration can remove stale flags/dependencies while preserving runtime settings. Group selection remains a generator development API/example, with no new normal-generation CLI flag.
