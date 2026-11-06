"""Vendoring keeps each grammar's upstream licence files next to its parser sources."""

from __future__ import annotations

import asyncio
import hashlib
import json
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import _vendor_licenses as licenses
import clone_vendors as vendor

REPO = "https://github.com/example/tree-sitter-example"
REV = "a" * 40


def _write(path: Path, text: str) -> Path:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)
    return path


@pytest.fixture
def layout(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> tuple[Path, Path]:
    vendor_dir, parsers_dir = tmp_path / "vendor", tmp_path / "parsers"
    monkeypatch.setattr(vendor, "vendor_directory", vendor_dir)
    monkeypatch.setattr(vendor, "parsers_directory", parsers_dir)
    return vendor_dir, parsers_dir


def _notices(tmp_path: Path, name: str, entry: dict, texts: dict[str, str]) -> Path:
    path = tmp_path / "grammar_notices.json"
    path.write_text(json.dumps({"languages": {name: entry}, "licenses": texts}))
    return path


def test_root_and_subdirectory_licences_are_kept(tmp_path: Path) -> None:
    repo = tmp_path / "vendor" / "tsx"
    _write(repo / "LICENSE", "root MIT\n")
    _write(repo / "COPYING.md", "root copying\n")
    _write(repo / "tsx" / "LICENSE", "subdir MIT\n")
    _write(repo / "tsx" / "NOTICE", "subdir notice\n")
    _write(repo / "README.md", "not a licence\n")
    target = tmp_path / "parsers" / "tsx"
    target.mkdir(parents=True)

    kept = licenses.retain_licenses("tsx", {"repo": REPO, "rev": REV, "directory": "tsx"}, repo, target)

    assert kept == ["COPYING.md", "LICENSE", "LICENSE.root", "NOTICE"]
    assert (target / "LICENSE").read_text() == "subdir MIT\n"
    assert (target / "LICENSE.root").read_text() == "root MIT\n"
    assert not (target / "README.md").exists()


def test_identical_root_and_subdirectory_licence_is_kept_once(tmp_path: Path) -> None:
    repo = tmp_path / "vendor" / "php"
    _write(repo / "LICENSE", "MIT\n")
    _write(repo / "php" / "LICENSE", "MIT\n")
    target = tmp_path / "parsers" / "php"
    target.mkdir(parents=True)
    assert licenses.retain_licenses("php", {"repo": REPO, "rev": REV, "directory": "php"}, repo, target) == ["LICENSE"]


def test_stale_licence_files_from_a_previous_run_are_replaced(tmp_path: Path) -> None:
    repo = tmp_path / "vendor" / "c"
    _write(repo / "LICENSE", "new\n")
    target = tmp_path / "parsers" / "c"
    _write(target / "LICENSE-OLD", "old\n")
    assert licenses.retain_licenses("c", {"repo": REPO, "rev": REV}, repo, target) == ["LICENSE"]
    assert not (target / "LICENSE-OLD").exists()


def test_missing_upstream_licence_gets_a_generated_notice_from_the_committed_record(tmp_path: Path) -> None:
    repo = tmp_path / "vendor" / "brightscript"
    _write(repo / "Cargo.toml", '[package]\nlicense = "MIT"\n')
    target = tmp_path / "parsers" / "brightscript"
    target.mkdir(parents=True)
    entry = {
        "repo": REPO,
        "rev": REV,
        "directory": None,
        "local": None,
        "resolution": "github-manifest",
        "spdx": "MIT",
        "spdx_source": "manifest",
        "license_files": [{"path": "Cargo.toml", "sha256": "f" * 64}],
    }
    notices_path = _notices(tmp_path, "brightscript", entry, {"f" * 64: '[package]\nlicense = "MIT"\n'})

    kept = licenses.retain_licenses("brightscript", {"repo": REPO, "rev": REV}, repo, target, notices_path=notices_path)

    assert kept == ["NOTICE"]
    notice = (target / "NOTICE").read_text()
    assert "SPDX-License-Identifier: MIT\n" in notice
    assert f"Upstream: {REPO} @ {REV}\n" in notice
    assert "Cargo.toml (sha256 " + "f" * 64 + ")" in notice
    assert "[package]" not in notice


def test_generated_notice_retains_an_override_licence_text(tmp_path: Path) -> None:
    repo = tmp_path / "vendor" / "groovy"
    repo.mkdir(parents=True)
    target = tmp_path / "parsers" / "groovy"
    target.mkdir(parents=True)
    text = "MIT License\n\nCopyright (c) groovy authors\n"
    digest = hashlib.sha256(text.encode()).hexdigest()
    entry = {
        "repo": REPO,
        "rev": REV,
        "resolution": "override",
        "spdx": "MIT",
        "spdx_source": "override",
        "license_files": [{"path": "sources/notice_overrides/groovy/LICENSE", "sha256": digest}],
    }
    notices_path = _notices(tmp_path, "groovy", entry, {digest: text})

    licenses.retain_licenses("groovy", {"repo": REPO, "rev": REV}, repo, target, notices_path=notices_path)

    assert (target / "NOTICE").read_text().endswith(text)


def test_language_missing_from_the_record_falls_back_to_the_inline_licence(tmp_path: Path) -> None:
    repo = tmp_path / "vendor" / "x"
    repo.mkdir(parents=True)
    target = tmp_path / "parsers" / "x"
    target.mkdir(parents=True)
    notices_path = tmp_path / "grammar_notices.json"
    notices_path.write_text(json.dumps({"languages": {}, "licenses": {}}))
    licenses.retain_licenses(
        "x", {"repo": REPO, "rev": REV, "license": "Apache-2.0"}, repo, target, notices_path=notices_path
    )
    assert "SPDX-License-Identifier: Apache-2.0\n" in (target / "NOTICE").read_text()


def test_move_src_folder_retains_licences(layout: tuple[Path, Path]) -> None:
    vendor_dir, parsers_dir = layout
    repo = vendor_dir / "example"
    _write(repo / "src" / "parser.c", "int x;\n")
    _write(repo / "LICENSE", "MIT\n")
    asyncio.run(vendor.move_src_folder("example", None, language_definition={"repo": REPO, "rev": REV}))
    assert (parsers_dir / "example" / "src" / "parser.c").is_file()
    assert (parsers_dir / "example" / "LICENSE").read_text() == "MIT\n"


def test_cached_parser_without_a_licence_is_refreshed(
    layout: tuple[Path, Path], monkeypatch: pytest.MonkeyPatch
) -> None:
    _, parsers_dir = layout
    monkeypatch.delenv("TSLP_NO_CACHE", raising=False)
    definition = {"repo": REPO, "rev": REV}
    manifest = {"example": vendor._language_cache_key(definition)}
    _write(parsers_dir / "example" / "src" / "parser.c", "int x;\n")
    assert not vendor._is_language_cached("example", definition, manifest)
    _write(parsers_dir / "example" / "LICENSE", "MIT\n")
    assert vendor._is_language_cached("example", definition, manifest)
