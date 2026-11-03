# Reusable refactor baseline

`review/codegen-pipeline` now points to `5c0dafda38b01f022d1076c76cee4272f8f2dc5c`, including a separate two-line compatibility backport `ad6027f1146ae55cb92c79fefd514a3e61864562` of upstream Reserve serde forwarding. Fork draft PR #3 describes the updated implementation and validation.

The baseline keeps the concrete generated SDK. Shared item/request type-use visitors feed one represented module-dependency builder. Cargo dependencies project that graph plus existing facade consumers; optional analysis recomputes the graph for deterministic module/SCC reports with canonical counts. No opaque types, model availability planner, consumer placement policy, or generated facade extension machinery is included.

`expt/codegen-v2` starts at that same commit. No v2 implementation has begun. The separate opaque experiment remains at `176c3f588c4e1553cad274c90f2f1a4a28e41772`.

The generic `openapi/tests/verify_generated_workspace.py` generates without installing, stages an isolated SDK, checks all 13 generated crates and webhooks with all features, runs standalone webhook tests, generated fixture and deserialization tests, and parser tests, and records actual compiler source files via rustc dep-info. It supports absolute and workspace-relative paths and preserves explicitly retained build caches.

Validation passed: 64 generator tests, strict all-target generator Clippy, release generator build, byte-identical normal output across 1,758 artifacts on v2494, 20 deterministic all-graph analysis files, stale-report ownership checks, and the complete isolated SDK verification. Repository source bytes remained unchanged. Counts from module analysis describe canonical declarations, not compiler source closure or feature activation; compiler-source counts are recorded separately.

`fix/codegen-experiment-correctness`, based on local upstream `cf3abb55`, retains two orthogonal signed commits: `8a9b6f11` rejects invalid optional cursors, and `0297ca75` adds Reserve to the webhook `full` feature. All 16 runtime types tests and strict Clippy pass; Cargo metadata verifies Reserve activation and existing serde forwarding. These fixes are independent of the refactor and opaque experiment.

`artifacts.tar.gz` retains the new analysis, artifact hashes, validation logs, compiler-source lists, upstream compatibility patch, draft description, and independent-fix validation. `manifest.json` records verified file hashes. SDK source workspaces and build caches are reproducible and omitted. Historical failed checker runs remain identifiable in the validation logs; the final `generated-verification/summary.json` and `validation-summary.json` record the passing state.
