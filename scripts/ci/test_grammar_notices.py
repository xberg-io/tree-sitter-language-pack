"""Unit tests for scripts/build_grammar_notices.py (issue #191).

Network-free: the one network-touching stage (`build`) is exercised with
``resolve_language`` monkeypatched, and every other path reads synthetic files
from ``tmp_path``.
"""

from __future__ import annotations

import hashlib
import json
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import build_grammar_notices as notices


def _sha(text: str) -> str:
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


MIT_TEXT = "MIT License\n\nPermission is hereby granted, free of charge, to any person obtaining a copy\n"
APACHE_TEXT = "Apache License\nVersion 2.0, January 2004\n"


def _entry(repo: str, rev: str, digest: str, spdx: str = "MIT") -> dict:
    return {
        "repo": repo,
        "rev": rev,
        "branch": None,
        "directory": None,
        "local": None,
        "resolution": "github-license",
        "spdx": spdx,
        "spdx_source": "detected",
        "license_files": [{"path": "LICENSE", "sha256": digest}],
    }


def _write_definitions(tmp_path: Path) -> dict:
    definitions = {
        "alpha": {"repo": "https://github.com/a/alpha", "rev": "a" * 40},
        "beta": {},
    }
    path = tmp_path / "language_definitions.json"
    path.write_text(json.dumps(definitions), encoding="utf-8")
    return definitions


@pytest.fixture
def patched_root(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> Path:
    definitions = _write_definitions(tmp_path)
    monkeypatch.setattr(notices, "_DEFINITIONS_PATH", tmp_path / "language_definitions.json")
    monkeypatch.setattr(notices, "_PROJECT_ROOT", tmp_path)
    assert definitions  # keep the fixture's side effect explicit
    return tmp_path


@pytest.mark.parametrize(
    ("text", "expected"),
    [
        (MIT_TEXT, "MIT"),
        (APACHE_TEXT, "Apache-2.0"),
        (
            "ISC License\nPermission to use, copy, modify, and/or distribute this software for any purpose with or without fee\n",
            "ISC",
        ),
        (
            "BSD 2-Clause\nRedistribution and use in source and binary forms, with or without modification\n",
            "BSD-2-Clause",
        ),
        (
            "Redistribution and use in source and binary forms\n3. Neither the name of the copyright holder\n",
            "BSD-3-Clause",
        ),
        (
            "This is free and unencumbered software released into the public domain.\n",
            "Unlicense",
        ),
    ],
)
def test_detect_spdx_identifies_common_permissive_licenses(text: str, expected: str) -> None:
    assert notices.detect_spdx(text) == expected


def test_detect_spdx_returns_none_for_unrecognised_text() -> None:
    assert notices.detect_spdx("some bespoke license nobody has seen") is None


def test_build_aggregates_identical_license_texts_by_digest(
    patched_root: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    digest = _sha(MIT_TEXT)

    def fake_resolve(name: str, _definition: dict, _override: dict | None) -> notices.LanguageResult:
        spdx = "MIT" if name == "alpha" else "Apache-2.0"
        entry = _entry("https://github.com/a/alpha", "a" * 40, digest, spdx)
        return notices.LanguageResult(name=name, entry=entry, texts={digest: MIT_TEXT})

    monkeypatch.setattr(notices, "resolve_language", fake_resolve)
    definitions = json.loads((patched_root / "language_definitions.json").read_text())
    result = notices.build(definitions, overrides={}, only=None)

    assert len(result["licenses"]) == 1, "identical texts must collapse to one digest entry"
    assert result["licenses"][digest] == MIT_TEXT
    assert set(result["languages"]) == {"alpha", "beta"}


def test_build_reports_every_unresolved_language(patched_root: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    def failing_resolve(name: str, _definition: dict, _override: dict | None) -> notices.LanguageResult:
        return notices.LanguageResult(name=name, error="no license text")

    monkeypatch.setattr(notices, "resolve_language", failing_resolve)
    definitions = json.loads((patched_root / "language_definitions.json").read_text())
    with pytest.raises(notices.NoticesError) as error:
        notices.build(definitions, overrides={}, only=None)
    assert "alpha" in str(error.value)
    assert "beta" in str(error.value)


def _complete_notices(patched_root: Path, definitions: dict) -> dict:
    digest = _sha(MIT_TEXT)
    return {
        "schema": notices.SCHEMA,
        "definitions_sha256": _sha((patched_root / "language_definitions.json").read_bytes().decode("utf-8")),
        "languages": {name: _entry("https://github.com/a/alpha", "a" * 40, digest) for name in definitions},
        "licenses": {digest: MIT_TEXT},
    }


def _reload_definitions(patched_root: Path) -> dict:
    return json.loads((patched_root / "language_definitions.json").read_text())


def test_validate_accepts_complete_coverage(patched_root: Path) -> None:
    definitions = _reload_definitions(patched_root)
    notices._validate(_complete_notices(patched_root, definitions), definitions, {})


def test_validate_rejects_missing_language(patched_root: Path) -> None:
    definitions = _reload_definitions(patched_root)
    data = _complete_notices(patched_root, definitions)
    del data["languages"]["beta"]
    with pytest.raises(notices.NoticesError, match="coverage mismatch"):
        notices._validate(data, definitions, {})


def test_validate_rejects_stale_definitions(patched_root: Path) -> None:
    definitions = _reload_definitions(patched_root)
    data = _complete_notices(patched_root, definitions)
    data["definitions_sha256"] = "0" * 64
    with pytest.raises(notices.NoticesError, match="stale"):
        notices._validate(data, definitions, {})


def test_validate_rejects_tampered_license_text(patched_root: Path) -> None:
    definitions = _reload_definitions(patched_root)
    data = _complete_notices(patched_root, definitions)
    digest = next(iter(data["licenses"]))
    data["licenses"][digest] = "tampered"
    with pytest.raises(notices.NoticesError, match="does not match its digest"):
        notices._validate(data, definitions, {})


def test_validate_rejects_orphaned_license_text(patched_root: Path) -> None:
    definitions = _reload_definitions(patched_root)
    data = _complete_notices(patched_root, definitions)
    data["licenses"]["f" * 64] = "orphan"
    with pytest.raises(notices.NoticesError, match="orphaned"):
        notices._validate(data, definitions, {})


def test_validate_rejects_override_that_is_not_the_retained_copy(
    patched_root: Path,
) -> None:
    definitions = _reload_definitions(patched_root)
    data = _complete_notices(patched_root, definitions)
    override_file = patched_root / "override.txt"
    override_file.write_text("a different text", encoding="utf-8")
    overrides = {"alpha": {"file": str(override_file), "spdx": "MIT"}}
    with pytest.raises(notices.NoticesError, match="is not the retained copy"):
        notices._validate(data, definitions, overrides)


def test_render_markdown_lists_each_license_text_once(patched_root: Path) -> None:
    definitions = _reload_definitions(patched_root)
    data = _complete_notices(patched_root, definitions)
    markdown = notices._render_markdown(data, "9.9.9")
    assert "v9.9.9" in markdown
    assert markdown.count(MIT_TEXT.strip()) == 1, "the shared text must appear once, not per language"
    assert "`alpha`" in markdown
    assert "`beta`" in markdown
