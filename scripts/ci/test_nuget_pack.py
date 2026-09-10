"""Guard the release workflow's managed and native NuGet package closure."""

from __future__ import annotations

import json
import os
import subprocess
import tempfile
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
PACKAGE = "XbergIo.TreeSitterLanguagePack"
MANAGED = "TreeSitterLanguagePack.csproj"
RUNTIME = "../TreeSitterLanguagePack.Runtime/TreeSitterLanguagePack.Runtime.csproj"
OUTPUT = "../../../dist/nuget"


class NugetPackWorkflowTests(unittest.TestCase):
    def setUp(self) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
        self.steps = workflow["jobs"]["build-csharp-package"]["steps"]
        self.pack = next(step for step in self.steps if step["name"] == "Build and pack NuGet")
        template = ROOT / "packages/csharp/TreeSitterLanguagePack/runtime.json.template"
        self.graph = json.loads(template.read_text().replace("{{VERSION}}", "1.17.0"))
        self.rids = sorted(rid for rid, entries in self.graph["runtimes"].items() if PACKAGE in entries)

    def run_pack(self, fail_rid: str = "") -> tuple[subprocess.CompletedProcess[str], list[list[str]]]:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "runtime.json").write_text(json.dumps(self.graph))
            calls = root / "calls.jsonl"
            dotnet = root / "dotnet"
            dotnet.write_text(
                "#!/usr/bin/env python3\n"
                "import json, os, sys\n"
                "with open(os.environ['PACK_CALLS'], 'a') as log:\n"
                "    log.write(json.dumps(sys.argv[1:]) + '\\n')\n"
                "if os.environ['FAIL_RID'] and '-p:PublishedRID=' + os.environ['FAIL_RID'] in sys.argv:\n"
                "    sys.exit(17)\n"
            )
            dotnet.chmod(0o755)
            environment = {
                **os.environ,
                "PATH": f"{root}:{os.environ['PATH']}",
                "PACK_CALLS": str(calls),
                "FAIL_RID": fail_rid,
            }
            result = subprocess.run(
                ["bash", "-e", "-o", "pipefail", "-c", self.pack["run"]],
                cwd=root,
                env=environment,
                capture_output=True,
                text=True,
                check=False,
            )
            recorded = [json.loads(line) for line in calls.read_text().splitlines()] if calls.exists() else []
            return result, recorded

    def test_pack_includes_every_concrete_runtime_before_managed_package(self) -> None:
        result, calls = self.run_pack()
        assert result.returncode == 0, result.stderr
        expected = [
            ["pack", RUNTIME, "-c", "Release", f"-p:PublishedRID={rid}", "--output", OUTPUT] for rid in self.rids
        ]
        expected.append(["pack", MANAGED, "-c", "Release", "--output", OUTPUT])
        assert calls == expected
        assert len(self.rids) == 6

    def test_runtime_pack_failure_stops_before_managed_package(self) -> None:
        result, calls = self.run_pack(self.rids[0])
        assert result.returncode == 17, result.stderr
        assert len(calls) == 1
        assert calls[0][1] == RUNTIME

    def test_artifact_upload_includes_all_packages(self) -> None:
        upload = next(step for step in self.steps if step["name"] == "Upload NuGet package")
        assert upload["with"]["path"] == "dist/nuget/*.nupkg"


class NugetRecoveryWorkflowTests(unittest.TestCase):
    def setUp(self) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
        recovery = workflow["jobs"]["recover-nuget-runtimes"]
        self.recovery = recovery
        self.validate = next(step for step in recovery["steps"] if step.get("id") == "source")["run"]
        self.source = {
            "repository": {"full_name": "xberg-io/tree-sitter-language-pack"},
            "path": ".github/workflows/publish.yaml",
            "event": "release",
            "head_branch": "v1.17.0",
            "head_sha": "72a6c3cb0a8e5117c91fe93b63aadf6009dfac98",
        }

    def validate_source(self) -> tuple[int, str]:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "source.json").write_text(json.dumps(self.source))
            command = root / "gh"
            command.write_text("#!/usr/bin/env bash\ncat source.json\n")
            command.chmod(0o755)
            output = root / "output"
            environment = {
                **os.environ,
                "PATH": f"{root}:{os.environ['PATH']}",
                "RELEASE_RUN_ID": "34390078000",
                "EXPECTED_TAG": "v1.17.0",
                "GITHUB_REPOSITORY": "xberg-io/tree-sitter-language-pack",
                "GITHUB_OUTPUT": str(output),
            }
            result = subprocess.run(
                ["bash", "-e", "-o", "pipefail", "-c", self.validate],
                cwd=root,
                env=environment,
                capture_output=True,
                text=True,
                check=False,
            )
            return result.returncode, output.read_text() if output.exists() else ""

    def test_recovery_respects_dry_run(self) -> None:
        publish = next(step for step in self.recovery["steps"] if "publish-nuget@" in step.get("uses", ""))
        assert publish["if"] == "${{ !inputs.dry_run }}"

    def test_recovery_accepts_only_the_original_release_source(self) -> None:
        code, output = self.validate_source()
        assert code == 0
        assert output == f"sha={self.source['head_sha']}\n"

    def test_recovery_rejects_wrong_repository_workflow_event_or_tag(self) -> None:
        for key, value in (
            ("repository", {"full_name": "other/repository"}),
            ("path", ".github/workflows/ci.yaml"),
            ("event", "pull_request"),
            ("head_branch", "v1.16.2"),
        ):
            with self.subTest(key=key):
                original = self.source[key]
                self.source[key] = value
                code, output = self.validate_source()
                assert code != 0
                assert output == ""
                self.source[key] = original


if __name__ == "__main__":
    unittest.main()
