"""Exercise the bounded PHP release recovery guards."""

import hashlib
import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
WORKFLOW = yaml.safe_load((ROOT / ".github/workflows/recover-php-117.yaml").read_text())


def python_block(job: str, step: str) -> str:
    steps = WORKFLOW["jobs"][job]["steps"]
    script = next(item["run"] for item in steps if item.get("name") == step)
    return script.split("python3 - <<'PY'\n", 1)[1].split("\nPY", 1)[0]


class PhpRecoveryTests(unittest.TestCase):
    def test_original_source_and_gate_guard_rejects_different_commit(self) -> None:
        run = {
            "id": 34390078000,
            "repository": {"full_name": "xberg-io/tree-sitter-language-pack"},
            "event": "release",
            "path": ".github/workflows/publish.yaml",
            "head_sha": WORKFLOW["env"]["RELEASE_SHA"],
            "head_branch": "v1.17.0",
            "status": "completed",
        }
        jobs = [{"name": f"Build PHP PIE binary ({index})", "conclusion": "success"} for index in range(15)]
        jobs.append({"name": "E2E gate — PHP", "conclusion": "success"})
        jobs.extend(
            {"name": f"Build PHP PIE binary (php{version} macos-x86_64)", "conclusion": "failure"}
            for version in ("8.2", "8.3", "8.4")
        )
        jobs.append({"name": "Aggregate release status", "conclusion": "failure"})
        script = python_block("validate-source", "Validate original release and successful PHP gates")
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            (path / "jobs.json").write_text(json.dumps([{"jobs": jobs}]))
            for commit, expected in ((WORKFLOW["env"]["RELEASE_SHA"], 0), ("different", 1)):
                run["head_sha"] = commit
                (path / "source.json").write_text(json.dumps(run))
                result = subprocess.run(
                    ["python3", "-c", script],
                    cwd=path,
                    env={**os.environ, **WORKFLOW["env"]},
                    capture_output=True,
                    check=False,
                )
                assert result.returncode == expected

    def test_archive_guard_rejects_missing_platform_and_corrupted_payload(self) -> None:
        script = python_block("publish-missing", "Verify and upload complete PHP archive set")
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory)
            packages = path / "packages"
            packages.mkdir()
            for index in range(18):
                archive = packages / f"php_{index}.tgz"
                archive.write_bytes(str(index).encode())
                Path(f"{archive}.sha256").write_text(hashlib.sha256(archive.read_bytes()).hexdigest())
            result = subprocess.run(["python3", "-c", script], cwd=path, capture_output=True, check=False)
            assert result.returncode == 0
            archive.write_bytes(b"corrupted")
            result = subprocess.run(["python3", "-c", script], cwd=path, capture_output=True, check=False)
            assert result.returncode != 0
            archive.unlink()
            result = subprocess.run(["python3", "-c", script], cwd=path, capture_output=True, check=False)
            assert result.returncode != 0

    def test_source_builder_rejects_invalid_inputs_before_downloading(self) -> None:
        for series, prefix in (("9.0", "/unused"), ("8.3", "relative")):
            result = subprocess.run(
                ["bash", str(ROOT / "scripts/ci/build-php-source.sh"), series, prefix],
                capture_output=True,
                text=True,
                check=False,
            )
            assert result.returncode != 0
            assert "Unsupported PHP series" in result.stderr or "prefix must be absolute" in result.stderr


if __name__ == "__main__":
    unittest.main()
