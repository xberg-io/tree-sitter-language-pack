"""A pinned grammar revision must still check out after upstream force-pushes it away.

usagi-coffee/tree-sitter-abl force-pushed master on 2026-11-03, leaving the pinned commit
2b5bbfe reachable from no branch or tag. A full clone does not carry it, so `git checkout <rev>`
failed with "unable to read tree" and every fresh vendor clone broke. GitHub still serves the
commit by id, so the checkout falls back to fetching the pinned revision directly.
"""

from __future__ import annotations

import asyncio
import subprocess
import sys
from pathlib import Path

from git import Repo

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import clone_vendors


def _git(*args: str, cwd: Path) -> str:
    return subprocess.run(["git", *args], cwd=cwd, check=True, capture_output=True, text=True).stdout.strip()


def _commit(repo: Path, name: str) -> str:
    (repo / name).write_text(f"{name}\n")
    _git("add", name, cwd=repo)
    _git("-c", "user.name=t", "-c", "user.email=t@t", "commit", "-qm", name, cwd=repo)
    return _git("rev-parse", "HEAD", cwd=repo)


def _force_pushed_upstream(tmp_path: Path) -> tuple[Path, str]:
    upstream = tmp_path / "upstream"
    upstream.mkdir()
    _git("init", "-q", "-b", "master", cwd=upstream)
    pinned = _commit(upstream, "pinned.txt")
    _git("checkout", "-q", "--orphan", "rewritten", cwd=upstream)
    _git("rm", "-rqf", ".", cwd=upstream)
    _commit(upstream, "rewritten.txt")
    _git("branch", "-qf", "master", "rewritten", cwd=upstream)
    _git("checkout", "-q", "master", cwd=upstream)
    _git("branch", "-qD", "rewritten", cwd=upstream)
    _git("config", "uploadpack.allowAnySHA1InWant", "true", cwd=upstream)
    return upstream, pinned


def test_checkout_fetches_a_pinned_revision_no_branch_reaches(tmp_path: Path) -> None:
    upstream, pinned = _force_pushed_upstream(tmp_path)
    clone = Repo.clone_from(f"file://{upstream}", tmp_path / "clone")
    missing = subprocess.run(["git", "cat-file", "-e", pinned], cwd=tmp_path / "clone", check=False)
    assert missing.returncode != 0, "the clone must not already carry the force-pushed commit"

    asyncio.run(clone_vendors._checkout_rev(clone, pinned, "abl"))

    assert clone.head.commit.hexsha == pinned
    assert (tmp_path / "clone" / "pinned.txt").is_file()


def test_checkout_of_a_reachable_revision_needs_no_fetch(tmp_path: Path) -> None:
    upstream = tmp_path / "upstream"
    upstream.mkdir()
    _git("init", "-q", "-b", "master", cwd=upstream)
    first = _commit(upstream, "first.txt")
    _commit(upstream, "second.txt")
    clone = Repo.clone_from(f"file://{upstream}", tmp_path / "clone")

    asyncio.run(clone_vendors._checkout_rev(clone, first, "abl"))

    assert clone.head.commit.hexsha == first
