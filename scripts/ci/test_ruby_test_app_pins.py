"""Keep Ruby registry pins checkable after generator quoting changes."""

from pathlib import Path

import pytest

from scripts import check_test_app_pins


@pytest.mark.parametrize("name_quote", ["'", '"'])
@pytest.mark.parametrize("version_quote", ["'", '"'])
def test_ruby_pin_preserves_valid_quoting_when_checking_and_fixing(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch, name_quote: str, version_quote: str
) -> None:
    app = tmp_path / "test_apps/ruby"
    app.mkdir(parents=True)
    source = f"gem {name_quote}tree_sitter_language_pack{name_quote}, {version_quote}1.22.2{version_quote}\n"
    manifest = app / "Gemfile"
    manifest.write_text(source)
    monkeypatch.setattr(check_test_app_pins, "ROOT", tmp_path)
    pin = next(pin for pin in check_test_app_pins.PINS if pin.app == "ruby")

    findings, changed = check_test_app_pins.check_pin(pin, "2.0.0", fix=False)
    assert [finding.detail for finding in findings] == ["pinned to 1.22.2, expected 2.0.0"]
    assert changed is False
    assert manifest.read_text() == source

    findings, changed = check_test_app_pins.check_pin(pin, "2.0.0", fix=True)
    assert findings == []
    assert changed is True
    assert manifest.read_text() == source.replace("1.22.2", "2.0.0")
    assert check_test_app_pins.check_pin(pin, "2.0.0", fix=False) == ([], False)


def test_missing_ruby_pin_still_fails_closed(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    app = tmp_path / "test_apps/ruby"
    app.mkdir(parents=True)
    (app / "Gemfile").write_text('gem "rspec", "~> 3.13"\n')
    monkeypatch.setattr(check_test_app_pins, "ROOT", tmp_path)
    pin = next(pin for pin in check_test_app_pins.PINS if pin.app == "ruby")
    findings, changed = check_test_app_pins.check_pin(pin, "2.0.0", fix=True)
    assert len(findings) == 1
    assert findings[0].detail.startswith("pattern matched nothing:")
    assert changed is False
