"""Assert every registry-mode test app is pinned to the release currently being built.

A registry-mode test app exists to prove that the *just-published* artifacts install and work.
Its value comes entirely from the pinned version matching the release under test: an app pinned
to an older release still installs, still passes, and proves nothing about the release it gates.

Two independent mechanisms let that drift happen silently, both observed at 1.15.7:

1. `alef sync-versions` refuses to apply a `[[workspace.sync.text_replacements]]` rule to a file
   it reaches through its catch-all branch unless the file carries an alef provenance marker --
   it treats such files as hand-written. It reports this as a WARN and exits 0, so the rule looks
   applied. Five apps (dart, elixir, kotlin_android, swift_e2e, php) are reached that way and
   were skipped on every release, drifting up to six releases behind.
2. The `Check version sync` CI step diffs only `packages/` and `crates/`, so `test_apps/` drift is
   outside the gate's scope even when sync does rewrite it.

A sync rule whose regex matches nothing is a third, previously observed failure mode (the dart
pubspec caret). This checker is deliberately independent of alef: it re-derives every pin from
`Cargo.toml` and fails when a pattern matches nothing, so a rule that silently stops matching is
an error here rather than an invisible no-op. ~keep

Usage:
    python3 scripts/check_test_app_pins.py                    # check against Cargo.toml
    python3 scripts/check_test_app_pins.py --release 1.15.7   # check against the release being cut
    python3 scripts/check_test_app_pins.py --fix              # rewrite drifted pins in place
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

import tomllib

ROOT = Path(__file__).resolve().parent.parent

REGISTRY_PACKAGES_PREFIX = "crates.e2e.registry.packages."


@dataclass(frozen=True)
class Pin:
    """One version pin in a test app.

    `pattern` must contain exactly one capturing group, and that group must capture the bare
    version string only -- any surrounding sigil (`v`, `^`, `==`) stays outside it so the rewrite
    preserves it. Every occurrence in the file is checked and rewritten, because some manifests
    repeat the coordinate (kotlin_android names it twice, zig once per platform tarball).

    Only pins that are pure version strings belong here. `test_apps/zig/build.zig.zon`'s `.hash`
    fields also embed a version, but they are content digests over the tarball and legitimately
    name the *previous* release between a version bump and its publish, so gating on them here
    would block the release that has to happen before they can be regenerated. They are checked
    against the artifact instead, by `scripts/sync_zig_zon_hashes.py`. ~keep
    """

    app: str
    path: str
    pattern: str


# ~keep This table intentionally re-states pins that `alef sync-versions` does maintain. The point
# of the gate is to be independent of whether alef wrote them, so that a change in alef's
# named-filename handling shows up as a failure here instead of as silent drift.
PINS: tuple[Pin, ...] = (
    Pin("node", "test_apps/node/package.json", r'"@xberg-io/tree-sitter-language-pack":\s*"([^"]*)"'),
    Pin("wasm", "test_apps/wasm/package.json", r'"@xberg-io/tree-sitter-language-pack-wasm":\s*"([^"]*)"'),
    Pin("wasm", "test_apps/wasm/pnpm-workspace.yaml", r'"@xberg-io/tree-sitter-language-pack-wasm@([^"]*)"'),
    Pin(
        "java",
        "test_apps/java/pom.xml",
        r"<artifactId>tree-sitter-language-pack</artifactId>\s*<version>([^<]+)</version>",
    ),
    Pin(
        "csharp",
        "test_apps/csharp/TreeSitterLanguagePack.E2eTests.csproj",
        r'Include="XbergIo\.TreeSitterLanguagePack"\s+Version="([^"]*)"',
    ),
    Pin("go", "test_apps/go/go.mod", r"xberg-io/tree-sitter-language-pack/packages/go v([^\s]+)"),
    # ~keep Not a test app, but the same drift with an extra hop. `alef e2e generate` copies this
    # literal into `e2e/go/go.mod`, and it runs LAST in `version:sync` -- after the sync step that
    # had already written the new version there -- so a stale value here silently reverts the pin
    # every release. 1.19.0 shipped with `e2e/go/go.mod` still naming 1.18.0 for exactly this
    # reason. Nothing else derives this key, so it is only ever as fresh as the last hand-edit.
    # The `replace` directive beside it makes the version cosmetic for the e2e run, which is why
    # the drift never failed a build and went unnoticed. This checker runs before
    # `alef e2e generate`, so fixing it here is what makes the generated file come out right.
    Pin("go_e2e", "alef.toml", r'(?ms)^\[crates\.e2e\.packages\.go\][^\[]*?^version = "v([0-9][0-9.]*)"'),
    Pin("rust", "test_apps/rust/Cargo.toml", r'package = "tree-sitter-language-pack", version = "([^"]*)"'),
    Pin("ruby", "test_apps/ruby/Gemfile", r"gem 'tree_sitter_language_pack', '([^']*)'"),
    Pin("python", "test_apps/python/pyproject.toml", r'tree-sitter-language-pack==([^"]+)"'),
    Pin("dart", "test_apps/dart/pubspec.yaml", r"tree_sitter_language_pack:\s*\^?([0-9][^\s]*)"),
    Pin("elixir", "test_apps/elixir/mix.exs", r':tree_sitter_language_pack, "([^"]*)"'),
    Pin("kotlin_android", "test_apps/kotlin_android/build.gradle.kts", r'tree-sitter-language-pack-android:([^"]*)"'),
    Pin("swift_e2e", "test_apps/swift_e2e/Package.swift", r'branch: "release/swift/([^"]*)"'),
    Pin("swift", "test_apps/swift/Package.swift", r'tree-sitter-language-pack\.git",\s*from: "([^"]*)"'),
    # ~keep Both installers are alef-generated, so their exact shell syntax is alef's to change:
    # 0.79.5 moved the php pin out of the `${1:-...}` default into its own `PINNED_VERSION`
    # assignment and switched both files' literals from double to single quotes, which turned
    # both patterns into no-matches. Quoting is the half most likely to churn again, so accept
    # either character; the assignment name is anchored to the line start to keep it unambiguous.
    # A further template change still lands here as a loud "matched nothing", which is the point.
    Pin("php", "test_apps/php/install.sh", r"""(?m)^PINNED_VERSION=["']([0-9][0-9.]*)["']"""),
    Pin("c", "test_apps/c/download_ffi.sh", r"""(?m)^VERSION=["']([0-9][0-9.]*)["']"""),
    Pin("zig", "test_apps/zig/build.zig.zon", r"/releases/download/v([0-9][0-9.]*)/tree-sitter-language-pack-zig-"),
    Pin("zig", "test_apps/zig/build.zig.zon", r"tree-sitter-language-pack-zig-v([0-9][0-9.]*)-"),
)


