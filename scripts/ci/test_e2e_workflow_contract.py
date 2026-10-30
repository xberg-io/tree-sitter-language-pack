"""Pin expanded E2E job behavior across reusable-workflow boundaries."""

import copy
import fnmatch
import hashlib
import json
import re
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[2]
CONTRACT = Path(__file__).with_name("fixtures") / "e2e-workflow-contract.json"
REUSABLE_PATH = ".github/workflows/ci-e2e-*.yaml"
CI_CONTRACT_COMMAND = "uv run --no-sync python -m pytest scripts/ci/test_e2e_workflow_contract.py"
CI_CONTRACT_PATH = ".github/workflows/ci-e2e*.yaml"


def load_workflow(path: Path) -> dict:
    document = yaml.safe_load(path.read_text())
    if True in document:
        document["on"] = document.pop(True)
    return document


def digest(value: dict) -> str:
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def verify_ci_contract_gate(root: Path) -> None:
    workflow = load_workflow(root / ".github/workflows/ci.yaml")
    paths = workflow["on"]["push"]["paths"]
    expected = json.loads(CONTRACT.read_text())
    dependencies = [".github/workflows/ci-e2e.yaml"] + [
        f".github/workflows/ci-e2e-{job.removeprefix('test-')}.yaml" for job in expected["delegated_jobs"]
    ]
    for path in dependencies:
        assert any(fnmatch.fnmatchcase(path, pattern) for pattern in paths), f"CI does not cover {path}"
    job = workflow["jobs"]["nuget-pack-contract"]
    assert "if" not in job, "CI contract job must be unconditional"
    assert not job.get("continue-on-error", False), "CI contract job must propagate failures"
    steps = [step for step in job["steps"] if step.get("name") == "Test NuGet packing workflow"]
    assert len(steps) == 1, "CI contract step count changed"
    assert "if" not in steps[0], "CI contract step must be unconditional"
    assert not steps[0].get("continue-on-error", False), "CI contract step must propagate failures"
    assert CI_CONTRACT_COMMAND in steps[0]["run"].splitlines(), "CI contract invocation missing"


def expand_jobs(root: Path, workflow: dict, delegated: list[str]) -> dict:
    jobs = copy.deepcopy(workflow["jobs"])
    actual_calls = {name for name, job in jobs.items() if "uses" in job}
    assert actual_calls == set(delegated), "delegated job set changed"
    for name in actual_calls:
        caller = jobs[name]
        assert set(caller) == {"if", "needs", "uses"}, f"{name} caller boundary changed"
        expected_path = f".github/workflows/ci-e2e-{name.removeprefix('test-')}.yaml"
        assert caller["uses"] == f"./{expected_path}", f"{name} reusable path changed"
        called = load_workflow(root / expected_path)
        assert set(called) == {"name", "on", "permissions", "env", "jobs"}, f"{name} workflow boundary changed"
        assert called["on"] == {"workflow_call": None}, f"{name} call inputs/secrets/outputs changed"
        assert called["permissions"] == workflow["permissions"], f"{name} token permissions changed"
        assert called["env"] == workflow["env"], f"{name} global environment changed"
        assert set(called["jobs"]) == {name}, f"{name} child job set changed"
        inner = called["jobs"][name]
        assert not ({"if", "needs", "uses"} & inner.keys()), f"{name} child controls changed"
        assert not re.search(r"\b(?:needs|secrets|inputs)\.", json.dumps(inner)), f"{name} needs an explicit input"
        jobs[name] = inner | {key: caller[key] for key in ("if", "needs")}
    return jobs


