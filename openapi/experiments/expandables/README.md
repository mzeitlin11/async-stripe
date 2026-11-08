# Expandable implementation on the refactor landing branch

The complete pinned v1 surface can now use retained expandable payloads and independently enabled concrete models. From openapi, stage the full SDK with:

    cargo +1.89.0 run --locked --offline -- generate --opaque --no-install --out /tmp/stripe-opaque-review

Omit --no-install to install it, including facade and manifest updates. Add --consumer-layout to place models by represented consumers within the existing packages. The default generator and the SDK files checked into this branch remain concrete. Use a separate checkout for an installed opaque SDK; regenerate it with the same mode/layout flags.

The neutral implementation remains at refactor/codegen-neutral, commit 4a483d791bf58cadf45968d4e6de02fb832ab0de. Feature commits follow it on refactor/codegen-landing. No upstream PR or production pin was changed.

## Review sequence

1. Start with [consumer cases](../../tests/fixtures/opaque-consumer/src/lib.rs) and [mixed records](../../tests/fixtures/opaque-consumer/src/mixed_records.rs). These demonstrate ID access, typed decoding, malformed and future payloads, independent decoders, retrieval through a mock client, and ordinary typed option bags. The adjacent webhook consumer shows model-only activation without HTTP builders.
2. Read runtime object_value.rs and expanded_value.rs, then record_value.rs and deferred_fields.rs. JSON is parsed eagerly; concrete construction is deferred. Consuming conversion uses the retained JSON tree. Record field access clones/decodes that subtree on each call.
3. Read ItemCatalogDraft::use_opaque_relationships and item_catalog/polymorphism.rs. All expandables are transformed, handling tagged and untagged resources separately. Distinct-object unions become opaque families. Live/deleted pairs remain concrete enums, and independent payment settings remain typed.
4. Follow CodeGen::build through draft mutation, RustModel::from_draft, placement, and rendering. Properties and canonical requests are finalized once. The feature no longer repairs a finished model or rebuilds requests manually.
5. Read model_availability.rs, facade.rs, and Cargo/webhook writers. Availability uses the existing canonical Rust-use traversal. Optional decoding methods belong to facade traits, so returning a DTO creates no model-to-model crate edge.
6. Review ComponentLayout::with_consumer_model_crates separately. This optional policy retains package names, service exports, and request ownership. It does not implement arbitrary DAG placement; default opaque generation keeps the legacy layout.

## Behavioral scope

Opaque fields retain future object types and unknown properties. ID access does not require a concrete DTO. An expanded object validates its envelope and identity immediately; DTO required fields validate during typed decoding. Optional polymorphic record fields validate when accessed. These intentionally differ from eager concrete decoding.

Tests exercise common fields, request construction, serialization forwarding, and model-only webhook activation. Enabled malformed webhook models fail; unknown or disabled payloads preserve their JSON. The standalone parser enables all model groups and rejects unknown event variants. Retained subtrees still defer validation.

A wire discriminator is not a unique schema identity: live and deleted objects can share it. Conversion follows the selected decoder. Existing miniserde deleted selection and serde untagged selection remain intact.

This is complete for production's pinned v2578 input, API 2026-09-30.endive. It does not update the schema, implement v2, select a new default production layout, or claim measured compilation speedups.

## Validation

See the [validation record](../../tests/expandable_landing_validation.json) and its adjacent evidence archive for exact inputs, toolchains, commands, physical field counts, compiler source closures, installation checks, and source-preservation results.

Concrete generation matches all 1,851 neutral staging files byte for byte. Both opaque layouts pass full v1 fixtures/parser tests and independent consumer checks. The real installer is exercised in a disposable destination, including rejection of inconsistent concrete/opaque manifests before mutation.

Counts distinguish physical crate contents from actual rustc source closures. They count canonical structs with generated miniserde deserialization once, excluding request-only fields, enum variants, IDs, builders, and arbitrary JSON contents. A source-level count is checked against the canonical opaque manifest.

Final mode checks run sequentially, cleaning local SDK artifacts between modes. Both verifiers share a compiler-source collector that rejects dep-info from another SDK workspace, with tests for absolute/relative paths and escaped spaces. Earlier interleaved diagnostic runs were superseded; their compiler closure results are not final evidence.

| Mode | Physical Shared fields | Physical total | Customer consumer fields |
|---|---:|---:|---:|
| Concrete | 4,863 | 6,986 | 4,944 |
| Opaque, legacy layout | 4,433 | 6,392 | 447 |
| Opaque, consumer layout | 656 | 6,392 | 447 |

The Customer checks request the Customer surface in each representation; the opaque API deliberately defers retained subtrees. These are field/input observations, not matched decoding semantics or measured speedups. Charge-only compiles 147 generated deserializable fields in either opaque layout.

Build caches belong to one generated workspace at a time. For different workspace paths or layouts, use separate SDK targets or clean local SDK packages between runs. The evidence archive includes the sequential cleanup driver used here. No target directories are inside source workspaces.
