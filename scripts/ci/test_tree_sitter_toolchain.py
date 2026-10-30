"""Keep grammar generation on the canonical mise CLI despite stale global executables."""

import copy
import fnmatch
import os
import subprocess
from pathlib import Path

import pytest
import yaml

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = {
    "ci-cli.yaml": "clone-vendors",
    "ci-docker.yaml": "build-and-test",
    "ci-e2e.yaml": "clone-vendors",
    "ci-rust.yaml": "clone-vendors",
    "ci-sanitize.yaml": "sanitize-scanners",
    "ci-swift.yaml": "clone-vendors",
    "ci-zig.yaml": "clone-vendors",
    "ci.yaml": "clone-vendors",
    "publish-docker.yaml": "build",
    "publish.yaml": "clone-vendors-shard",
}
CI_DEPENDENCIES = [f".github/workflows/{name}" for name in WORKFLOWS] + [
    "scripts/stage_wasm_types.sh",
    "scripts/ci/test_tree_sitter_toolchain.py",
]
CI_COMMAND = "uv run --no-sync python -m pytest scripts/ci/test_tree_sitter_toolchain.py"


def verify_ci_gate(workflow: dict) -> None:
    paths = workflow.get("on", workflow.get(True))["push"]["paths"]
    for path in CI_DEPENDENCIES:
        assert any(fnmatch.fnmatchcase(path, pattern) for pattern in paths), f"CI does not cover {path}"
    job = workflow["jobs"]["nuget-pack-contract"]
    assert not ({"if", "needs"} & job.keys()), "CLI contract job must run unconditionally"
    assert not job.get("continue-on-error", False), "CLI contract job must propagate failures"
    steps = [step for step in job["steps"] if step.get("name") == "Test NuGet packing workflow"]
    assert len(steps) == 1, "CLI contract step count changed"
    assert "if" not in steps[0], "CLI contract step must run unconditionally"
    assert not steps[0].get("continue-on-error", False), "CLI contract step must propagate failures"
    assert CI_COMMAND in steps[0]["run"].splitlines(), "CLI contract invocation missing"


def test_actual_ci_gate_selects_every_workflow_and_helper_and_invokes_contract() -> None:
    assert len(CI_DEPENDENCIES) == 12
    verify_ci_gate(yaml.safe_load((ROOT / ".github/workflows/ci.yaml").read_text()))


@pytest.mark.parametrize("path", CI_DEPENDENCIES)
def test_ci_gate_rejects_removed_dependency_coverage(path: str) -> None:
    workflow = yaml.safe_load((ROOT / ".github/workflows/ci.yaml").read_text())
    verify_ci_gate(workflow)
    changed = copy.deepcopy(workflow)
    paths = changed[True]["push"]["paths"]
    changed[True]["push"]["paths"] = [pattern for pattern in paths if not fnmatch.fnmatchcase(path, pattern)]
    with pytest.raises(AssertionError, match="CI does not cover"):
        verify_ci_gate(changed)


def test_ci_gate_rejects_removed_contract_invocation() -> None:
    workflow = yaml.safe_load((ROOT / ".github/workflows/ci.yaml").read_text())
    verify_ci_gate(workflow)
    steps = workflow["jobs"]["nuget-pack-contract"]["steps"]
    step = next(step for step in steps if step.get("name") == "Test NuGet packing workflow")
    step["run"] = "\n".join(line for line in step["run"].splitlines() if line != CI_COMMAND) + "\n"
    with pytest.raises(AssertionError, match="CLI contract invocation missing"):
        verify_ci_gate(workflow)


def workflow_steps(filename: str) -> list[dict]:
    workflow = yaml.safe_load((ROOT / ".github/workflows" / filename).read_text())
    return workflow["jobs"][WORKFLOWS[filename]]["steps"]


@pytest.mark.parametrize("filename", WORKFLOWS)
def test_generator_jobs_install_the_manifest_cli(filename: str) -> None:
    steps = workflow_steps(filename)
    setup = next(step for step in steps if step.get("uses", "").startswith("xberg-io/actions/setup-mise@"))
    assert "tree-sitter" in setup["with"]["install-args"].split(), filename


@pytest.mark.parametrize("filename", WORKFLOWS)
def test_generator_jobs_do_not_install_an_independent_npm_cli(filename: str) -> None:
    assert not any("tree-sitter-cli@" in step.get("run", "") for step in workflow_steps(filename)), filename


def executable(path: Path, body: str) -> None:
    path.write_text("#!/bin/bash\nset -euo pipefail\n" + body + "\n")
    path.chmod(0o755)


