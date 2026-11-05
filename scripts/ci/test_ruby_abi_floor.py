"""Every Ruby the publish workflow stages or verifies a native gem on satisfies the gemspec's floor."""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
GEMSPEC = ROOT / "packages/ruby/tree_sitter_language_pack.gemspec"
PUBLISH = ROOT / ".github/workflows/publish.yaml"


def version_tuple(version: str) -> tuple[int, ...]:
    return tuple(int(part) for part in version.split("."))


def gemspec_floor() -> tuple[int, ...]:
    found = re.findall(r'required_ruby_version\s*=\s*">=\s*([\d.]+)"', GEMSPEC.read_text())
    assert len(found) == 1, found
    return version_tuple(found[0])[:2]


def workflow_rubies(text: str) -> set[str]:
    return set(re.findall(r"ruby=(\d+\.\d+)", text)) | set(re.findall(r'ruby-version:\s*"(\d+\.\d+)"', text))


def test_publish_workflow_installs_rubies() -> None:
    assert len(workflow_rubies(PUBLISH.read_text())) >= 2


def test_every_published_ruby_meets_the_gemspec_floor() -> None:
    floor = gemspec_floor()
    below = sorted(ruby for ruby in workflow_rubies(PUBLISH.read_text()) if version_tuple(ruby) < floor)
    assert below == []


def test_detector_finds_a_ruby_below_the_floor() -> None:
    text = 'tools: ruby=3.2\n          ruby-version: "3.2"\n          ruby-version: "4.0"\n'
    assert workflow_rubies(text) == {"3.2", "4.0"}
