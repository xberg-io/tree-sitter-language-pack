"""Keep release dry runs read-only while retaining artifact validation."""

import contextlib
import hashlib
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
        workflows = [".github/workflows/publish.yaml", ".github/workflows/release-prepare.yaml"]
        scanned = 0
        for path in workflows:
            for name, job in yaml.safe_load((ROOT / path).read_text())["jobs"].items():
                for step in job.get("steps", []):
                    scanned += "run" in step
                    with self.subTest(workflow=path, job=name, step=step.get("name")):
                        assert ref_rewrites(step.get("run", "")) == []
        assert scanned > 100

    def test_swift_bundle_is_built_once_by_the_dry_run_with_a_receipt(self) -> None:
        job = self.jobs["build-swift-artifactbundle"]
        for dry, expected in [(True, False), ("true", True), (False, False), ("false", False)]:
            with self.subTest(dry=dry):
                assert (f"{OUTPUT_DRY} == 'true'" in job["if"] and dry == "true") == expected
        bundle = next(step for step in job["steps"] if step.get("id") == "bundle")
        assert bundle["with"]["artifact-name"] == "TreeSitterLanguagePack-rs"
        assert bundle["with"]["receipt-version"] == "${{ steps.version.outputs.version }}"
        upload = next(step for step in job["steps"] if step.get("with", {}).get("name") == "swift-artifactbundle")
        assert upload["with"]["path"].splitlines() == [
            "${{ steps.bundle.outputs.bundle-zip }}",
            "${{ steps.bundle.outputs.receipt }}",
        ]
        assert not any(step.get("with", {}).get("name") == "swift-checksum" for step in job["steps"])

    def test_swift_publication_verifies_the_immutable_tag(self) -> None:
        assert "update-swift-package-manifest" not in self.jobs
        job = self.jobs["publish-swift-release"]
        assert job["needs"] == ["prepare", "e2e-swift"]
        assert "||" not in job["if"]
        for dry, expected in [(True, False), ("true", False), (False, True), ("false", True)]:
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

    def test_every_needs_edge_names_an_existing_job(self) -> None:
        edges = 0
        for name, job in self.jobs.items():
            needs = job.get("needs", [])
            for target in [needs] if isinstance(needs, str) else needs:
                edges += 1
                with self.subTest(job=name, needs=target):
                    assert target in self.jobs
        assert edges > 100

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


BUNDLE = "dist/swift-artifactbundle/TreeSitterLanguagePack-rs.artifactbundle.zip"
REPOSITORY = "xberg-io/tree-sitter-language-pack"
MANIFEST = """let package = Package(
  targets: [
    .binaryTarget(
      name: "RustBridgeBinary",
      url: "https://github.com/{repository}/releases/download/{tag}/TreeSitterLanguagePack-rs.artifactbundle.zip",
      checksum: "__ALEF_SWIFT_CHECKSUM__"
    ),
  ]
)
"""


def git(cwd: Path, *arguments: str) -> str:
    environment = dict(
        os.environ,
        GIT_CONFIG_GLOBAL=os.devnull,
        GIT_AUTHOR_NAME="t",
        GIT_AUTHOR_EMAIL="t@example.com",
        GIT_COMMITTER_NAME="t",
        GIT_COMMITTER_EMAIL="t@example.com",
    )
    result = subprocess.run(["git", *arguments], cwd=cwd, env=environment, capture_output=True, text=True, check=True)
    return result.stdout.strip()


