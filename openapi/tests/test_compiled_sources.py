import json
from pathlib import Path
import tempfile
import unittest

from verify_generated_workspace import compiled_generated_sources_from_messages


class CompilerSourcesTest(unittest.TestCase):
    def collect(self, root, paths):
        workspace = root / "SDK with spaces"
        source = workspace / "generated/async-stripe-example/src/mod.rs"
        source.parent.mkdir(parents=True)
        source.write_text("// generated")
        artifact = root / "libstripe_example-123.rmeta"
        dep_info = root / "stripe_example-123.d"
        dep_info.write_text(
            f"{artifact}: " + " ".join(str(path).replace(" ", r"\ ") for path in paths(source))
        )
        message = json.dumps({
            "reason": "compiler-artifact",
            "target": {"src_path": str(source)},
            "filenames": [str(artifact)],
        })
        return compiled_generated_sources_from_messages(message, workspace)

    def test_absolute_and_workspace_relative_paths_with_escaped_spaces(self):
        for relative in [False, True]:
            with self.subTest(relative=relative), tempfile.TemporaryDirectory() as directory:
                def paths(source):
                    return [Path("generated/async-stripe-example/src/mod.rs") if relative else source]
                self.assertEqual(
                    self.collect(Path(directory), paths),
                    ["generated/async-stripe-example/src/mod.rs"],
                )

    def test_rejects_a_cached_artifact_from_another_generated_sdk(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            def paths(source):
                other = root / "other SDK/generated/async-stripe-example/src/mod.rs"
                other.parent.mkdir(parents=True)
                other.write_text("// incompatible layout")
                return [other]
            with self.assertRaisesRegex(RuntimeError, "another generated SDK"):
                self.collect(root, paths)


if __name__ == "__main__":
    unittest.main()
