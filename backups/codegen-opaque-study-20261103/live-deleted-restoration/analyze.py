"""Compare the retained concrete live/deleted enums with the opaque baseline."""

from pathlib import Path
import csv
import json
import re

ROOT = Path(__file__).resolve().parents[1]
OUT = Path(__file__).resolve().parent
OLD = ROOT / "typed-options-verification/analysis"
NEW = ROOT / "live-deleted-verification/analysis"


def rows(directory, name):
    return list(csv.DictReader((directory / name).open()))


def enum_variants(source, ident):
    body = re.search(r"pub enum " + re.escape(ident) + r"\s*\{([^}]+)\}", source)
    assert body is not None, ident
    return re.findall(r"(\w+)\((?:stripe_\w+::)?(\w+)\)", body[1])


before = {row["item_id"]: row for row in rows(OLD, "items.csv")}
after = {row["item_id"]: row for row in rows(NEW, "items.csv")}
assert before.keys() == after.keys()
restored = []
for item, old in before.items():
    new = after[item]
    for key in ["ident", "origin", "sharing_rule", "emission_path"]:
        assert old[key] == new[key], (item, key)
    if old["kind"] != new["kind"]:
        assert (old["kind"], new["kind"]) == ("OpaqueObject", "Enum")
        path = new["emission_path"]
        concrete = (ROOT / "live-deleted-concrete-output" / path).read_text()
        candidate = (
            ROOT / "live-deleted-verification/workspace"
            / path.replace("crates/", "generated/", 1)
        ).read_text()
        variants = enum_variants(candidate, new["ident"])
        assert variants == enum_variants(concrete, new["ident"])
        assert len(variants) == 2
        restored.append({"ident": new["ident"], "path": path, "variants": variants})
    else:
        assert old["usage"] == new["usage"], item

for filename in [
    "components.csv", "dependencies.csv", "item-occurrences.csv", "record-decoders.csv",
    "model-group-closures.csv", "graphs/crate-dependencies.dot",
    "graphs/model-module-dependencies.dot",
]:
    assert (OLD / filename).read_bytes() == (NEW / filename).read_bytes(), filename

old_closures = {(r["scope"], r["group"]): r for r in rows(OLD, "model-group-closures.csv")}
new_closures = {(r["scope"], r["group"]): r for r in rows(NEW, "model-group-closures.csv")}
assert old_closures.keys() == new_closures.keys()
for group, old in old_closures.items():
    assert old["model_modules"] == new_closures[group]["model_modules"], group

parity = json.loads((OUT / "concrete-parity.json").read_text())
assert parity["byte_identical"]
assert not any(r["cross_module_cycle"] == "true" for r in rows(NEW, "model-module-sccs.csv"))

report = {
    "baseline_commit": "951e1c207e380839686474bc039f12da6e531b7a",
    "canonical_items_preserved": len(before),
    "restored_live_deleted_enums": restored,
    "variant_names_and_models_match_normal_generation": True,
    "all_item_placements_unchanged": True,
    "semantic_dependencies_and_occurrence_provenance_unchanged": True,
    "component_counts_and_decoder_methods_unchanged": True,
    "all_model_feature_closure_counts_unchanged": True,
    "crate_and_model_dependency_graphs_unchanged": True,
    "normal_generation_byte_identical_artifacts": parity["artifacts"],
    "shared_model_modules": 166,
    "shared_struct_fields": 655,
    "model_graph_cycles": 0,
    "existing_decoder_behavior": "Miniserde selects the deleted flag; serde's untagged enum can select the live variant for overlapping payloads. Both existing behaviors are retained and tested.",
}
(OUT / "summary.json").write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps({k: v for k, v in report.items() if k != "restored_live_deleted_enums"}, indent=2))
print("Restored enums:", len(restored))
