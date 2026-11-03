# Concrete v2 integration experiment

The experiment generates Event Destinations, Events, and the Accounts family through the refactored pipeline, and executes their requests through the existing Hyper, Surf, and blocking clients. It starts from the concrete baseline and uses ordinary concrete models. The pinned input, production CLI, normal generated SDK, and opaque-v1 experiment remain independent.

Implementation commits are `f0700e0c` (runtime boundary) and `8c2f3c21` (codegen and verification). The input is the combined GA SDK specification at `v2494`, commit `3e5f190f81ef54900d83909fc733226b87afccb1`, API revision `2026-08-26.dahlia`. [Input hashes and source references](../../research/v2-architecture/inputs.json) and the [original investigation](../../research/v2-architecture/README.md) retain the discovery evidence.

**Using the prototype.** From `openapi`, generate the small slice with explicit input and output:

```console
CARGO_TARGET_DIR=/tmp/stripe-v2-generator cargo +1.89.0 run --locked --offline \
  --example v2_slice -- --spec /path/to/openapi.spec3.sdk.json --out /tmp/stripe-v2-output
```

`--resources` accepts qualified schema names, comma-separated. The defaults are `v2.core.event_destination,v2.core.event`. Add `v2.core.account,v2.core.account_person,v2.core.account_token,v2.core.account_person_token,v2.core.account_link` for the complete experimental Accounts family. The example emits raw staging output; the verifier formats and compiles it. It never installs output. `--layout` selects a file named `gen_crates.toml`; the default [placement configuration](gen_crates.toml) puts everything in one experimental `async-stripe-v2` package. The existing configuration loader is process-scoped, so different configurations use separate invocations.

The generated public paths include `stripe_v2::core::Account`, `stripe_v2::core::EventDestination`, and builders such as `stripe_v2::core::account::Retrieve` and `stripe_v2::core::event_destination::Create`. Qualified internal names remain available. A facade under the client crate's `stripe::v2` is a possible later packaging choice.

A v2-only consumer uses the client with `default-features = false` and a selected transport plus `v2`, for example `features = ["default-tls", "v2"]`. The generated v2 package disables client-core defaults. The client retains default `v1` compatibility for existing consumers; explicitly disabling defaults requires enabling `v1` when those exports and rich legacy errors are wanted. Both features compose. Client/account header setters accept strings or `AsRef<str>` IDs, and version overrides accept strings. These are deliberate runtime API changes; generated v1 DTO/request APIs are unchanged.

**What changed at the boundaries.**

- Request inference and canonical request lowering now retain body encoding, the generated API revision, and URL-based pagination. Operation identities/selectors retain the API namespace. V2 parameter IDs are encoded as individual path segments. Rendering uses these facts, and the existing type-use traversal includes pagination's associated item type.
- Client configuration and request customization store protocol strings rather than requiring generated v1 models. An optional v1 integration supplies the historical revision fallback, ID exports, and rich v1 errors. V2 errors retain code, message, status, and raw response without model dependencies. Generated request revisions take precedence over the legacy fallback; explicit client/request overrides remain available.
- Both transports use the same relative target resolver and execute the selected body media type. Namespace semantics survive custom API-base path prefixes. V2 mutation keys are selected once before retries, including DELETE. The `v2` capability enables UUID generation.
- A separate paginator follows returned v2 targets, preserves account/context/version/strategy/timeout, and handles empty intermediate pages. It rejects external targets. The v1 cursor paginator is unchanged.
- Thin notifications parse independently of resource DTOs. Their generic related-object request lets the caller choose the response type and revision, including a v1 resource, without a dependency from the notification layer to that model. Thin schema annotations are distinguished from snapshot wrappers during inference.
- The explicitly scoped v2 input does not apply the v1 webhook ApiVersion extraction rule. Applicable selectors remain strict. It also avoids mandatory legacy Shared facade dependencies. The prototype has no per-resource request-only features: the selected generated scope exposes models and requests together, with serde/debug capabilities retained.
- V2 Event's open `changes` object exposed a serializer gap: constant-object-tag serialization bypassed the existing JSON-value field plan. It now uses that plan and the existing serde bridge.

**Validation and reproduction.** The checker requires the pinned SDK file, cached Cargo dependencies, nightly rustfmt, and the normal pinned legacy fixtures. A normal `generate --no-install` populates those legacy caches without updating the pin. From the repository root:

```console
python3 openapi/tests/verify_v2_slice.py \
  --spec /path/to/openapi.spec3.sdk.json --work-dir /tmp/stripe-v2-validation \
  --generator-target-dir /tmp/stripe-v2-generator --target-dir /tmp/stripe-v2-sdk-target
```

The verifier checks the input hash and creates isolated SDKs for the Event slice and the combined slice. It checks actual compiler sources, runs the same wire suite with both transports, tests blocking execution and pagination, and verifies mixed v1/v2 Account retrieval. It covers create/retrieve/list/update/delete/enable/disable/ping, JSON null metadata entries, indexed query arrays, required/overridden versions, context/account headers, stable retry keys, escaped IDs, empty pages, and invalid continuation targets. It preserves source bytes. Reserved Shared output containing v2 models is rejected rather than silently replacing the legacy Shared package.

