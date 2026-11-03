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

To also exercise the consumer-based placement candidate:

```console
python3 tests/verify_opaque_groups.py --full-v1 --consumer-layout
```

Add `--keep-workspace` to retain the formatted SDK sources for reviewing the actual generated crate contents. Build caches remain outside that source workspace.

This candidate keeps the existing package names, request ownership, and domain reexports. It assigns model modules to a single consuming domain when possible, with cross-domain support remaining shared. Consumer sets propagate through actual model dependencies and retained domain exports before placement. Optional decoder methods live in facade extension traits and do not couple model placement. It requires the complete supported surface so changing a consumer's feature selection does not change the inferred organization.

Full v1 generation selects every resource with a wire object tag or identity, plus components with HTTP operations. Supporting schemas follow their actual dependencies rather than receiving an individual public feature. Fixture validation deliberately enables all selected groups and uses actual model addresses. The staged test manifest declares its representation-specific test feature; normal tests keep their concrete behavior, including under `--all-features`.

For inspecting the unformatted output and analysis directly:

```console
cargo run --example opaque_model_groups -- /tmp/stripe-opaque-output balance_transaction charge
# Or generate every supported v1 resource/endpoint group:
cargo run --example opaque_model_groups -- /tmp/stripe-opaque-output --all
# Apply the placement candidate without installing it:
cargo run --example opaque_model_groups -- /tmp/stripe-opaque-output --all --consumer-layout
```

`analysis/model-availability.csv` records each canonical item's emission path and availability (`always`, `disabled`, or its enabling groups). Availability closes actual represented dependencies at source-module granularity: sibling declarations in the same file compile together. The crate dependency and availability passes share their definition of type consumers.

`model-module-dependencies.csv` and `graphs/model-module-dependencies.dot` expose that same dependency graph after representation and placement. `model-module-sccs.csv` identifies the remaining model cycles; `model-group-closures.csv` separates always-on support from conditional group closures. `record-decoders.csv` lists optional methods and their facade extension traits; these methods add no dependencies between model crates. Overlapping group counts must not be summed.

Representation changes occur after sharing and canonical allocation. `ExpandableValue<I>` stores an ID or a complete JSON object without naming its concrete model. Named polymorphic families use distinct opaque wrappers; their typed conversion checks the wire discriminator. IDs, component-level opaque families, and type-only raw record wrappers are lightweight unconditional support. Error scalars remain typed, while resource objects embedded as diagnostic context become `ObjectValue`. The original semantic relationships and occurrence provenance are retained.

`ObjectValue<I = SmolStr>` retains the selected ID type. Its default constructor requires a string ID. Optional identities such as `ExpandableValue<Option<InvoiceId>>` accept absent/null IDs when the existing model permits them, while rejecting malformed nonstring IDs. Expanded and ID-only states each retain one typed identity rather than duplicate caches.

Tagged expanded resources use `ExpandableValue<I>`. Untagged resources such as Radar Rule use `ExpandedValue<I>`: the same ID-or-preserved-object states, with structural typed decoding because the wire schema has no `object` discriminator. Strict tagged-object checks remain on `ObjectValue` and `ExpandableValue`. Every expandable relationship is deferred; its concrete target can be enabled separately.

The JSON tree is parsed eagerly; concrete model construction is deferred. A consuming decode such as `source.into_expanded_as::<stripe::Charge>()` distinguishes an ID-only value from invalid expanded data. Enabling a model does not change base parsing or discard unknown payloads.

Polymorphic response records preserve typed common fields and defer their branch payloads. Closed string discriminators select matching optional object fields, including singleton tags and nullable branches; open `type` records use a conservative shape rule. Source's open `flow` and tax transaction-attempt `status` axes have explicit selectors because their schemas list values only in prose. Payment-method option bags have ordinary typed fields; independently present settings are not inferred to be polymorphic. Request-input and mixed-input declarations remain eager and constructible.

