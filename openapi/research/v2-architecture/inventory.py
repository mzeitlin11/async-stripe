#!/usr/bin/env python3
"""Inspect schema structure without running or changing the Rust generator.

Counts are schema occurrences, not generated declarations or compiler inputs.
Input provenance and hashes are recorded separately in inputs.json.
"""

import argparse
import hashlib
import json
from collections import Counter
from pathlib import Path


METHODS = {"get", "post", "delete", "put", "patch", "head", "options", "trace"}
PREFIX = "#/components/schemas/"


def schema_nodes(schema, path=()):
    if not isinstance(schema, dict):
        return
    yield path, schema
    for key in ("properties", "patternProperties", "$defs", "definitions"):
        for name, child in schema.get(key, {}).items():
            yield from schema_nodes(child, (*path, key, name))
    for key in ("items", "additionalProperties", "not", "contains", "propertyNames"):
        yield from schema_nodes(schema.get(key), (*path, key))
    for key in ("oneOf", "anyOf", "allOf"):
        for index, child in enumerate(schema.get(key, [])):
            yield from schema_nodes(child, (*path, key, str(index)))


def references(schema):
    return [
        {"path": "/".join(path), "reference": node["$ref"]}
        for path, node in schema_nodes(schema)
        if "$ref" in node
    ]


def shape(schema):
    nodes = [node for _, node in schema_nodes(schema)]
    return {
        "objects_with_properties": sum(bool(node.get("properties")) for node in nodes),
        "property_entries": sum(len(node.get("properties", {})) for node in nodes),
        "enum_nodes": sum(bool(node.get("enum")) for node in nodes),
        "enum_entries": sum(len(node.get("enum", [])) for node in nodes),
        "nullable_nodes": sum(node.get("nullable", False) for node in nodes),
        "union_nodes": sum("oneOf" in node or "anyOf" in node for node in nodes),
        "all_of_nodes": sum("allOf" in node for node in nodes),
        "discriminator_nodes": sum("discriminator" in node for node in nodes),
        "expandable_annotations": sum("x-expandableFields" in node for node in nodes),
        "expansion_resource_annotations": sum("x-expansionResources" in node for node in nodes),
        "date_time_nodes": sum(node.get("format") == "date-time" for node in nodes),
    }


def load(path):
    data = path.read_bytes()
    return json.loads(data), hashlib.sha256(data).hexdigest()


def inspect(path, legacy):
    spec, sha256 = load(path)
    schemas = spec["components"]["schemas"]
    v2 = {name: schema for name, schema in schemas.items() if name.startswith("v2.")}
    operations = []
    for endpoint, item in spec["paths"].items():
        if not endpoint.startswith("/v2/"):
            continue
        for method, operation in item.items():
            if method not in METHODS:
                continue
            content = operation.get("requestBody", {}).get("content", {})
            operations.append({
                "method": method,
                "path": endpoint,
                "request_media_types": list(content),
                "body_shapes": {
                    media: shape(value.get("schema")) for media, value in content.items()
                },
                "parameters": [
                    {key: parameter[key] for key in ("name", "in", "required") if key in parameter}
                    for parameter in operation.get("parameters", [])
                ],
                "responses": {
                    status: {
                        media: references(value.get("schema"))
                        for media, value in response.get("content", {}).items()
                    }
                    for status, response in operation.get("responses", {}).items()
                },
            })
    cross_namespace = []
    v2_model_references = []
    for name, schema in schemas.items():
        for reference in references(schema):
            target = reference["reference"]
            if name.startswith("v2."):
                v2_model_references.append({"source": name, **reference})
            if target.startswith(PREFIX) and name.startswith("v2.") != target[len(PREFIX):].startswith("v2."):
                cross_namespace.append({"source": name, **reference})
    result = {
        "sha256": sha256,
        "api_revision": spec["info"]["version"],
        "all_paths": len(spec["paths"]),
        "all_schemas": len(schemas),
        "v2_paths": sum(endpoint.startswith("/v2/") for endpoint in spec["paths"]),
        "v2_operations": len(operations),
        "v2_methods": dict(sorted(Counter(operation["method"] for operation in operations).items())),
        "v2_named_schemas": len(v2),
        "v2_resource_metadata_count": sum("x-stripeResource" in schema for schema in v2.values()),
        "v2_detailed_error_schemas": sum(name.startswith("v2.error.") for name in v2),
        "v2_schema_extensions": dict(sorted(Counter(
            key for schema in v2.values() for key in schema if key.startswith("x-")
        ).items())),
        "v2_model_references": v2_model_references,
        "cross_namespace_model_references": cross_namespace,
        "v2_non_error_shapes": {
            name: shape(schema) for name, schema in v2.items() if not name.startswith("v2.error.")
        },
        "thin_event_schema_names": [
            name for name, schema in schemas.items()
            if schema.get("x-stripeEvent", {}).get("kind") == "thin"
        ],
        "v2_operations_inventory": operations,
    }
    if legacy:
        old, old_hash = load(legacy)
        old_paths = set(old["paths"])
        new_v1_paths = {endpoint for endpoint in spec["paths"] if endpoint.startswith("/v1/")}
        old_schemas = old["components"]["schemas"]
        result["legacy_comparison"] = {
            "legacy_sha256": old_hash,
            "legacy_paths": len(old_paths),
            "legacy_schemas": len(old_schemas),
            "old_only_paths": sorted(old_paths - new_v1_paths),
            "combined_only_v1_paths": sorted(new_v1_paths - old_paths),
            "equal_schema_values": sum(schema == schemas.get(name) for name, schema in old_schemas.items()),
        }
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sdk", type=Path, required=True)
    parser.add_argument("--public", type=Path, required=True)
    parser.add_argument("--legacy", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    report = {
        "measurement": "Raw schema occurrences; vendor annotations and examples excluded from structural traversal",
        "sdk": inspect(args.sdk, args.legacy),
        "public": inspect(args.public, None),
    }
    args.output.write_text(json.dumps(report, indent=2) + "\n")


if __name__ == "__main__":
    main()