def read_release_version() -> str:
    """Return the version this build would publish, from the workspace Cargo.toml."""
    manifest = ROOT / "Cargo.toml"
    data = tomllib.loads(manifest.read_text(encoding="utf-8"))
    try:
        return data["workspace"]["package"]["version"]
    except KeyError as exc:
        message = f"{manifest}: no [workspace.package].version to gate test-app pins against"
        raise SystemExit(message) from exc


@dataclass
class Finding:
    label: str
    detail: str


def check_go_module(*, fix: bool) -> tuple[list[Finding], bool]:
    """Keep the unmarked registry consumer on the configured Go major-version module."""
    configuration = tomllib.loads((ROOT / "alef.toml").read_text(encoding="utf-8"))
    expected = configuration["crates"][0]["go"]["module"]
    path = ROOT / "test_apps/go/go.mod"
    text = path.read_text(encoding="utf-8")
    pattern = r"github\.com/xberg-io/tree-sitter-language-pack/packages/go(?:/v\d+)?(?=\s+v\d)"
    matches = list(re.finditer(pattern, text))
    if len(matches) != 1:
        return [Finding("test_apps/go/go.mod", "expected exactly one language-pack module requirement")], False
    if matches[0].group(0) == expected:
        return [], False
    if not fix:
        return [Finding("test_apps/go/go.mod", f"module path must be {expected}")], False
    path.write_text(re.sub(pattern, lambda _: expected, text), encoding="utf-8")
    return [], True


