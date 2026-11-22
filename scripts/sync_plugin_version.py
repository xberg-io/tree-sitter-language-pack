#!/usr/bin/env python3
"""Sync every manifest that tracks the product version to the core ``Cargo.toml`` version.

``Cargo.toml`` is the repo's version source of truth. Besides the coding-agent plugin config
(rendered into ``plugin/`` by ``ai-rulez generate --plugin``) and the MCP launcher pin, this
script keeps the CLI proxy packages (``cli-proxy/npm``, ``cli-proxy/pypi``) and the MCP Registry
``server.json`` in lockstep with the release, the same way ``alef sync-versions`` pins the
alef-managed binding manifests. The PyPI manifest takes the PEP 440 spelling of a prerelease
(``1.28.0-rc.1`` -> ``1.28.0rc1``); everything else takes the semver form. Run via
``mise run version:sync`` (which then regenerates the plugin bundles); freshness of the generated
bundles themselves is enforced by ``ai-rulez verify --plugin`` in CI.

Usage:
    python3 scripts/sync_plugin_version.py                 # apply
    python3 scripts/sync_plugin_version.py --check         # verify against Cargo.toml
    python3 scripts/sync_plugin_version.py --expect X.Y.Z  # verify every pin equals X.Y.Z

``--check`` answers "do the pins track core?" and is the gate CI runs on `main`.
``--expect`` additionally pins them to the version actually being released, so a release
cannot ship a plugin, proxy package or registry entry that lags the tag it is built from.
Both exit 1 on drift.
"""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PLUGIN_CONFIG = ROOT / "plugin" / ".ai-rulez" / "config.plugins.toml"
if not PLUGIN_CONFIG.exists():
    PLUGIN_CONFIG = ROOT / "plugin" / ".ai-rulez" / "config.toml"
SYNC_HINT = "run `mise run version:sync` to re-pin these manifests and regenerate the plugin bundles"


@dataclass(frozen=True)
class Pin:
    """A manifest field that must carry the product version."""

    label: str
    path: Path
    pattern: re.Pattern[str]
    pep440: bool = False

    def current(self) -> list[str]:
        """Every value the pattern matches; more than one when a file repeats the version."""
        matches = self.pattern.findall(self.path.read_text(encoding="utf-8"))
        if not matches:
            sys.exit(f"could not read {self.label} from {self.path.relative_to(ROOT)}")
        return [match[1] for match in matches]

    def spelling(self, version: str) -> str:
        """The form of ``version`` this manifest's ecosystem accepts."""
        return to_pep440(version) if self.pep440 else version

    def rewrite(self, version: str) -> tuple[str, str]:
        """Return the file text before and after pinning every match to ``version``."""
        original = self.path.read_text(encoding="utf-8")
        if not self.pattern.search(original):
            sys.exit(f"could not read {self.label} from {self.path.relative_to(ROOT)}")
        return original, self.pattern.sub(rf"\g<1>{self.spelling(version)}\g<3>", original)


def to_pep440(version: str) -> str:
    """Respell a semver prerelease the way PyPI does: 1.9.0-rc.2 -> 1.9.0rc2.

    Mirrors the `pypi` form `prepare` computes in publish.yaml; a final release is unchanged.
    """
    version = re.sub(r"-alpha\.?([0-9]+)$", r"a\1", version)
    version = re.sub(r"-beta\.?([0-9]+)$", r"b\1", version)
    return re.sub(r"-([A-Za-z]+)\.?([0-9]+)$", r"\1\2", version)


# ~keep Group 1 and 3 are the text kept around the version, group 2 the version itself.
PINS = (
    # Matches the `version` key of the `[plugin]` table only. The file also has a top-level
    # ai-rulez schema `version = "4.0"` that must never be rewritten.
    Pin("[plugin].version", PLUGIN_CONFIG, re.compile(r'(\[plugin\][^\[]*?\nversion = ")([^"]*)(")', re.DOTALL)),
    # The launcher pins the exact release it downloads and the minimum binary version it accepts.
    Pin(
        "PINNED_VERSION",
        ROOT / "plugin" / ".ai-rulez" / "scripts" / "mcp-launch.sh",
        re.compile(r'(?m)^(PINNED_VERSION=")([^"]*)(")'),
    ),
    Pin(
        "npm proxy version",
        ROOT / "cli-proxy" / "npm" / "package.json",
        re.compile(r'(?m)^(  "version": ")([^"]*)(")'),
    ),
    Pin(
        "PyPI proxy version",
        ROOT / "cli-proxy" / "pypi" / "pyproject.toml",
        re.compile(r'(?m)^(version = ")([^"]*)(")'),
        pep440=True,
    ),
    # The downloader builds the GitHub release URL from this, so it carries the tag form.
    Pin(
        "PyPI proxy __version__",
        ROOT / "cli-proxy" / "pypi" / "ts_pack_cli" / "__init__.py",
        re.compile(r'(?m)^(__version__ = ")([^"]*)(")'),
    ),
    # Both the registry entry's version and its package's version.
    Pin(
        "MCP Registry version",
        ROOT / "server.json",
        re.compile(r'(?m)^(\s*"version": ")([^"]*)(")'),
    ),
)


def core_version() -> str:
    """Read the workspace crate version from ``Cargo.toml``."""
    text = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
    match = re.search(r'(?m)^version = "([^"]+)"', text)
    if not match:
        sys.exit("could not read version from Cargo.toml")
    return match.group(1)


def parse_expect(argv: list[str]) -> str | None:
    """Return the version passed to ``--expect``, or ``None`` when absent."""
    for index, arg in enumerate(argv):
        if arg == "--expect":
            if index + 1 >= len(argv):
                sys.exit("--expect requires a version argument")
            return argv[index + 1]
        if arg.startswith("--expect="):
            return arg.split("=", 1)[1]
    return None


def drift(expected: str, core: str) -> list[str]:
    """Describe every pin (and ``Cargo.toml``) that disagrees with ``expected``."""
    lines = []
    if core != expected:
        lines.append(f"  Cargo.toml [workspace.package].version -> {core}")
    for pin in PINS:
        want = pin.spelling(expected)
        found = pin.current()
        if any(value != want for value in found):
            lines.append(f"  {pin.path.relative_to(ROOT)} {pin.label} -> {', '.join(found)} (expected {want})")
    return lines


def report_drift(expected: str, lines: list[str]) -> int:
    """Print every disagreement and fail."""
    print(f"version drift - expected {expected}")
    print("\n".join(lines))
    print(SYNC_HINT)
    return 1


def main() -> int:
    argv = sys.argv[1:]
    expect = parse_expect(argv)
    core = core_version()

    if expect is not None:
        lines = drift(expect, core)
        if lines:
            return report_drift(expect, lines)
        print(f"Cargo.toml, the plugin, the launcher pin, the CLI proxy packages and server.json all at {expect}")
        return 0

    lines = drift(core, core)
    if not lines:
        return 0
    if "--check" in argv:
        return report_drift(core, lines)
    for pin in PINS:
        _, updated = pin.rewrite(core)
        pin.path.write_text(updated, encoding="utf-8")
    print(f"synced plugin, launcher pin, CLI proxy packages and server.json -> {core}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
