"""Every shipped macOS binary targets the macOS minimum the root Package.swift declares."""

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


def test_package_minimum_is_macos_15() -> None:
    assert package_minimum() == "15.0"


def test_every_workflow_value_equals_the_package_minimum() -> None:
    minimum = package_minimum()
    found = []
    for path in WORKFLOWS:
        text = path.read_text()
        for where, value in deployment_targets(yaml.safe_load(text)):
            found.append((f"{path.name}{where}", value))
        found.extend((f"{path.name} (run)", value) for value in re.findall(rf"{SETTING}=([\w.\"']+)", text))
    assert found
    assert [entry for entry in found if entry[1].strip("\"'") != minimum] == []


def test_publish_workflow_sets_the_target_for_every_job() -> None:
    workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
    assert workflow["env"][SETTING] == package_minimum()


def test_detector_finds_a_nested_low_value() -> None:
    document = {"jobs": {"swift": {"steps": [{"env": {SETTING: "13.0"}}]}}}
    assert deployment_targets(document) == [("/jobs/swift/steps[0]/env", "13.0")]


def test_cargo_config_does_not_lower_the_target() -> None:
    config = ROOT / ".cargo/config.toml"
    assert SETTING not in config.read_text() or f'{SETTING} = "{package_minimum()}"' in config.read_text()