def verify_contract(root: Path) -> dict:
    expected = json.loads(CONTRACT.read_text())
    workflow = load_workflow(root / ".github/workflows/ci-e2e.yaml")
    assert set(workflow["jobs"]) == set(expected["jobs"]), "E2E job set changed"
    header = {key: value for key, value in workflow.items() if key != "jobs"}
    paths = header["on"]["push"]["paths"]
    assert paths.count(REUSABLE_PATH) == 1, "reusable workflow changes must trigger E2E"
    paths.remove(REUSABLE_PATH)
    assert header == expected["header"], "E2E workflow triggers or global controls changed"
    jobs = expand_jobs(root, workflow, expected["delegated_jobs"])
    filter_steps = [step for step in jobs["changes"]["steps"] if step.get("id") == "filter"]
    assert len(filter_steps) == 1, "change detection step count changed"
    filters = filter_steps[0]["with"]["filters"]
    assert yaml.safe_load(filters)["core"].count(REUSABLE_PATH) == 1, "child workflows must select core producers"
    filter_steps[0]["with"]["filters"] = filters.replace(f"  - '{REUSABLE_PATH}'\n", "", 1)
    for name, job in jobs.items():
        assert digest(job) == expected["jobs"][name], f"{name} expanded job contract changed"
    steps = sum(len(job.get("steps", [])) for job in jobs.values())
    assert steps == expected["step_count"], "E2E step count changed"
    return {"jobs": len(jobs), "steps": steps, "reusable_workflows": len(expected["delegated_jobs"])}


@pytest.fixture
def copied_workflows(tmp_path: Path) -> Path:
    directory = tmp_path / ".github/workflows"
    directory.mkdir(parents=True)
    for path in [ROOT / ".github/workflows/ci.yaml", *(ROOT / ".github/workflows").glob("ci-e2e*.yaml")]:
        (directory / path.name).write_bytes(path.read_bytes())
    return tmp_path


def write_workflow(root: Path, name: str, document: dict) -> None:
    (root / ".github/workflows" / name).write_text(yaml.safe_dump(document, sort_keys=False))


def test_expanded_e2e_jobs_preserve_original_contract() -> None:
    assert verify_contract(ROOT) == {"jobs": 21, "steps": 179, "reusable_workflows": 7}
    for path in (ROOT / ".github/workflows").glob("ci-e2e*.yaml"):
        assert len(path.read_text().splitlines()) <= 1000, f"{path.name} exceeds the file-size limit"


