"""Check release reporting can resolve its repository without a checkout."""

import unittest
from pathlib import Path

import yaml


class ReleaseRepositoryTests(unittest.TestCase):
    def test_checkout_free_release_lookup_selects_repository_explicitly(self) -> None:
        root = Path(__file__).resolve().parents[2]
        workflow = yaml.safe_load((root / ".github/workflows/publish.yaml").read_text())
        steps = workflow["jobs"]["release-finalize"]["steps"]
        report = next(step for step in steps if step.get("name") == "Verify every enabled publish target succeeded")
        assert 'gh release view "$TAG"' in report["run"]
        assert report["env"]["GH_REPO"] == "${{ github.repository }}"


if __name__ == "__main__":
    unittest.main()