class ReleasePreparationTests(unittest.TestCase):
    """Run the preparation workflow's own steps against a disposable remote."""

    @classmethod
    def setUpClass(cls) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/release-prepare.yaml").read_text())
        cls.workflow = workflow
        cls.steps = {step.get("name"): step for step in workflow["jobs"]["prepare"]["steps"]}

    def setUp(self) -> None:
        self.directory = tempfile.TemporaryDirectory()
        self.root = Path(self.directory.name)
        self.origin = self.root / "origin.git"
        git(self.root, "init", "--bare", "--initial-branch=main", str(self.origin))
        self.work = self.root / "work"
        git(self.root, "clone", "--quiet", str(self.origin), str(self.work))
        (self.work / ".gitignore").write_text("dist/\n")
        (self.work / "Package.swift").write_text(MANIFEST.format(repository=REPOSITORY, tag="v1.2.3"))
        git(self.work, "add", ".")
        git(self.work, "commit", "--quiet", "-m", "chore(release): 1.2.3")
        git(self.work, "push", "--quiet", "origin", "HEAD:refs/heads/main")
        self.source = git(self.work, "rev-parse", "HEAD")
        bundle = self.work / BUNDLE
        bundle.parent.mkdir(parents=True)
        bundle.write_bytes(b"exact bundle bytes")
        self.digest = hashlib.sha256(bundle.read_bytes()).hexdigest()
        receipt = bundle.with_name(bundle.name.removesuffix(".zip") + ".release.json")
        receipt.write_text(f'{{"sha256": "{self.digest}"}}\n')

    def tearDown(self) -> None:
        self.directory.cleanup()

    def run_step(self, name: str, cwd: Path | None = None) -> tuple[subprocess.CompletedProcess[str], dict[str, str]]:
        step = self.steps[name]
        outputs = self.root / "outputs"
        outputs.write_text("")
        supplied = {"TAG": "v1.2.3", "SOURCE_SHA": self.source, "REPOSITORY": REPOSITORY}
        assert set(step.get("env", {})) <= set(supplied), step.get("env")
        environment = dict(os.environ, GITHUB_OUTPUT=str(outputs), GIT_CONFIG_GLOBAL=os.devnull, **supplied)
        environment["BUNDLE"] = BUNDLE
        result = subprocess.run(
            ["bash", "-c", step["run"]],
            cwd=cwd or self.work,
            env=environment,
            capture_output=True,
            text=True,
            check=False,
        )
        values = dict(line.split("=", 1) for line in outputs.read_text().splitlines() if "=" in line)
        return result, values

    def pin(self, checksum: str | None = None) -> None:
        manifest = self.work / "Package.swift"
        manifest.write_text(manifest.read_text().replace("__ALEF_SWIFT_CHECKSUM__", checksum or self.digest))

    def remote_ref(self, ref: str) -> str:
        return git(self.origin, "rev-parse", ref)

    def test_job_reads_the_bundle_path_it_was_tested_with(self) -> None:
        assert self.workflow["jobs"]["prepare"]["env"]["BUNDLE"] == BUNDLE

    def test_first_preparation_tags_the_pinned_child_of_the_dry_run_commit(self) -> None:
        result, values = self.run_step("Detect an earlier preparation")
        assert result.returncode == 0, result.stderr
        assert values == {"existing": "false"}
        self.pin()
        result, _ = self.run_step("Verify the pinned manifest")
        assert result.returncode == 0, result.stderr
        result, _ = self.run_step("Commit, tag and push")
        assert result.returncode == 0, result.stderr
        tagged = self.remote_ref("refs/tags/v1.2.3^{commit}")
        assert self.remote_ref("refs/heads/main") == tagged
        assert self.remote_ref(f"{tagged}^") == self.source
        assert f'checksum: "{self.digest}"' in git(self.origin, "show", f"{tagged}:Package.swift")
        changed = git(self.origin, "diff", "--name-only", self.source, tagged)
        assert changed == "Package.swift"

    def test_rerun_after_tagging_keeps_the_tag_object_and_pushes_nothing(self) -> None:
        self.pin()
        for name in ["Verify the pinned manifest", "Commit, tag and push"]:
            result, _ = self.run_step(name)
            assert result.returncode == 0, result.stderr
        tag_object = self.remote_ref("refs/tags/v1.2.3")
        main = self.remote_ref("refs/heads/main")
        rerun = self.root / "rerun"
        git(self.root, "clone", "--quiet", str(self.origin), str(rerun))
        git(rerun, "checkout", "--quiet", self.source)
        result, values = self.run_step("Detect an earlier preparation", cwd=rerun)
        assert result.returncode == 0, result.stderr
        assert values == {"existing": "true"}
        for name in ["Pin the bundle checksum", "Verify the pinned manifest", "Commit, tag and push"]:
            assert self.steps[name]["if"] == "steps.earlier.outputs.existing != 'true'"
        assert self.remote_ref("refs/tags/v1.2.3") == tag_object
        assert self.remote_ref("refs/heads/main") == main

    def test_existing_tag_on_an_unrelated_commit_is_rejected(self) -> None:
        git(self.work, "commit", "--quiet", "--allow-empty", "-m", "unrelated")
        git(self.work, "push", "--quiet", "origin", "HEAD:refs/tags/v1.2.3")
        git(self.work, "reset", "--quiet", "--hard", self.source)
        result, values = self.run_step("Detect an earlier preparation")
        assert result.returncode != 0
        assert "existing" not in values
        assert "does not pin" in result.stdout + result.stderr

    def test_main_that_moved_past_the_dry_run_is_rejected(self) -> None:
        git(self.work, "commit", "--quiet", "--allow-empty", "-m", "later")
        git(self.work, "push", "--quiet", "origin", "HEAD:refs/heads/main")
        git(self.work, "reset", "--quiet", "--hard", self.source)
        result, values = self.run_step("Detect an earlier preparation")
        assert result.returncode != 0
        assert "existing" not in values
        assert "main moved" in result.stdout + result.stderr

    def test_manifest_gate_rejects_a_wrong_checksum_url_or_extra_edit(self) -> None:
        cases = {
            "wrong checksum": lambda: self.pin("0" * 64),
            "placeholder kept": lambda: None,
            "wrong url": lambda: (self.pin(), self.rewrite("/download/v1.2.3/", "/download/v1.2.2/")),
            "extra edit": lambda: (self.pin(), self.rewrite("Package(", 'Package(\n  name: "x",')),
        }
        for label, mutate in cases.items():
            with self.subTest(label):
                git(self.work, "checkout", "--quiet", "--", "Package.swift")
                mutate()
                result, _ = self.run_step("Verify the pinned manifest")
                assert result.returncode != 0, label
        assert git(self.origin, "tag", "--list") == ""

    def rewrite(self, old: str, new: str) -> None:
        manifest = self.work / "Package.swift"
        manifest.write_text(manifest.read_text().replace(old, new))

    def test_receipt_digest_must_match_the_bundle(self) -> None:
        self.pin()
        (self.work / BUNDLE).write_bytes(b"rebuilt bundle bytes")
        result, _ = self.run_step("Verify the pinned manifest")
        assert result.returncode != 0


if __name__ == "__main__":
    unittest.main()
