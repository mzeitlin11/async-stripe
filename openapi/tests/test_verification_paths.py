from pathlib import Path
import os
import subprocess
import sys
import tempfile
import unittest

from verify_generated_workspace import REPO, validate_verification_paths


class VerificationPathsTest(unittest.TestCase):
    def test_source_and_disposable_cache_overlaps_are_rejected_without_writes(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            repo, work = root / "repo", root / "checks"
            for invalid_work in [repo, repo / "nested", root]:
                with self.subTest(work=invalid_work), self.assertRaises(ValueError):
                    validate_verification_paths(invalid_work, [root / "cache"], repo=repo)
            for cache in [repo, repo / "target", root, work, work / "workspace",
                          work / "workspace/target", work / "generated-output/target"]:
                with self.subTest(cache=cache), self.assertRaises(ValueError):
                    validate_verification_paths(work, [cache], repo=repo)
            validate_verification_paths(work, [work / "sdk-target", root / "generator-target"], repo=repo)
            self.assertFalse(work.exists())
            self.assertFalse(repo.exists())

    def test_cli_rejects_overlap_before_replacing_a_managed_workspace(self):
        for script, marker in [
            ("verify_generated_workspace.py", ".generated-workspace-verification"),
            ("verify_opaque_groups.py", ".opaque-groups-verification"),
        ]:
            with self.subTest(script=script), tempfile.TemporaryDirectory() as directory:
                work = Path(directory).resolve()
                (work / marker).write_text("original marker")
                source = work / "workspace/important.txt"
                source.parent.mkdir()
                source.write_text("preserve this workspace")
                result = subprocess.run([
                    sys.executable, "-B", str(REPO / "openapi/tests" / script),
                    "--work-dir", str(work), "--target-dir", str(work / "workspace/target"),
                ], cwd=REPO, env=dict(os.environ, PYTHONDONTWRITEBYTECODE="1"),
                    capture_output=True, text=True, timeout=10)
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertIn("overlaps disposable source", result.stderr)
                self.assertEqual(source.read_text(), "preserve this workspace")
                self.assertEqual((work / marker).read_text(), "original marker")

    def test_cli_rejects_repository_work_directory_before_creating_a_marker(self):
        for script in ["verify_generated_workspace.py", "verify_opaque_groups.py"]:
            with self.subTest(script=script), tempfile.TemporaryDirectory(dir=REPO) as directory:
                work = Path(directory).resolve()
                result = subprocess.run([
                    sys.executable, "-B", str(REPO / "openapi/tests" / script),
                    "--work-dir", str(work),
                ], cwd=REPO, env=dict(os.environ, PYTHONDONTWRITEBYTECODE="1"),
                    capture_output=True, text=True, timeout=10)
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertIn("outside the repository", result.stderr)
                self.assertEqual(list(work.iterdir()), [])


if __name__ == "__main__":
    unittest.main()
