"""Keep release dry runs read-only while retaining artifact validation."""

import contextlib
import io
import os
import re
import runpy
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import tomllib
import yaml

ROOT = Path(__file__).resolve().parents[2]
OUTPUT_DRY = "needs.prepare.outputs.dry_run"
INPUT_DRY = "inputs.dry_run"


def allows_mutation(condition: str, variable: str, value: bool | str) -> bool:
    condition = condition.replace("${{", "(").replace("}}", ")")
    comparisons = re.findall(rf"(?:^|&&|\()\s*{re.escape(variable)}\s*!=\s*('true'|true)\s*(?=&&|\)|$)", condition)
    for comparison in comparisons:
        expected = True if comparison == "true" else "true"
        if type(value) is type(expected) and value == expected:
            return False
    return True


def undefined_dispatch_inputs(workflow: dict) -> set[str]:
    events = workflow.get("on", workflow.get(True))
    declared = set(events["workflow_dispatch"]["inputs"])
    expressions = re.findall(r"\$\{\{(.*?)\}\}", yaml.safe_dump(workflow), re.DOTALL)
    referenced = {name for expression in expressions for name in re.findall(r"(?<![\w.])inputs\.([\w-]+)", expression)}
    return referenced - declared


class ReleaseDryMutationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]
        cls.source_version = runpy.run_path(str(ROOT / "scripts/check_test_app_pins.py"))["read_release_version"]()

    def test_publish_expressions_reference_only_declared_dispatch_inputs(self) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
        events = workflow.get("on", workflow.get(True))
        assert len(events["workflow_dispatch"]["inputs"]) == 6
        assert undefined_dispatch_inputs(workflow) == set(), undefined_dispatch_inputs(workflow)

    def test_dispatch_input_check_rejects_an_undefined_property(self) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
        step = next(
            step
            for step in workflow["jobs"]["validate-versions"]["steps"]
            if step.get("name") == "Check test-app pins match the release being published"
        )
        step["env"]["RELEASE_REF"] = "${{ inputs.not_declared }}"
        assert undefined_dispatch_inputs(workflow) == {"not_declared"}

    def test_or_mutation_is_rejected_instead_of_mistaken_for_a_guard(self) -> None:
        unsafe = f"{INPUT_DRY} != true || {INPUT_DRY} != 'true'"
        safe = f"{INPUT_DRY} != true && {INPUT_DRY} != 'true'"
        for dry in [True, "true"]:
            with self.subTest(dry=dry):
                assert allows_mutation(unsafe, INPUT_DRY, dry)
                assert not allows_mutation(safe, INPUT_DRY, dry)

    def test_swift_mutation_blocks_both_true_forms_and_allows_real_release(self) -> None:
        steps = self.jobs["update-swift-package-manifest"]["steps"]
        commit = next(step for step in steps if step.get("name") == "Commit and push Package.swift")
        assert "||" not in commit.get("if", "")
        for dry, expected in [(True, False), ("true", False), (False, True), ("false", True)]:
            with self.subTest(dry=dry), tempfile.TemporaryDirectory() as directory:
                log = Path(directory) / "git.log"
                git = Path(directory) / "git"
                git.write_text('#!/bin/sh\nprintf "%s\\n" "$*" >> "$GIT_LOG"\n')
                git.chmod(0o755)
                if allows_mutation(commit.get("if", ""), OUTPUT_DRY, dry):
                    script = commit["run"].replace("${{ needs.prepare.outputs.version }}", "1.22.2")
                    script = script.replace("${{ needs.prepare.outputs.tag }}", "v1.22.2")
                    environment = dict(os.environ, PATH=f"{directory}:{os.environ['PATH']}", GIT_LOG=str(log))
                    subprocess.run(["bash", "-eu", "-c", script], env=environment, check=True)
                commands = log.read_text().splitlines() if log.exists() else []
                pushes = [command for command in commands if command.startswith("push ")]
                assert len(pushes) == (2 if expected else 0)
                if expected:
                    assert pushes[0].endswith("refs/tags/v1.22.2")
                    assert pushes[1].endswith("HEAD:refs/heads/release/swift/1.22.2")

    def test_swift_dry_run_keeps_checksum_validation_and_artifact_builds(self) -> None:
        job = self.jobs["update-swift-package-manifest"]
        assert "dry_run" not in job["if"]
        assert job["needs"] == ["prepare", "build-swift-artifactbundle", "e2e-swift"]
        validate = next(step for step in job["steps"] if step.get("name", "").startswith("Update Package.swift"))
        assert "if" not in validate
        assert '[[ -z "$CHECKSUM" ]]' in validate["run"]
        assert '[[ "$placeholders" -eq 0 ]]' in validate["run"]
        assert "/releases/download/v${VERSION}/" in validate["run"]

    def test_plugins_only_dispatch_blocks_both_true_forms(self) -> None:
        for name in ["dispatch-plugin-npm", "dispatch-plugin-hermes"]:
            assert "||" not in self.jobs[name]["if"]
            for dry, expected in [(True, False), ("true", False), (False, True), ("false", True)]:
                with self.subTest(job=name, dry=dry):
                    assert allows_mutation(self.jobs[name]["if"], INPUT_DRY, dry) == expected

    def test_contract_runs_in_the_existing_ci_job(self) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/ci.yaml").read_text())
        steps = workflow["jobs"]["nuget-pack-contract"]["steps"]
        assert sum(step.get("run") == "mise run contract-tests:release-dry-mutations" for step in steps) == 1
        assert any(step.get("uses", "").startswith("xberg-io/actions/setup-mise@") for step in steps)
        tasks = tomllib.loads((ROOT / "mise.toml").read_text())["tasks"]
        assert tasks["contract-tests:release-dry-mutations"]["run"] == (
            "uv run --no-sync python -m unittest scripts.ci.test_release_dry_mutations -v"
        )

    def run_test_app_pin_step(
        self,
        *,
        release: str,
        dry: str,
        stale: bool = False,
        tag: str = "",
        source_sha: str = "b111c8c371f57b3af1deb279eb107ca016ac5482",
    ) -> subprocess.CompletedProcess[str]:
        steps = self.jobs["validate-versions"]["steps"]
        step = next(
            item for item in steps if item.get("name") == "Check test-app pins match the release being published"
        )
        assert step["run"] == "mise run test-apps:check-release-pins"
        # Run the task's own command instead of mise, so the copy needs no mise project or network,
        # and pin that command here so the wiring stays covered. ~keep
        task = tomllib.loads((ROOT / "mise-tasks/test-apps.toml").read_text())["test-apps:check-release-pins"]
        command = task["run"]
        assert " ".join(command.split()) == (
            "python3 scripts/check_test_app_pins.py"
            ' --release "${RELEASE_VERSION:?RELEASE_VERSION is required}"'
            ' --dry-run "${RELEASE_DRY_RUN:?RELEASE_DRY_RUN is required}"'
            ' --source-sha "${RELEASE_SOURCE_SHA:?RELEASE_SOURCE_SHA is required}"'
            ' --ref "${RELEASE_REF:-}"'
            ' --tag "${RELEASE_TAG:-}"'
        ), command
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            checker = ROOT / "scripts/check_test_app_pins.py"
            pins = runpy.run_path(str(checker))["PINS"]
            paths = {"Cargo.toml", "alef.toml", *(pin.path for pin in pins)}
            assert len(paths) == 20
            for name in paths:
                destination = root / name
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(ROOT / name, destination)
            (root / "scripts").mkdir(exist_ok=True)
            shutil.copy2(ROOT / "scripts/check_test_app_pins.py", root / "scripts/check_test_app_pins.py")
            if stale:
                manifest = root / "test_apps/node/package.json"
                original = manifest.read_text()
                assert self.source_version != "0.0.1"
                source_pin = f'"{self.source_version}"'
                assert original.count(source_pin) == 1
                changed = original.replace(source_pin, '"0.0.1"')
                assert changed != original
                assert changed.count('"0.0.1"') == 1
                manifest.write_text(changed)
            values = {
                "needs.prepare.outputs.version": release,
                "needs.prepare.outputs.dry_run": dry,
                "github.sha": source_sha,
                "inputs.tag": tag,
            }

            def expand(text: str) -> str:
                for expression, value in values.items():
                    text = text.replace("${{ " + expression + " }}", value)
                assert "${{" not in text, text
                return text

            environment = dict(os.environ)
            environment.update({name: expand(str(value)) for name, value in step.get("env", {}).items()})
            environment["PATH"] = str(Path(sys.executable).parent) + os.pathsep + environment["PATH"]
            return subprocess.run(
                ["bash", "-eu", "-c", command],
                cwd=root,
                env=environment,
                capture_output=True,
                text=True,
                check=False,
            )

    def test_real_release_validates_real_source_pins(self) -> None:
        result = self.run_test_app_pin_step(release=self.source_version, dry="false")
        assert result.returncode == 0, result.stdout + result.stderr

    def test_virtual_dry_release_validates_real_source_pins(self) -> None:
        result = self.run_test_app_pin_step(release=f"{self.source_version}-dryrun-b111c8c", dry="true")
        assert result.returncode == 0, result.stdout + result.stderr

    def test_virtual_dry_release_still_rejects_a_stale_test_app_pin(self) -> None:
        result = self.run_test_app_pin_step(release=f"{self.source_version}-dryrun-b111c8c", dry="true", stale=True)
        assert result.returncode != 0
        output = result.stdout + result.stderr
        findings = [line.strip() for line in output.splitlines() if line.startswith("  ")]
        assert findings == [f"test_apps/node/package.json: pinned to 0.0.1, expected {self.source_version}"], output
        assert "Cargo.toml [workspace.package].version:" not in output

    def test_real_release_does_not_accept_a_synthetic_version(self) -> None:
        result = self.run_test_app_pin_step(release=f"{self.source_version}-dryrun-b111c8c", dry="false")
        assert result.returncode != 0
        assert "Cargo.toml" in result.stdout + result.stderr

    def test_virtual_dry_release_does_not_accept_wrong_source_or_sha(self) -> None:
        for version in ["0.0.1-dryrun-b111c8c", f"{self.source_version}-dryrun-0000000"]:
            with self.subTest(version=version):
                result = self.run_test_app_pin_step(release=version, dry="true")
                assert result.returncode != 0
                assert "Cargo.toml" in result.stdout + result.stderr

    def test_explicit_tag_does_not_normalize_virtual_version(self) -> None:
        result = self.run_test_app_pin_step(
            release=f"{self.source_version}-dryrun-b111c8c", dry="true", tag=f"v{self.source_version}"
        )
        assert result.returncode != 0
        assert "Cargo.toml [workspace.package].version:" in result.stdout + result.stderr

    def test_checker_explicit_ref_does_not_normalize_virtual_version(self) -> None:
        main = runpy.run_path(str(ROOT / "scripts/check_test_app_pins.py"))["main"]
        argv = [
            "checker",
            "--release",
            f"{self.source_version}-dryrun-b111c8c",
            "--dry-run",
            "true",
            "--source-sha",
            "b111c8c371f57b3af1deb279eb107ca016ac5482",
            "--ref",
            "main",
        ]
        output = io.StringIO()
        with patch.object(sys, "argv", argv), contextlib.redirect_stdout(output):
            result = main()
        assert result != 0
        assert "Cargo.toml [workspace.package].version:" in output.getvalue()

    def test_virtual_dry_release_requires_a_full_hex_source_sha(self) -> None:
        for sha in ["b111c8c", "b111c8c" + "x" * 33]:
            with self.subTest(sha=sha):
                result = self.run_test_app_pin_step(
                    release=f"{self.source_version}-dryrun-b111c8c", dry="true", source_sha=sha
                )
                assert result.returncode != 0
                assert "Cargo.toml [workspace.package].version:" in result.stdout + result.stderr

    def test_real_prerelease_version_is_preserved_exactly(self) -> None:
        checker = runpy.run_path(str(ROOT / "scripts/check_test_app_pins.py"))
        main = checker["main"]
        source = "1.22.2-rc.1-dryrun-b111c8c"
        with patch.dict(
            main.__globals__,
            {
                "read_release_version": lambda: source,
                "check_go_module": lambda **_: ([], False),
                "registry_pins": lambda *_, **__: ([], False),
                "PINS": (),
            },
        ):
            for released, expected in [(source, 0), ("1.22.2-rc.1", 1)]:
                with self.subTest(released=released), patch.object(sys, "argv", ["checker", "--release", released]):
                    output = io.StringIO()
                    with contextlib.redirect_stdout(output):
                        actual = main()
                    assert actual == expected, output.getvalue()
                    if expected:
                        assert f"{source}, expected {released}" in output.getvalue()
                    else:
                        assert f"({source})" in output.getvalue()

    def test_every_remote_shell_mutation_has_a_dry_guard(self) -> None:
        found = []
        for name, job in self.jobs.items():
            for step in job.get("steps", []):
                if not re.search("\\bgit push\\b|\\bgh (?:release create|workflow run)\\b", step.get("run", "")):
                    continue
                found.append((name, step["name"]))
                condition = f"{job.get('if', '')} {step.get('if', '')}"
                with self.subTest(job=name, step=step["name"]):
                    assert not allows_mutation(condition, OUTPUT_DRY, "true")
        assert len(found) == 8


if __name__ == "__main__":
    unittest.main()
