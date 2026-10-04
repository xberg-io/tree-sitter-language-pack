"""clone_vendors.py must not inherit the caller's repo-local git environment.

A git hook exports GIT_INDEX_FILE (and friends) pointing at the committing repo. The
pre-commit hook builds the crate, build.rs runs clone_vendors.py, and every vendor
`git` call then wrote the grammar checkout's index over the committing repo's index.
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import clone_vendors


def _git(*args: str, cwd: Path) -> None:
    subprocess.run(["git", *args], cwd=cwd, check=True, capture_output=True)


def _make_repo(path: Path, filename: str) -> None:
    path.mkdir()
    _git("init", "-q", cwd=path)
    (path / filename).write_text("x\n")
    _git("add", filename, cwd=path)
    _git("-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", "init", cwd=path)


def _simulate_vendor_checkout(vendor: Path) -> None:
    _git("checkout", "HEAD", "--", ".", cwd=vendor)
    _git("add", "-A", cwd=vendor)


@pytest.fixture
def hook_env(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> tuple[Path, Path]:
    outer = tmp_path / "outer"
    vendor = tmp_path / "vendor"
    _make_repo(outer, "outer.txt")
    _make_repo(vendor, "vendor.txt")
    monkeypatch.setenv("GIT_INDEX_FILE", str(outer / ".git" / "index"))
    monkeypatch.setenv("GIT_DIR", str(outer / ".git"))
    return outer, vendor


def test_leaked_hook_env_corrupts_outer_index_without_scrub(hook_env: tuple[Path, Path]) -> None:
    outer, vendor = hook_env
    index = outer / ".git" / "index"
    before = index.read_bytes()
    _simulate_vendor_checkout(vendor)
    assert index.read_bytes() != before


def test_scrub_keeps_vendor_git_calls_off_the_outer_index(hook_env: tuple[Path, Path]) -> None:
    outer, vendor = hook_env
    index = outer / ".git" / "index"
    before = index.read_bytes()
    clone_vendors._scrub_repo_local_git_env()
    _simulate_vendor_checkout(vendor)
    assert index.read_bytes() == before
    assert "GIT_INDEX_FILE" not in os.environ
    assert "GIT_DIR" not in os.environ


def test_scrub_leaves_unrelated_git_settings(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setenv("GIT_LFS_SKIP_SMUDGE", "1")
    monkeypatch.setenv("GIT_TERMINAL_PROMPT", "0")
    monkeypatch.setenv("GIT_WORK_TREE", "/nonexistent")
    clone_vendors._scrub_repo_local_git_env()
    assert os.environ["GIT_LFS_SKIP_SMUDGE"] == "1"
    assert os.environ["GIT_TERMINAL_PROMPT"] == "0"
    assert "GIT_WORK_TREE" not in os.environ


if __name__ == "__main__":
    raise SystemExit(pytest.main([__file__, "-q"]))
