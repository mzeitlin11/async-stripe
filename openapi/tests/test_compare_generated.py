"""Regression cases for the parity check's success and failure conditions."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest


SCRIPT = Path(__file__).with_name("compare_generated.py")


class CompareGeneratedTests(unittest.TestCase):
    def setUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.before = Path(self.directory.name) / "before"
        self.after = Path(self.directory.name) / "after"
        self.before.mkdir()
        self.after.mkdir()

    def run_comparison(self):
        return subprocess.run(
            [sys.executable, str(SCRIPT), str(self.before), str(self.after)],
            capture_output=True,
            text=True,
        )

    def test_directory_only_trees_cannot_claim_parity(self):
        for root in [self.before, self.after]:
            (root / "crates/async-stripe-core").mkdir(parents=True)
        result = self.run_comparison()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("expected generated files", result.stderr)
        self.assertEqual(result.stdout, "")

    def test_equal_trees_include_hidden_files(self):
        for root in [self.before, self.after]:
            (root / "Cargo.toml").write_text("manifest")
            (root / ".marker").write_text("owned")
        result = self.run_comparison()
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout)
        self.assertTrue(report["equal"])
        self.assertEqual(report["before_files"], 2)

    def test_changed_added_and_removed_files_fail(self):
        (self.before / "Cargo.toml").write_text("before")
        (self.after / "Cargo.toml").write_text("after")
        (self.before / "removed.rs").write_text("removed")
        (self.after / "added.rs").write_text("added")
        result = self.run_comparison()
        self.assertEqual(result.returncode, 1)
        report = json.loads(result.stdout)
        self.assertEqual(report["changed"], ["Cargo.toml"])
        self.assertEqual(report["removed"], ["removed.rs"])
        self.assertEqual(report["added"], ["added.rs"])


if __name__ == "__main__":
    unittest.main()
