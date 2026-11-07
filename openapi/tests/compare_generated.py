#!/usr/bin/env python3
"""Compare complete formatted codegen staging trees, including their file sets."""

import argparse
import hashlib
import json
from pathlib import Path


def inventory(root):
    return {
        str(path.relative_to(root)): hashlib.sha256(path.read_bytes()).hexdigest()
        for path in sorted(root.rglob("*"))
        if path.is_file()
    }


def compare(before, after):
    a, b = inventory(before), inventory(after)
    result = {
        "before_files": len(a),
        "after_files": len(b),
        "removed": sorted(a.keys() - b.keys()),
        "added": sorted(b.keys() - a.keys()),
        "changed": sorted(name for name in a.keys() & b.keys() if a[name] != b[name]),
        "before_tree_sha256": hashlib.sha256(json.dumps(a, sort_keys=True).encode()).hexdigest(),
        "after_tree_sha256": hashlib.sha256(json.dumps(b, sort_keys=True).encode()).hexdigest(),
    }
    result["equal"] = a == b
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("before", type=Path)
    parser.add_argument("after", type=Path)
    parser.add_argument("--result", type=Path)
    args = parser.parse_args()
    for root in [args.before, args.after]:
        if not root.is_dir():
            parser.error(f"expected a generated tree directory: {root}")
    result = compare(args.before, args.after)
    for root, count in [(args.before, result["before_files"]), (args.after, result["after_files"])]:
        if count == 0:
            parser.error(f"expected generated files in tree: {root}")
    document = json.dumps(result, indent=2) + "\n"
    if args.result:
        args.result.write_text(document)
    print(document, end="")
    return 0 if result["equal"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
