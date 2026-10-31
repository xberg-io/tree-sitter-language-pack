"""Keep the excluded Elixir NIF's core requirement aligned with release corrections."""

from pathlib import Path

import pytest
import tomllib

from scripts import check_test_app_pins

NIF_PATH = "packages/elixir/native/tree_sitter_language_pack_nif/Cargo.toml"


def nif_pin() -> check_test_app_pins.Pin:
    matches = [pin for pin in check_test_app_pins.PINS if pin.path == NIF_PATH]
    assert len(matches) == 1, "the release gate must examine the excluded Elixir NIF core requirement"
    return matches[0]


def test_current_native_requirement_matches_the_canonical_release() -> None:
    findings, changed = check_test_app_pins.check_pin(nif_pin(), check_test_app_pins.read_release_version(), fix=False)
    assert findings == []
    assert changed is False


def test_native_requirement_can_be_corrected_without_rewriting_other_versions(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch
) -> None:
    manifest = tmp_path / NIF_PATH
    manifest.parent.mkdir(parents=True)
    source = (
        '[package]\nname = "tree_sitter_language_pack_nif"\nversion = "1.23.0"\n'
        '[dependencies]\nserde_json = "2.0.0"\n'
        'tree-sitter-language-pack = { version = "2.0.0", path = "../../../../crates/ts-pack-core" }\n'
    )
    manifest.write_text(source)
    monkeypatch.setattr(check_test_app_pins, "ROOT", tmp_path)
    findings, changed = check_test_app_pins.check_pin(nif_pin(), "1.23.0", fix=False)
    assert [finding.detail for finding in findings] == ["pinned to 2.0.0, expected 1.23.0"]
    assert changed is False
    assert manifest.read_text() == source
    assert check_test_app_pins.check_pin(nif_pin(), "1.23.0", fix=True) == ([], True)
    assert manifest.read_text() == source.replace(
        'tree-sitter-language-pack = { version = "2.0.0"',
        'tree-sitter-language-pack = { version = "1.23.0"',
    )
    assert check_test_app_pins.check_pin(nif_pin(), "1.23.0", fix=False) == ([], False)


def test_missing_native_requirement_fails_closed(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    manifest = tmp_path / NIF_PATH
    manifest.parent.mkdir(parents=True)
    manifest.write_text('[package]\nversion = "1.23.0"\n')
    monkeypatch.setattr(check_test_app_pins, "ROOT", tmp_path)
    findings, changed = check_test_app_pins.check_pin(nif_pin(), "1.23.0", fix=False)
    assert len(findings) == 1
    assert findings[0].detail.startswith("pattern matched nothing:")
    assert changed is False


def test_canonical_sync_explicitly_covers_the_native_core_requirement() -> None:
    config = tomllib.loads((check_test_app_pins.ROOT / "alef.toml").read_text())
    rules = [rule for rule in config["workspace"]["sync"]["text_replacements"] if rule["path"] == NIF_PATH]
    assert any("tree-sitter-language-pack" in rule["search"] for rule in rules)