def check_pin(pin: Pin, release: str, *, fix: bool) -> tuple[list[Finding], bool]:
    """Check one pin, optionally rewriting it. Returns (findings, file_changed)."""
    path = ROOT / pin.path
    if not path.is_file():
        return [Finding(pin.path, "file does not exist")], False

    text = path.read_text(encoding="utf-8")
    matches = list(re.finditer(pin.pattern, text))
    if not matches:
        # ~keep A rule that matches nothing is the failure mode that hid the dart drift for four
        # releases. It must be an error, never a silent pass.
        return [Finding(pin.path, f"pattern matched nothing: {pin.pattern}")], False

    expected = release
    stale = [m for m in matches if m.group(1) != expected]
    if not stale:
        return [], False

    if not fix:
        found = ", ".join(sorted({m.group(1) for m in stale}))
        return [Finding(pin.path, f"pinned to {found}, expected {expected}")], False

    def replace(match: re.Match[str]) -> str:
        start, end = match.span(1)
        return match.group(0)[: start - match.start()] + expected + match.group(0)[end - match.start() :]

    path.write_text(re.sub(pin.pattern, replace, text), encoding="utf-8")
    return [], True


def registry_pins(release: str, *, fix: bool) -> tuple[list[Finding], bool]:
    """Check the `alef.toml` registry pins that `alef test-apps run` resolves against.

    These are the versions registry mode actually fetches, so they gate the release even more
    directly than the manifests do -- and `alef sync-versions` skips `alef.toml` for the same
    catch-all reason it skips the five manifests. ~keep
    """
    path = ROOT / "alef.toml"
    text = path.read_text(encoding="utf-8")
    data = tomllib.loads(text)

    stale: dict[str, str] = {}
    for crate in data.get("crates", []):
        packages = crate.get("e2e", {}).get("registry", {}).get("packages", {})
        for name, spec in sorted(packages.items()):
            pinned = spec.get("version")
            if pinned is None:
                continue
            expected = f"v{release}" if pinned.startswith("v") else release
            if pinned != expected:
                stale[name] = expected

    if not stale:
        return [], False
    if not fix:
        return [Finding(f"alef.toml [{REGISTRY_PACKAGES_PREFIX}{n}]", f"expected {v}") for n, v in stale.items()], False

    # ~keep Rewrite only the `version =` line inside each stale package's own section, so an
    # unrelated `version` key elsewhere in alef.toml is never touched.
    section = None
    lines = text.splitlines(keepends=True)
    for index, line in enumerate(lines):
        header = re.match(r"\s*\[([^\]]+)\]", line)
        if header:
            section = header.group(1)
            continue
        name = (section or "").removeprefix(REGISTRY_PACKAGES_PREFIX) if section else ""
        if name in stale and re.match(r"\s*version\s*=", line):
            lines[index] = re.sub(r'"[^"]*"', f'"{stale[name]}"', line, count=1)
    path.write_text("".join(lines), encoding="utf-8")
    return [], True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--fix", action="store_true", help="rewrite drifted pins instead of only reporting them")
    # ~keep The publish workflow knows the release it is cutting; passing it makes this gate catch
    # a Cargo.toml that itself disagrees with the tag, which deriving the version cannot.
    parser.add_argument("--release", help="version being released (defaults to [workspace.package].version)")
    args = parser.parse_args()

    if args.release and args.fix:
        parser.error("--fix rewrites pins to the Cargo.toml version; it cannot be combined with --release")

    declared = read_release_version()
    release = args.release.lstrip("v") if args.release else declared

    findings: list[Finding] = []
    fixed: list[str] = []

    if release != declared:
        findings.append(Finding("Cargo.toml [workspace.package].version", f"{declared}, expected {release}"))

    module_findings, module_changed = check_go_module(fix=args.fix)
    findings.extend(module_findings)
    if module_changed:
        fixed.append("test_apps/go/go.mod")

    for pin in PINS:
        pin_findings, changed = check_pin(pin, release, fix=args.fix)
        findings.extend(pin_findings)
        if changed:
            fixed.append(pin.path)

    registry_findings, registry_changed = registry_pins(release, fix=args.fix)
    findings.extend(registry_findings)
    if registry_changed:
        fixed.append("alef.toml")

    if fixed:
        for path in sorted(set(fixed)):
            print(f"fixed  {path} -> {release}")

    if findings:
        print(f"\ntest-app pins do not match the release being built ({release}):\n")
        for finding in findings:
            print(f"  {finding.label}: {finding.detail}")
        print(
            "\nA registry-mode test app pinned to an older release proves that the OLD release "
            "still installs.\nRun `python3 scripts/check_test_app_pins.py --fix` and commit."
        )
        return 1

    print(f"all {len(PINS)} test-app pins and the alef.toml registry pins match the release being built ({release})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
