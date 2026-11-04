# Verified thin-event slice

This extends the concrete v2 experiment at `6cee8654383288f90bab0a1f972bde3a809d1fb9`. It uses the same recorded v2494 combined GA SDK input, API revision `2026-08-26.dahlia`, SHA-256 `92363b86025d012852f648554372bf5fa6146af1472fb63f3c29d7c5fe6bd19a`. Production input selection is unchanged.

The selected event types are `v2.core.account.updated` and `v1.billing.meter.error_report_triggered`, plus an unknown-notification fallback. Notification handling, full-event decoding, and related-resource decoding have independent availability. The [layout](gen_crates.toml) puts the two fetched-event models in an experimental `async-stripe-events` package and resources in `async-stripe-v2`. These are configurable placements, not a proposed final published package split. There are no per-event feature flags or opaque resource representations.

**API and ownership.** `async-stripe-client-core` exposes `WebhookSignature` behind the optional `webhook-signatures` capability. It authenticates original bytes, checks timestamps, and considers every `v1` signature for signing-secret rotation. The signature scheme name is independent of the API namespace. The existing snapshot webhook API delegates verification to this helper and retains its envelope, decoding, and error API. Verification alone needs no generated resource package.

`EventNotification::parse_verified(body, signature, secret)` verifies before parsing the shared notification envelope. `parse` remains available for separately authenticated deliveries. The envelope preserves reason/request metadata and additional unknown properties. Snapshot object tags are rejected. EventBridge/Azure wrapper extraction is outside this slice.

The generated `stripe_events::Notification::from_notification(notification)` dispatches the selected event types, with `Unknown(EventNotification)` for other types. Its typed variants carry `TypedEventNotification<E>` wrappers over the common envelope; they don't embed the fetched event or related resource. Generated `ThinEvent` metadata associates each wrapper with the full event model, expected event/resource tags, and pinned dated revision.

```rust,ignore
let notification = EventNotification::parse_verified(body, signature, secret)?;
match stripe_events::Notification::from_notification(notification)? {
    stripe_events::Notification::V2CoreAccountUpdated(notification) => {
        let full_event = notification.event_request().send(&client).await?;
        // This is an independent fetch of the current resource, with a DTO
        // dependency selected explicitly by this application.
        let current_account = notification
            .related_request::<stripe_v2::core::Account>().unwrap()
            .send(&client).await?;
    }
    _ => {}
}
```

Full events are ordinary concrete models emitted through canonical lowering, placement, and rendering. The pinned Account event's `changes.before/after` are JSON objects, not complete Account snapshots; its `data` is open JSON. They remain schema-directed JSON values without importing Account. The Billing event's detailed validation-error data is concretely typed. Fetches preserve context, carry `Stripe-Request-Trigger`, select the dated revision, encode event IDs, and follow the related URL's namespace. Related-resource output is caller-selected rather than automatically tied to every DTO in the event catalog; a consumer can use a full SDK model or its own small model.

**Generator findings.** Thin-event kind comes from `x-stripeEvent`, independently of component namespace. A `v1.billing.*` event can be retrieved through `/v2/core/events`. Explicitly classified thin schemas are accepted in the experimental scope; ordinary v1 resource schemas and paths are still rejected. Selected event schemas are retention roots. Multiple event schemas share `object: v2.core.event`; they use string envelope IDs rather than being mistaken for distinct resource identities with conflicting generated ID types. Event metadata emission follows final component placement. Resource DTOs aren't introduced as inferred dependencies merely because a related-object tag names them.

**Fields first.** Normal analysis now produces `deserialization-fields.csv` and `deserialization-crates.csv`. Counts use each canonical generated struct once when its usage emits a miniserde deserializer. This includes response structs in request modules and shared declarations. It excludes request-only parameter structs, generated decoder helpers, ID wrappers, enum variants, fixed object tags, handwritten runtime code, and the contents of arbitrary JSON values. A struct with both serde and miniserde decoding counts once. These are physical generated crate counts; the verifier intersects detailed rows with actual compiler sources to report each consumer's activated generated crates.

| Consumer | Handwritten notification fields in client-core | Generated event fields | Generated resource fields |
|---|---:|---:|---:|
| Notifications only | 16 | 0 | 0 |
| Selected typed events | 16 | 46 | 0 |
| Typed events plus complete Accounts family | 16 | 46 | 810 |

The event package contains 14 deserializable structs. Enabling the resource package adds 260. Actual compiler source lists show zero generated packages for notification-only handling, exactly the three event-package source files for typed events, and the resource package only for the Accounts workload. None compiles legacy Shared. The handwritten count is 9 envelope fields, 3 related-reference fields, and two fields each for reason and originating request; the unknown-property map counts as one declared field.

For context, normal generation places 4,723 canonical deserializable fields in 1,041 structs in v1 Shared. This is a physical-crate observation, not a matched API comparison or speedup claim. Shared foundational generic utilities and handwritten error decoders are outside the generated field reports and remain common runtime costs. No timings, binary benchmarks, or LLVM measurements were needed for this slice.

**Assessment.** Canonical model generation was reusable; the required adaptations were event classification/retention, envelope identity semantics, model-independent verification, and fetch metadata. Thin delivery creates a useful dependency boundary naturally. A typed event catalog need not pull in resource DTOs. Automatically typed related-resource conveniences would need deliberate dependency ownership, such as optional family adapters. The experiment's common envelope plus explicit output type is a simple starting point. Broader event coverage, automatic resource adapters, cloud delivery wrappers, and final public packaging remain open.

**Reproduce.** From the repository root, using task-owned cache/work paths outside source checkouts:

```sh
python3 openapi/tests/verify_v2_slice.py --webhooks \
  --spec /path/to/openapi.spec3.sdk.json --work-dir /tmp/stripe-thin-check \
  --generator-target-dir /tmp/stripe-codegen-target --target-dir /tmp/stripe-thin-target
```

The checker generates the two selected events and the resource comparison scope, runs notification-only/typed-event/Accounts consumers, tests Hyper, Surf, and blocking fetches, checks both decoding implementations for full events, validates signatures/unknown events/invalid targets, and records fields plus compiler sources. Test payloads are synthetic schema-valid examples with test credentials; there are no live Stripe calls. [Results](results/summary.json) and the adjacent evidence archive preserve the field reports, compiler lists, commands, and test results.

Protocol decisions were corroborated against the previously pinned official Python SDK `v16.1.0`, commit `94756ffee7754e2fddc4bf5de4c3a407971128a5`, which targets a newer dated API revision than our schema input: [verification and notification parsing](https://github.com/stripe/stripe-python/blob/94756ffee7754e2fddc4bf5de4c3a407971128a5/stripe/_stripe_client.py), [event/related fetches](https://github.com/stripe/stripe-python/blob/94756ffee7754e2fddc4bf5de4c3a407971128a5/stripe/v2/core/_event.py). These establish runtime behavior, not schema equivalence across the two revisions.