Mixed records such as `PaymentMethod` keep their public ID, discriminator, billing details, timestamps, and other common fields. A private `DeferredFields` bag owns the remaining variant and unknown fields. Serialization combines the current common fields and that bag, without a duplicate cached root. Serde and miniserde use the same value decoder; common fields validate during parsing, and variant payloads validate when accessed. Unknown future branch names survive round trips.

Type-only records such as `PaymentMethodDetails` retain the compact `RecordValue` wrapper. Facade extension methods such as `details.card()?` borrow and decode only that field. Import `stripe::prelude::*` or a specific trait such as `stripe::PaymentMethodDetailsExt`. Each raw record has one extension trait; its methods are enabled by the payload decoder features, while the trait and implementation require the receiver model. Model crates keep the generic `field_as<T>` entry point. Missing/null fields return `None`; malformed present fields fail on access. Access follows field presence independently of the discriminator, preserving the schema's existing acceptance rules. Repeated calls decode again without caching. The `payment_method_details_card` flag remains independently selectable. Additional optional decoders are grouped by stable existing domains (for example, `payment_decoders` or `core_decoders`) rather than generating hundreds of globally forwarded schema flags. Decoder discovery reaches a fixed point as payload models reveal further lazy records.

Response unions spanning distinct object types use opaque named families. Live/deleted pairs retain their existing concrete enums and eager variant decoding: the small deleted model does not justify removing the matchable distinction. Their ordinary model dependencies follow the response automatically. These enums keep the existing decoder behavior: miniserde selects the `deleted` flag, while serde uses untagged matching and can select the live variant when a payload satisfies both schemas. Expandable payloads can still be decoded into these enums after checking their shared wire tag. Resource payloads can be enabled through decoder-only groups without activating their HTTP builders. Request-input unions retain their existing typed constructors and serializers.

Named decoders are defined in one facade extension trait per raw record. Receiver and payload types can occupy different crates; the facade already depends on both. Trait and implementation availability follow the receiver, and each method additionally follows its payload decoder. No accessor-only edges enter the generated model graph, and there are no accessor colocation rules. Pure type-only record wrappers join IDs and opaque object families in lightweight Shared support, so a standalone decoder can use them without enabling a parent resource.

The wire tag is not a unique schema identity: live and deleted models can share it. Callers who need that distinction can decode into the existing live/deleted enum. Explicit conversion to a leaf model follows that decoder's existing structural acceptance rules; the experiment adds no stricter live/deleted validation to leaf models.

Selected models and endpoint builders are available at the root (`stripe::Charge`, `stripe::CreateCharge`). Request helpers live under their resource module (`stripe::charge::OptionalFieldsAddress`), so different groups can use the same helper name. Common runtime types such as `stripe::Currency` and `stripe::ObjectValue` are available without another dependency.
Available model modules and their public helpers are reexported according to actual availability, including `stripe::PaymentMethodDetails` and the independently enabled `stripe::PaymentMethodDetailsCard`.

Webhooks gate typed payloads using the same model availability plan. Unknown and disabled payloads preserve their complete data in `EventObject::Unknown` on both serde and miniserde paths; malformed enabled models still fail. The standalone parser enables all model groups and rejects unknown event variants. Opaque expandable/polymorphic subtrees remain opaque there too, so this is not recursive validation of every retained subtree.

Where a generated package implements a group's HTTP requests, it distinguishes an internal model flag from the public resource/request flag. Other groups use their existing flag directly for models. A webhook enables the backing model flags, without enabling HTTP builders or the optional client-interface dependency. The public facade enables models and their selected endpoints together. `full` contains public groups. The isolated check also verifies that publishable manifests stay within [crates.io's default 300-feature limit](https://doc.rust-lang.org/cargo/reference/features.html), including implicit dependency features.

This is an experimental v1 SDK representation and an optional crate-placement candidate. The default experiment keeps the current crate map; consumer placement changes model locations within the same packages. Repository installation remains rejected while the normal installer lacks ownership of the additional facade, webhook, and test manifests. Those manifests record generated model groups so regeneration can remove stale flags/dependencies while preserving runtime settings. Group selection remains a generator development API/example, with no new normal-generation CLI flag.
