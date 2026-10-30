"""Exercise prerelease Go resolution across semantic import major versions."""

import re
from pathlib import Path

import pytest

from scripts import check_test_app_pins, prerelease_test_apps


@pytest.mark.parametrize("major", [1, 2])
def test_go_prerelease_resolves_the_configured_module(
    tmp_path: Path, monkeypatch: pytest.MonkeyPatch, major: int
) -> None:
    module = "github.com/xberg-io/tree-sitter-language-pack/packages/go"
    if major > 1:
        module += f"/v{major}"
    package = tmp_path / "packages/go"
    package.mkdir(parents=True)
    (package / "go.mod").write_text(f"module {module}\n\ngo 1.26\n")
    (tmp_path / "alef.toml").write_text(f'[[crates]]\n[crates.go]\nmodule = "{module}"\n')
    staged = tmp_path / "staged"
    staged.mkdir()
    (staged / "go.mod").write_text(f"module example.com/consumer\n\ngo 1.26\n\nrequire {module} v{major}.0.0\n")
    monkeypatch.setattr(prerelease_test_apps, "ROOT", tmp_path)
    monkeypatch.setenv("GOPROXY", "off")
    prerelease_test_apps._write_go_override(staged)
    assert prerelease_test_apps.verify_resolution(prerelease_test_apps.TARGETS["go"], staged)


def test_go_version_pin_accepts_the_major_version_module(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    app = tmp_path / "test_apps/go"
    app.mkdir(parents=True)
    (app / "go.mod").write_text(
        "module example.com/consumer\n\nrequire github.com/xberg-io/tree-sitter-language-pack/packages/go/v2 v2.0.0\n"
    )
    monkeypatch.setattr(check_test_app_pins, "ROOT", tmp_path)
    pin = next(pin for pin in check_test_app_pins.PINS if pin.app == "go")
    findings, changed = check_test_app_pins.check_pin(pin, "2.0.0", fix=False)
    assert findings == []
    assert changed is False


def test_handwritten_capsule_test_imports_the_current_go_module() -> None:
    root = Path(__file__).resolve().parents[2]
    module = next(
        line.removeprefix("module ").strip()
        for line in (root / "packages/go/go.mod").read_text().splitlines()
        if line.startswith("module ")
    )
    source = (root / "e2e/go/capsule_passthrough_test.go").read_text()
    imported = re.search(r'\btspack\s+"([^"]+)"', source)
    assert imported is not None
    assert imported.group(1) == module


def test_go_readme_installation_uses_the_current_module() -> None:
    root = Path(__file__).resolve().parents[2]
    module = next(
        line.removeprefix("module ").strip()
        for line in (root / "packages/go/go.mod").read_text().splitlines()
        if line.startswith("module ")
    )
    readme = (root / "packages/go/README.md").read_text()
    assert f"go get {module}\n" in readme
    assert f'https://pkg.go.dev/{module}"' in readme
    assert f'"{module}"' in readme
