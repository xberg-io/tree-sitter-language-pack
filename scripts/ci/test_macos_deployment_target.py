"""The macOS binary target and the Swift package minimum each hold their own floor.

Binaries read the target from publish.yaml's top-level env; the Swift minimum reads the root Package.swift.
"""

import re
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = sorted((ROOT / ".github/workflows").glob("*.y*ml"))
SETTING = "MACOSX_DEPLOYMENT_TARGET"


def package_minimum() -> str:
    found = re.findall(r"\.macOS\(\.v(\d+)\)", (ROOT / "Package.swift").read_text())
    assert len(found) == 1, found
    return f"{found[0]}.0"


def deployment_targets(document: object, where: str = "") -> list[tuple[str, str]]:
    found = []
    if isinstance(document, dict):
        for key, value in document.items():
            if key == SETTING:
                found.append((where, str(value)))
            found.extend(deployment_targets(value, f"{where}/{key}"))
    elif isinstance(document, list):
        for index, value in enumerate(document):
            found.extend(deployment_targets(value, f"{where}[{index}]"))
    return found


def binary_target() -> str:
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    return str(workflow["env"][SETTING])


def test_binary_target_is_macos_10_13() -> None:
    assert binary_target() == "10.13"


def test_package_minimum_is_macos_13() -> None:
    assert package_minimum() == "13.0"


def test_every_workflow_value_equals_the_binary_target() -> None:
    target = binary_target()
    found = []
    for path in WORKFLOWS:
        text = path.read_text()
        for where, value in deployment_targets(yaml.safe_load(text)):
            found.append((f"{path.name}{where}", value))
        found.extend((f"{path.name} (run)", value) for value in re.findall(rf"{SETTING}=([\w.\"']+)", text))
    assert found
    assert [entry for entry in found if entry[1].strip("\"'") != target] == []


def test_detector_finds_a_nested_low_value() -> None:
    document = {"jobs": {"swift": {"steps": [{"env": {SETTING: "13.0"}}]}}}
    assert deployment_targets(document) == [("/jobs/swift/steps[0]/env", "13.0")]


def test_cargo_config_does_not_change_the_target() -> None:
    config = ROOT / ".cargo/config.toml"
    assert SETTING not in config.read_text() or f'{SETTING} = "{binary_target()}"' in config.read_text()


def test_every_package_manifest_and_alef_match_the_package_minimum() -> None:
    minimum = package_minimum()
    manifests = [ROOT / "packages/swift/Package.swift", *sorted(ROOT.glob("e2e/swift*/Package.swift"))]
    manifests += sorted(ROOT.glob("test_apps/swift*/Package.swift"))
    declared = {str(path.relative_to(ROOT)): re.findall(r"\.macOS\(\.v(\d+)\)", path.read_text()) for path in manifests}
    assert {path: found for path, found in declared.items() if found != [minimum.split(".")[0]]} == {}
    alef = re.search(r'^min_macos_version = "([\d.]+)"', (ROOT / "alef.toml").read_text(), re.MULTILINE)
    assert alef is not None
    assert alef.group(1) == minimum
