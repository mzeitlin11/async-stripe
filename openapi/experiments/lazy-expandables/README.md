# Expandable-only experiment

Branch `expt/codegen-lazy-expandables` starts directly from the completed neutral refactor, `4a483d791bf58cadf45968d4e6de02fb832ab0de`. Runtime support is commit `383f7245`; generation and consumer tests are `7d59c448`. The broader opaque experiment remains on `refactor/codegen-landing` at `ce446cb33e3c97d467adaa792b80ab642f7eb50f` for comparison.

Only `Expandable<T>` storage changes: generated fields use `LazyExpandable<T::Id>` with the existing concrete ID type. The transform runs before canonical properties and request models are finalized. The existing represented-use traversal sees the ID reference automatically.

Ordinary object unions remain concrete enums. Payment-method details and API-error diagnostic objects remain typed and eager, including their ordinary required fields. Their own expandable relationships receive the same lazy treatment as other expandables. Request builders, public exports, crate placement, feature activation, schema pins, and fixtures retain the neutral base's policy. This experiment does not include the broader branch's model groups, record decoder traits, raw record bags, or consumer-based layout.

## Generate and verify

From `openapi`, stage the full pinned SDK without installing it:

```sh
cargo +1.89.0 run --locked --offline -- generate --lazy-expandables --no-install --out /tmp/stripe-lazy-expandables
```

Omit `--no-install` to install the generated bindings in a task-owned checkout. Default generation and the SDK files committed to this branch remain concrete.

From the repository root, verify the experiment in an isolated workspace:

```sh
python3 -B openapi/tests/verify_generated_workspace.py --lazy-expandables --keep-workspace
```

The existing verifier checks all generated packages, a narrow Customer consumer, webhooks, fixtures, deserialization tests, and the parser. This mode also runs the [six focused consumer tests](../../tests/fixtures/lazy-expandable-consumer/src/lib.rs).

## Consumer contract

```rust
let reference = charge.customer.unwrap(); // LazyExpandable<CustomerId>
let customer = reference.into_expanded_as::<Customer>()?; // Option<Customer>
```

An unexpanded ID produces `Ok(None)`. An expanded object retains its complete JSON; the ID is validated immediately and the remaining model fields validate during conversion. JSON parsing is eager. Unknown fields and object types survive storage and serialization.

Conversions use the existing `Object` and `FromValueOpt` traits. A target must have the relationship's ID type. Union-backed relationships decode into their existing union before matching a variant. The existing model decoder determines discriminator acceptance, including for resources without an `object` tag; there is no additional discriminator policy or generated trait.

`to_expanded_as` retains the reference and clones its JSON on each call, without requiring a cloneable ID. `into_expanded_as` consumes the JSON without cloning. Neither caches decoded models. Optional resource IDs, such as an expanded Invoice's missing/null ID, keep their existing type and acceptance rules.

## Review and evidence

Start with the consumer tests, then [runtime storage](../../../async-stripe-types/src/lazy_expandable.rs), the [representation transform](../../src/item_catalog/lazy_expandables.rs), and `CodeGen::build`. The remaining generator changes select the transform and render the new container. The ordinary deserialization tests use representation-independent serialization checks for expanded values; the focused consumer tests check typed decoding and intentional late failures.

See [validation.json](validation.json) and its evidence archive for exact provenance. Default generation matches all 1,851 neutral staging files byte for byte. Lazy generation changes 169 Rust files, exclusively in expandable type expressions and rustfmt formatting; manifests, exports, layout, requests, and generated fixture declarations are byte-identical. The rendered expressions occur in fields and generated decoder/builder code, so their occurrence count is not a DTO field count.

The final verification passes 87 generator library tests, one CLI test, 18 default/22 all-feature runtime tests, three Python tests, strict generator/runtime/consumer Clippy, a release build, the full SDK checks, and all six focused consumer tests.

All concrete models remain present, and IDs remain in their existing model modules. Physical field counts and the full compiler source closure therefore stay unchanged: all-feature compilation uses the same 1,834 generated source files as the neutral base. This experiment isolates lazy expandable storage and decoding; no compilation speedup or packaging reduction is claimed.

## Focused review

The follow-up review at `6278757ec3c7b43cbfaa8ec4b531cacb75fe6184` removes identity inference from live/deleted return-union classification. Only the metadata that generates an `Object` implementation can grant an expandable target its identity. Unsupported targets now fail during the transform instead of producing a lazy relationship that the typed conversion API cannot decode. Ordinary matchable return unions remain supported in their existing positions.

Regression coverage checks that classification cannot invent an identity and that request signatures receive the finalized lazy representation. CLI configuration, version, and documentation-map loading now use one shared path. Runtime storage, conversion, optional-ID behavior, cloning, serialization, and redaction were reviewed without requiring changes.

All 89 generator library tests, the CLI test, strict generator Clippy, and the release build pass. Both concrete and lazy generation match all 1,851 files from the earlier verified outputs byte for byte. The prior full SDK/runtime/consumer results remain applicable; those suites were not rebuilt in this pass. See [review-validation.json](review-validation.json) and its small evidence archive. The original validation record remains historical evidence.
