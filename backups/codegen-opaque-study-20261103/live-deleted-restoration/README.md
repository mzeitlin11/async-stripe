# Concrete live/deleted response enums

The opaque v1 experiment now preserves ordinary `MaybeDeleted` enums. General response unions over distinct object types and flat discriminator-based records remain deferred. Expandable values remain raw and may be decoded into the restored enum explicitly.

All 11 restored response enums retain the same variant names and payload model identities as normal generation. The general opaque-object item no longer carries an unused optional shared tag.

Compared with implementation baseline `951e1c207e380839686474bc039f12da6e531b7a`:

- All 6,859 canonical declaration identities, names, origins, and sharing rules are retained.
- Item placements, component counts, semantic dependency facts, occurrence provenance, and record decoder methods are unchanged.
- Model feature closures and the crate/model dependency graphs are unchanged. Shared remains at 166 model-bearing modules and 655 struct fields.
- Normal generation remains byte-identical across 1,758 artifacts.
- The standard Customer tests use their original concrete enum matches in both representations.
- Consumer tests cover direct live/deleted parsing, explicit decoding from expandable storage, initial validation, serialization round trips, the automatic availability of the deleted model, and the existing decoder behavior for overlapping schemas.

The existing decoders are retained: miniserde selects the `deleted` flag, whereas serde's untagged enum tries the live variant first and can select it for payloads that satisfy both schemas. This compatibility detail is explicitly tested and documented; it is not new behavior in the generated enums.

`summary.json` contains the complete restored-enum list. `analyze.py` verifies those API shapes and dependency/layout invariants against the retained baseline and normal output. `concrete-parity.json` stores the normal artifact hashes. Validation logs and generated-consumer summaries are retained alongside this directory.

Reproduce the primary checks from the implementation worktree:

```console
cd openapi
cargo +1.89.0 test --locked --offline
cargo +1.89.0 clippy --locked --offline --all-targets -- -D warnings
python3 tests/verify_opaque_groups.py --full-v1 --consumer-layout --keep-workspace
python3 tests/verify_opaque_groups.py
```