@pytest.fixture
def tools(tmp_path: Path) -> tuple[Path, dict[str, str]]:
    binaries = tmp_path / "bin"
    canonical = tmp_path / "canonical"
    binaries.mkdir()
    canonical.mkdir()
    executable(binaries / "tree-sitter", 'echo stale-global >"$CLI_RESULT"')
    executable(canonical / "tree-sitter", 'echo canonical >"$CLI_RESULT"')
    executable(binaries / "uv", "tree-sitter --version")
    executable(binaries / "rustup", ":")
    executable(
        binaries / "mise",
        'if [[ "$1" == -C ]]; then cd "$2"; shift 2; fi\n'
        '[[ "$PWD" == "$EXPECTED_ROOT" ]] || exit 88\n'
        'if [[ "$1" == which ]]; then\n'
        '  [[ -f "$TOOL_INSTALLED" ]] || exit 1\n'
        '  printf "%s/tree-sitter\\n" "$CANONICAL_DIR"\n'
        'elif [[ "$*" == "install --locked tree-sitter" ]]; then\n'
        '  printf "%s\\n" "$*" >>"$INSTALL_LOG"\n'
        '  touch "$TOOL_INSTALLED"\n'
        "else exit 99; fi",
    )
    environment = {
        **os.environ,
        "PATH": f"{binaries}:{os.environ['PATH']}",
        "CANONICAL_DIR": str(canonical),
        "TOOL_INSTALLED": str(tmp_path / "installed"),
        "CLI_RESULT": str(tmp_path / "cli-result"),
        "INSTALL_LOG": str(tmp_path / "install-log"),
        "EXPECTED_ROOT": str(tmp_path),
    }
    return binaries, environment


@pytest.mark.parametrize("filename", WORKFLOWS)
def test_generator_commands_prefer_canonical_cli_over_stale_global(
    filename: str, tools: tuple[Path, dict[str, str]], tmp_path: Path
) -> None:
    _, environment = tools
    Path(environment["TOOL_INSTALLED"]).touch()
    commands = [step["run"] for step in workflow_steps(filename) if "scripts/clone_vendors.py" in step.get("run", "")]
    assert len(commands) == 1
    result = subprocess.run(
        ["bash", "-euo", "pipefail", "-c", commands[0]],
        cwd=tmp_path,
        env=environment,
        capture_output=True,
        text=True,
        check=False,
    )
    assert result.returncode == 0, result.stderr
    assert Path(environment["CLI_RESULT"]).read_text() == "canonical\n"


@pytest.mark.parametrize("filename", WORKFLOWS)
def test_generator_commands_stop_if_the_canonical_cli_cannot_be_resolved(
    filename: str, tools: tuple[Path, dict[str, str]], tmp_path: Path
) -> None:
    _, environment = tools
    commands = [step["run"] for step in workflow_steps(filename) if "scripts/clone_vendors.py" in step.get("run", "")]
    assert len(commands) == 1
    result = subprocess.run(
        ["bash", "-euo", "pipefail", "-c", commands[0]],
        cwd=tmp_path,
        env=environment,
        capture_output=True,
        text=True,
        check=False,
    )
    assert result.returncode != 0
    assert not Path(environment["CLI_RESULT"]).exists()


@pytest.mark.parametrize("installed", [False, True])
def test_wasm_staging_resolves_manifest_cli_with_a_stale_global_present(
    installed: bool, tools: tuple[Path, dict[str, str]], tmp_path: Path
) -> None:
    binaries, environment = tools
    script = tmp_path / "scripts/stage_wasm_types.sh"
    script.parent.mkdir()
    script.write_bytes((ROOT / "scripts/stage_wasm_types.sh").read_bytes())
    crate = tmp_path / "crates/ts-pack-core-wasm"
    crate.mkdir(parents=True)
    executable(
        binaries / "wasm-pack",
        "mkdir -p pkg/nodejs; touch pkg/nodejs/ts_pack_core_wasm.d.ts",
    )
    if installed:
        Path(environment["TOOL_INSTALLED"]).touch()
    result = subprocess.run(["bash", str(script)], env=environment, capture_output=True, text=True, check=False)
    assert result.returncode == 0, result.stderr
    assert Path(environment["CLI_RESULT"]).read_text() == "canonical\n"
    log = Path(environment["INSTALL_LOG"])
    assert (log.read_text() if log.exists() else "") == ("" if installed else "install --locked tree-sitter\n")
