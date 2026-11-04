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
import tomllib
import unittest
from pathlib import Path
from unittest.mock import patch

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


REF_REWRITE = re.compile(
    r"git tag\s+(?:-f|--force)\b|git push\b[^\n]*\s(?:-f|--force(?:-with-lease)?)\b|git push\b[^\n]*\s\+"
    r"|sed\b[^\n]*__ALEF_SWIFT_CHECKSUM__"
)


def ref_rewrites(script: str) -> list[str]:
    return REF_REWRITE.findall(script)


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
        assert len(found) == 7


class SwiftReleaseWorkflowTests(unittest.TestCase):
    """Swift releases keep the core tag immutable (#215)."""

    @classmethod
    def setUpClass(cls) -> None:
        cls.jobs = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())["jobs"]

    def test_ref_rewrite_detector_catches_the_retired_swift_retagging(self) -> None:
        retired = "\n".join(
            [
                'sed -i "s/__ALEF_SWIFT_CHECKSUM__/${CHECKSUM}/g" Package.swift',
                'git tag -f "v1.22.2"',
                'git push origin -f "refs/tags/v1.22.2"',
                'git push --force origin "HEAD:refs/heads/release/swift/1.22.2"',
                "git push origin +HEAD:refs/heads/main",
            ]
        )
        assert len(ref_rewrites(retired)) == 5
        assert ref_rewrites('git push --atomic origin HEAD:refs/heads/main "refs/tags/${TAG}"') == []

    def test_release_workflows_never_move_a_tag_or_force_push(self) -> None:
        workflows = [".github/workflows/publish.yaml", ".github/workflows/prepare-swift-release.yaml"]
        scanned = 0
        for path in workflows:
            for name, job in yaml.safe_load((ROOT / path).read_text())["jobs"].items():
                for step in job.get("steps", []):
                    scanned += "run" in step
                    with self.subTest(workflow=path, job=name, step=step.get("name")):
                        assert ref_rewrites(step.get("run", "")) == []
        assert scanned > 100

    def test_publish_builds_no_swift_bundle(self) -> None:
        builders = [
            name
            for name, job in self.jobs.items()
            for step in job.get("steps", [])
            if step.get("uses", "").startswith("xberg-io/actions/build-swift-artifactbundle@")
        ]
        assert builders == []
        assert "build-swift-artifactbundle" not in self.jobs

    def test_swift_release_is_prepared_by_the_shared_workflow(self) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/prepare-swift-release.yaml").read_text())
        events = workflow.get("on", workflow.get(True))
        assert set(events) == {"workflow_dispatch"}
        assert events["workflow_dispatch"]["inputs"]["dry-run"]["default"] is False
        jobs = workflow["jobs"]
        assert set(jobs) == {"clone-vendors-shard", "clone-vendors", "swift"}
        swift = jobs["swift"]
        assert swift["uses"] == "xberg-io/actions/.github/workflows/reusable-swift-release-prepare.yml@v1"
        assert swift["needs"] == "clone-vendors"
        assert swift["with"] == {
            "dry-run": "${{ inputs.dry-run }}",
            "crate-name": "tree-sitter-language-pack-swift",
            "artifact-name": "TreeSitterLanguagePack-rs",
            "binary-target-name": "RustBridgeBinary",
            "setup-artifact": "language-parsers",
            "setup-artifact-path": "parsers",
        }
        assert swift["secrets"] == {
            "app-id": "${{ secrets.BOT_APP_ID }}",
            "app-private-key": "${{ secrets.BOT_APP_PRIVATE_KEY }}",
        }
        assert jobs["clone-vendors"]["needs"] == "clone-vendors-shard"
        upload = jobs["clone-vendors"]["steps"][-1]["with"]
        assert (upload["name"], upload["path"]) == ("language-parsers", "parsers")
        checkout = jobs["clone-vendors-shard"]["steps"][0]
        assert checkout["with"]["ref"] == "${{ github.sha }}"
        ignored = subprocess.run(["git", "check-ignore", "-q", "parsers/x/src/parser.c"], cwd=ROOT, check=False)
        assert ignored.returncode == 0

    def test_vendor_jobs_match_publish(self) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/prepare-swift-release.yaml").read_text())
        for name in ["clone-vendors-shard", "clone-vendors"]:
            with self.subTest(job=name):
                published = {key: value for key, value in self.jobs[name].items() if key not in {"if", "needs"}}
                prepared = {key: value for key, value in workflow["jobs"][name].items() if key != "needs"}
                text = yaml.safe_dump(published).replace("needs.prepare.outputs.checkout_ref", "github.sha")
                assert yaml.safe_load(text) == prepared

    def test_swift_publication_verifies_the_immutable_tag(self) -> None:
        assert "update-swift-package-manifest" not in self.jobs
        job = self.jobs["publish-swift-release"]
        assert job["needs"] == ["prepare", "validate-versions", "e2e-swift"]
        assert "||" not in job["if"]
        # ~keep Job outputs are strings, so only the string forms can occur; a boolean comparison is dead.
        assert f"{OUTPUT_DRY} != true " not in job["if"]
        for dry, expected in [("true", False), ("false", True)]:
            with self.subTest(dry=dry):
                assert allows_mutation(job["if"], OUTPUT_DRY, dry) == expected
        checkout = next(step for step in job["steps"] if step.get("uses", "").startswith("actions/checkout@"))
        assert checkout["with"]["ref"] == "${{ needs.prepare.outputs.tag }}"
        assert checkout["with"]["fetch-depth"] == 0
        assert checkout["with"]["token"] == "${{ steps.app-token.outputs.token }}"
        publish = [step for step in job["steps"] if step.get("uses", "").startswith("xberg-io/actions/publish-swift-")]
        assert [step["uses"] for step in publish] == ["xberg-io/actions/publish-swift-release@v1"]
        assert publish[0]["with"]["tag"] == "${{ needs.prepare.outputs.tag }}"
        assert publish[0]["with"]["bundle-zip"] == ""
        assert "dry-run" not in publish[0]["with"]
        assert (
            publish[0]["with"]["receipt"]
            == "dist/swift-artifactbundle/TreeSitterLanguagePack-rs.artifactbundle.release.json"
        )
        for name in ["finalize-github-release", "release-finalize"]:
            with self.subTest(job=name):
                assert "publish-swift-release" in self.jobs[name]["needs"]
                env = next(step for step in self.jobs[name]["steps"] if "RESULT_SWIFT_RELEASE" in step.get("env", {}))
                assert env["env"]["RESULT_SWIFT_RELEASE"] == "${{ needs.publish-swift-release.result }}"
                assert '"swift-release" "$RESULT_SWIFT_RELEASE" "$RELEASE_SWIFT"' in env["run"]

    def swift_gate(self) -> dict:
        steps = self.jobs["validate-versions"]["steps"]
        gates = [step for step in steps if step.get("uses", "").startswith("xberg-io/actions/publish-swift-release@")]
        assert len(gates) == 1
        return gates[0]

    def test_tagged_swift_release_is_verified_before_any_registry_publishes(self) -> None:
        gate = self.swift_gate()
        assert gate["uses"] == "xberg-io/actions/publish-swift-release@v1"
        assert gate["with"]["dry-run"] == "true"
        assert gate["with"]["bundle-zip"] == ""
        assert gate["with"]["tag"] == "${{ needs.prepare.outputs.tag }}"
        assert gate["with"]["receipt"] == (
            "dist/swift-artifactbundle/TreeSitterLanguagePack-rs.artifactbundle.release.json"
        )
        condition = gate["if"]
        assert "needs.prepare.outputs.release_swift == 'true'" in condition
        for dry, expected in [("true", False), ("false", True)]:
            with self.subTest(dry=dry):
                assert allows_mutation(condition, OUTPUT_DRY, dry) == expected
        steps = self.jobs["validate-versions"]["steps"]
        checkout = next(step for step in steps if step.get("uses", "").startswith("actions/checkout@"))
        assert checkout["with"]["fetch-depth"] == 0
        assert checkout["with"]["ref"] == "${{ needs.prepare.outputs.checkout_ref }}"

    def gated(self, name: str, seen: frozenset[str] = frozenset()) -> bool:
        """Whether a failed validate-versions keeps `name` from running."""
        job = self.jobs[name]
        needs = job.get("needs", [])
        needs = [needs] if isinstance(needs, str) else needs
        condition = str(job.get("if", ""))
        overrides_failure = any(word in condition for word in ["always()", "!cancelled()", "failure()"])
        if self.fails_without_validation(job):
            return True
        for need in needs:
            if need in seen:
                continue
            upstream = need == "validate-versions" or self.gated(need, seen | {name})
            required = (
                not overrides_failure
                or f"needs.{need}.result == 'success'" in condition
                or f"needs.{need}.outputs.promoted == 'true'" in condition
            )
            if upstream and required:
                return True
        return False

    @staticmethod
    def fails_without_validation(job: dict) -> bool:
        """A job that runs regardless but fails its own step when validate-versions did not succeed."""
        for step in job.get("steps", []):
            names = [key for key, value in step.get("env", {}).items() if value == "${{ needs.validate-versions.result }}"]
            if any(f'require_success "validate-versions" "${name}"' in step.get("run", "") for name in names):
                return step.get("if", "always()") in {"always()", f"always() && {OUTPUT_DRY} != 'true'"}
        return False

    def test_gate_walk_detects_a_job_that_ignores_validation(self) -> None:
        original = self.jobs.get("publish-zig")
        self.jobs["publish-zig"] = {"needs": ["validate-versions"], "if": "${{ always() }}"}
        try:
            assert not self.gated("publish-zig")
        finally:
            self.jobs["publish-zig"] = original

    def test_every_registry_publish_needs_the_swift_gate(self) -> None:
        registry = [
            name
            for name in self.jobs
            if name.startswith(("publish-", "trigger-", "upload-", "stage-"))
        ]
        assert len(registry) > 20, registry
        ungated = [name for name in registry if not self.gated(name)]
        assert ungated == []

    def test_every_needs_edge_names_an_existing_job(self) -> None:
        edges = 0
        for name, job in self.jobs.items():
            needs = job.get("needs", [])
            for target in [needs] if isinstance(needs, str) else needs:
                edges += 1
                with self.subTest(job=name, needs=target):
                    assert target in self.jobs
        assert edges > 100


if __name__ == "__main__":
    unittest.main()