All 24 scoped integration-test executions passed. The 65 generator tests, strict all-target generator/runtime Clippy, release generator build, 15 client-core tests, and existing client tests passed. The normal isolated checker passed all 13 generated packages, standalone webhooks, fixtures, deserialization tests, and parser tests. Its compiled generated source count remains 1,741. Normal generation matches all 1,758 baseline artifacts byte-for-byte.

The pinned Account resource example omits required fields; its create/retrieve examples also use timestamps where the schema specifies boolean `applied`. The verifier uses the schema-valid pinned update example. It does not relax generated required fields or rewrite the schema to accommodate those examples.

**Artifact results.** [summary.json](results/summary.json) records exact bytes, features, collector identity, environment, compiler inputs, and IR scopes. [evidence.tar.gz](results/evidence.tar.gz) preserves passing validation logs, dependency/SCC reports, benchmark reports and command manifests, IR reports, and the measured executables. Compiler targets and generated SDK source snapshots are reproducible and omitted.

The same Event Destination consumer and payload were compiled against two generated scopes, each in one v2 package. Both use Rust 1.89.0 on aarch64 macOS. The existing compile-bench collector measured one build per scope/profile; separate diagnostics selected the consumer and v2 library for release IR. There are no timing samples. To reproduce with a separately built current collector:

```console
python3 openapi/experiments/v2/measure.py --verified /tmp/stripe-v2-validation \
  --collector /path/to/compile-bench --output /tmp/stripe-v2-artifacts
```

| Measure | Events and Event Destinations | Plus Accounts family |
| --- | ---: | ---: |
| Generated operations | 10 | 25 |
| Represented model modules | 3 | 8 |
| Model declarations / fields / explicit variants | 20 / 42 / 19 | 313 / 798 / 633 |
| Cross-module model cycles | 0 | 0 |
| Actual generated compiler source files | 8 | 23 |
| V1 Shared compiled | No | No |
| Dev v2 `.rlib` bytes | 4,367,104 | 53,242,040 |
| Dev v2 `.rmeta` bytes | 706,957 | 12,414,452 |
| Release v2 `.rlib` bytes | 1,333,856 | 21,127,752 |
| Release v2 `.rmeta` bytes | 778,336 | 14,809,934 |
| Release stripped executable bytes | 409,904 | 427,280 |
| Release consumer LLVM lines / copies | 1,448 / 81 | 1,448 / 81 |
| Release v2-library LLVM lines / copies | 47,277 / 1,397 | 609,053 / 17,059 |

Model counts come from canonical declarations and aggregate each SCC once. They exclude request modules, IDs-only modules, helper declarations, and implicit unknown variants as documented by the reports. Compiler-file counts include module/request/index files. Library artifacts and metadata are separate measurements, not additive code sizes. Consumer and library IR exclude their separately compiled dependencies and are not a whole-program total. Seven common generated source files are byte-identical between scopes; the root adds the other APIs.

This is a comparison of API availability within a coarse package, not two layouts exposing the same full API set. It shows that an almost unchanged executable and identical consumer IR can coexist with much larger generated library work. The combined output has 788 canonical items and 2,167 fields overall; request sites account for a substantial part beyond the 313 model declarations and 798 model fields. A future coarse request capability is worth measuring separately from response-model availability.

**Architecture assessment.** The refactor was useful: the slice reused component/operation identities, local deduplication, canonical lowering, item emission, address resolution, dependency traversal, and materialization. It required no alternate renderer or opaque representation. The remaining gaps were operation facts, protocol/model coupling, scope applicability, thin-event classification, and one shared rendering-plan bypass.

For the pinned represented model graphs there are no cross-module model edges, so this slice gives no reason to introduce opaque related-object fields to break such dependencies. Accounts remains substantial internally. Keeping its code in the same package makes a small event consumer compile it; availability and physical placement can address that independently of v1 representation work. Mixed applications that select v1 still pay its concrete Shared costs.

The most useful next refactor steps are:

1. Choose final placement from canonical represented uses after rewrites, retaining component ownership as a hint. The current early component policy still promotes cross-crate dependencies to Shared; a different layout needs deliberate support for ordinary DAG edges and appropriate common-type ownership.
2. Make public facade exports, family/capability availability, manifests, and type uses agree. The prototype's schema scope is its availability mechanism. Adding flags or separate physical packages should preserve public paths and avoid introducing mandatory facade dependencies on every family.
3. Replace remaining process-global/profile assumptions only when needed for another real layout or input. In particular, the reserved `shared` role still names the legacy package. A nonempty v2 common package must have deliberate ownership and naming so it can coexist with v1. A single configurable package is sufficient for the current experiment.

Full v2 support still needs the remaining Billing/Commerce resources, their special auth/endpoint behavior, full typed event/notification coverage and signature integration, and validation across future schema shapes. The ordinary concrete slice and the measured package-size question are complete; those broader changes are not prerequisites for evaluating this result.
