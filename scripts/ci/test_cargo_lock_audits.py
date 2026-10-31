"""Prove that the security gate examines every independent Cargo resolution."""

import copy
import fnmatch
import importlib
import json
import subprocess
from pathlib import Path
from types import ModuleType
from typing import Any

import pytest
import tomllib
import yaml

LOCKS = (
    "Cargo.lock",
    "e2e/rust/Cargo.lock",
    "packages/elixir/native/tree_sitter_language_pack_nif/Cargo.lock",
    "packages/ruby/ext/ts_pack_core_rb/native/Cargo.lock",
    "test_apps/rust/Cargo.lock",
)


Gate = tuple[ModuleType, Path, dict[str, Any], list[list[str]]]


@pytest.fixture
def gate(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> Gate:
    module = importlib.import_module("scripts.ci.check_cargo_lock_audits")
    for lock in LOCKS:
        path = tmp_path / lock
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('version = 4\n[[package]]\nname = "fixture"\nversion = "1.0.0"\n')
    calls: list[list[str]] = []
    report = {
        "lockfile": {"dependency-count": 1},
        "vulnerabilities": {"count": 0, "found": False, "list": []},
    }

    def run(args: list[str], **_kwargs: Any) -> subprocess.CompletedProcess[str]:
        calls.append(args)
        if args[:2] == ["git", "ls-files"]:
            return subprocess.CompletedProcess(args, 0, "\0".join(LOCKS) + "\0", "")
        return subprocess.CompletedProcess(args, 0, json.dumps(report), "")

    monkeypatch.setattr(module.subprocess, "run", run)
    return module, tmp_path, report, calls


def test_all_five_lockfiles_are_parsed_and_audited_exactly_once(gate: Gate) -> None:
    module, root, _, calls = gate
    rows = module.audit_all(root)
    assert [row["lock"] for row in rows] == list(LOCKS)
    assert [row["packages"] for row in rows] == [1] * 5
    assert [args[3] for args in calls if args[:2] == ["cargo", "audit"]] == list(LOCKS)


@pytest.mark.parametrize("problem", ["missing", "extra", "empty"])
def test_missing_extra_or_empty_lock_inventory_fails_before_any_audit(
    gate: Gate, problem: str, monkeypatch: pytest.MonkeyPatch
) -> None:
    module, root, _, calls = gate
    original = module.subprocess.run

    def run(args: list[str], **kwargs: Any) -> subprocess.CompletedProcess[str]:
        if args[:2] == ["git", "ls-files"]:
            locks = list(LOCKS)
            if problem == "missing":
                locks.pop()
            elif problem == "extra":
                locks.append("packages/extra/Cargo.lock")
            else:
                locks.clear()
            return subprocess.CompletedProcess(args, 0, "\0".join(locks), "")
        return original(args, **kwargs)

    monkeypatch.setattr(module.subprocess, "run", run)
    with pytest.raises(ValueError, match="lockfile inventory"):
        module.audit_all(root)
    assert not calls


@pytest.mark.parametrize("source", ["version = 4\n", "version = 4\npackage = []\n", "not TOML!"])
def test_empty_or_malformed_lock_never_produces_a_clean_audit(gate: Gate, source: str) -> None:
    module, root, _, calls = gate
    (root / LOCKS[0]).write_text(source)
    with pytest.raises(ValueError, match=r"nonzero|Expected|Invalid"):
        module.audit_all(root)
    assert len(calls) == 1


@pytest.mark.parametrize("count", [0, 2, True, "1", None])
def test_audited_dependency_count_must_match_actual_nonzero_lock_packages(gate: Gate, count: object) -> None:
    module, root, report, _ = gate
    report["lockfile"]["dependency-count"] = count
    with pytest.raises(ValueError, match="dependency count"):
        module.audit_all(root)


def test_advisory_mutation_fails_even_when_the_auditor_exits_zero(gate: Gate) -> None:
    module, root, report, _ = gate
    report["vulnerabilities"] = {"count": 1, "found": True, "list": [{"advisory": "fixture"}]}
    with pytest.raises(ValueError, match="vulnerabilities"):
        module.audit_all(root)


@pytest.mark.parametrize(("output", "exit_code"), [("not JSON", 0), ("{}", 0), ("[]", 0), (None, 1)])
def test_malformed_or_failed_audit_never_passes(
    gate: Gate, monkeypatch: pytest.MonkeyPatch, output: str | None, exit_code: int
) -> None:
    module, root, report, _ = gate
    original = module.subprocess.run

    def run(args: list[str], **kwargs: Any) -> subprocess.CompletedProcess[str]:
        if args[:2] == ["git", "ls-files"]:
            return original(args, **kwargs)
        return subprocess.CompletedProcess(args, exit_code, output or json.dumps(report), "fixture failure")

    monkeypatch.setattr(module.subprocess, "run", run)
    with pytest.raises(ValueError, match=r"Expecting|malformed|failed"):
        module.audit_all(root)


def assert_ci_wiring(workflow: dict) -> None:
    job = workflow["jobs"]["cargo-security"]
    assert "if" not in job
    assert "continue-on-error" not in job
    step = next(step for step in job["steps"] if step.get("run") == "mise run rust:audit")
    assert "if" not in step
    assert "continue-on-error" not in step
    contract = workflow["jobs"]["nuget-pack-contract"]["steps"]
    assert any("python -m pytest scripts/ci/test_cargo_lock_audits.py" in step.get("run", "") for step in contract)
    paths = workflow.get("on", workflow.get(True))["push"]["paths"]
    required = [
        *LOCKS,
        "scripts/ci/check_cargo_lock_audits.py",
        "scripts/ci/test_cargo_lock_audits.py",
        "mise-tasks/rust.toml",
    ]
    assert all(any(fnmatch.fnmatchcase(path, pattern) for pattern in paths) for path in required)


def test_ci_runs_the_audit_and_its_contract_without_conditions() -> None:
    root = Path(__file__).resolve().parents[2]
    workflow = yaml.safe_load((root / ".github/workflows/ci.yaml").read_text())
    assert_ci_wiring(workflow)


def test_auditor_is_exactly_pinned_and_locked_as_a_task_tool() -> None:
    root = Path(__file__).resolve().parents[2]
    tasks = tomllib.loads((root / "mise-tasks/rust.toml").read_text())
    assert tasks["rust:audit"]["tools"] == {"cargo:cargo-audit": "0.22.1"}
    locked = tomllib.loads((root / "mise.lock").read_text())["tools"]["cargo:cargo-audit"]
    assert len(locked) == 1
    assert locked[0]["version"] == "0.22.1"
    assert locked[0]["backend"] == "cargo:cargo-audit"


@pytest.mark.parametrize("mutation", ["missing-path", "missing-invocation", "conditional", "ignored-errors"])
def test_ci_contract_rejects_missing_coverage_or_a_skippable_audit(mutation: str) -> None:
    root = Path(__file__).resolve().parents[2]
    workflow = copy.deepcopy(yaml.safe_load((root / ".github/workflows/ci.yaml").read_text()))
    job = workflow["jobs"]["cargo-security"]
    if mutation == "missing-path":
        workflow.get("on", workflow.get(True))["push"]["paths"].remove("test_apps/rust/Cargo.lock")
    elif mutation == "missing-invocation":
        job["steps"][-1]["run"] = "echo skipped"
    elif mutation == "conditional":
        job["if"] = "false"
    else:
        job["continue-on-error"] = True
    with pytest.raises((AssertionError, StopIteration)):
        assert_ci_wiring(workflow)