def test_rejects_a_dropped_job(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci-e2e.yaml")
    del workflow["jobs"]["test-go"]
    write_workflow(copied_workflows, "ci-e2e.yaml", workflow)
    with pytest.raises(AssertionError, match="E2E job set changed"):
        verify_contract(copied_workflows)


def test_rejects_a_dropped_artifact_producer_dependency(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci-e2e.yaml")
    workflow["jobs"]["test-go"]["needs"].remove("build-e2e-bundles")
    write_workflow(copied_workflows, "ci-e2e.yaml", workflow)
    with pytest.raises(AssertionError, match="test-go expanded job contract changed"):
        verify_contract(copied_workflows)


def test_rejects_a_dropped_artifact_download(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci-e2e-java.yaml")
    steps = workflow["jobs"]["test-java"]["steps"]
    steps.remove(next(step for step in steps if step.get("name") == "Download FFI artifacts"))
    write_workflow(copied_workflows, "ci-e2e-java.yaml", workflow)
    with pytest.raises(AssertionError, match="test-java expanded job contract changed"):
        verify_contract(copied_workflows)


def test_rejects_a_missing_global_environment_value(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci-e2e-java.yaml")
    del workflow["env"]["BUILD_PROFILE"]
    write_workflow(copied_workflows, "ci-e2e-java.yaml", workflow)
    with pytest.raises(AssertionError, match="test-java global environment changed"):
        verify_contract(copied_workflows)


def test_rejects_widened_token_permissions(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci-e2e-java.yaml")
    workflow["permissions"]["contents"] = "write"
    write_workflow(copied_workflows, "ci-e2e-java.yaml", workflow)
    with pytest.raises(AssertionError, match="test-java token permissions changed"):
        verify_contract(copied_workflows)


def test_rejects_a_changed_runner_matrix(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci-e2e-csharp.yaml")
    workflow["jobs"]["test-csharp"]["strategy"]["matrix"]["os"].remove("macos-26")
    write_workflow(copied_workflows, "ci-e2e-csharp.yaml", workflow)
    with pytest.raises(AssertionError, match="test-csharp expanded job contract changed"):
        verify_contract(copied_workflows)


def test_rejects_missing_child_workflow_trigger_coverage(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci-e2e.yaml")
    workflow["on"]["push"]["paths"].remove(REUSABLE_PATH)
    write_workflow(copied_workflows, "ci-e2e.yaml", workflow)
    with pytest.raises(AssertionError, match="reusable workflow changes must trigger E2E"):
        verify_contract(copied_workflows)


def test_child_only_changes_select_the_producer_and_leaf_checks() -> None:
    workflow = load_workflow(ROOT / ".github/workflows/ci-e2e.yaml")
    filters = next(step for step in workflow["jobs"]["changes"]["steps"] if step.get("id") == "filter")
    core_patterns = yaml.safe_load(filters["with"]["filters"])["core"]
    expected = json.loads(CONTRACT.read_text())
    for job in expected["delegated_jobs"]:
        path = f".github/workflows/ci-e2e-{job.removeprefix('test-')}.yaml"
        assert any(fnmatch.fnmatchcase(path, pattern) for pattern in core_patterns), path
    for job in ["build-ffi", *expected["delegated_jobs"]]:
        assert "needs.changes.outputs.core == 'true'" in workflow["jobs"][job]["if"], job


def test_rejects_missing_child_workflow_core_filter(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci-e2e.yaml")
    filters = next(step for step in workflow["jobs"]["changes"]["steps"] if step.get("id") == "filter")
    filters["with"]["filters"] = filters["with"]["filters"].replace(f"  - '{REUSABLE_PATH}'\n", "", 1)
    write_workflow(copied_workflows, "ci-e2e.yaml", workflow)
    with pytest.raises(AssertionError, match="child workflows must select core producers"):
        verify_contract(copied_workflows)


def test_real_ci_gate_covers_parent_and_children_and_runs_contract() -> None:
    verify_ci_contract_gate(ROOT)


def test_rejects_removed_real_ci_workflow_path_coverage(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci.yaml")
    workflow["on"]["push"]["paths"].remove(CI_CONTRACT_PATH)
    write_workflow(copied_workflows, "ci.yaml", workflow)
    with pytest.raises(AssertionError, match="CI does not cover"):
        verify_ci_contract_gate(copied_workflows)


def test_rejects_removed_real_ci_contract_invocation(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci.yaml")
    step = next(
        step
        for step in workflow["jobs"]["nuget-pack-contract"]["steps"]
        if step.get("name") == "Test NuGet packing workflow"
    )
    step["run"] = "\n".join(line for line in step["run"].splitlines() if line != CI_CONTRACT_COMMAND) + "\n"
    write_workflow(copied_workflows, "ci.yaml", workflow)
    with pytest.raises(AssertionError, match="CI contract invocation missing"):
        verify_ci_contract_gate(copied_workflows)


def test_rejects_a_conditional_real_ci_contract_job(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci.yaml")
    workflow["jobs"]["nuget-pack-contract"]["if"] = False
    write_workflow(copied_workflows, "ci.yaml", workflow)
    with pytest.raises(AssertionError, match="CI contract job must be unconditional"):
        verify_ci_contract_gate(copied_workflows)


def test_rejects_ignored_real_ci_contract_failures(copied_workflows: Path) -> None:
    workflow = load_workflow(copied_workflows / ".github/workflows/ci.yaml")
    step = next(
        step
        for step in workflow["jobs"]["nuget-pack-contract"]["steps"]
        if step.get("name") == "Test NuGet packing workflow"
    )
    step["continue-on-error"] = True
    write_workflow(copied_workflows, "ci.yaml", workflow)
    with pytest.raises(AssertionError, match="CI contract step must propagate failures"):
        verify_ci_contract_gate(copied_workflows)
