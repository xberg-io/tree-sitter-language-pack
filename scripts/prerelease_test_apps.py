"""Run the registry-mode test apps against in-repo sources instead of public registries.

Registry mode (`alef test-apps run`) resolves every package from its public registry at the
version pinned in `alef.toml` `[crates.e2e.registry.packages.*]`. `alef sync-versions` rewrites
those pins as the first step of release prep, so from the bump until the publish finishes the
pinned version exists on no registry and the whole suite is unrunnable — precisely during the
window where it would catch a bad release.

This provides a "prerelease" mode: a throwaway copy of each test app is staged under
`.prerelease/`, its dependency resolution is redirected at the in-repo package source using that
ecosystem's own override mechanism, and the app's normal test command runs there.

Why in-repo source overrides rather than a local registry: covering these ecosystems with real
registries means standing up a separate daemon per ecosystem (verdaccio, devpi, a Maven repo, a
NuGet server, a Hex mirror, a pub server, ...), several of which have no usable local-server
story at all. Every one of these package managers already ships a first-class "resolve this
dependency from a local path instead" mechanism, which needs no daemon and no network, and is
the mechanism these ecosystems' own maintainers use before a release.

Why a staged copy rather than editing `test_apps/` in place: `test_apps/` is alef-generated and
hash-stamped, so editing it makes `alef verify` fail and makes a subsequent `alef test-apps run`
report a wrong cause. Staging keeps every tracked file untouched and lets lockfiles churn freely.
`.prerelease/<app>` sits at the same depth below the repo root as `test_apps/<app>`, so the
`../../` references the generated apps use for native libraries keep resolving to the repo root.

Usage:
    python3 scripts/prerelease_test_apps.py status
    python3 scripts/prerelease_test_apps.py stage [--lang rust,go]
    python3 scripts/prerelease_test_apps.py verify [--lang rust,go]
    python3 scripts/prerelease_test_apps.py run [--lang rust,go]
    python3 scripts/prerelease_test_apps.py clean
"""

from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from collections.abc import Callable

ROOT = Path(__file__).resolve().parent.parent
STAGING_ROOT = ROOT / ".prerelease"

# ~keep Copied trees are throwaway, but a stale build directory from the source tree makes a
# staged run resolve against artifacts built for the registry version instead of the override.
IGNORED_WHEN_STAGING = shutil.ignore_patterns(
    ".build",
    ".dart_tool",
    ".gradle",
    "_build",
    "build",
    "deps",
    "node_modules",
    "target",
    "vendor",
    "zig-cache",
    "zig-out",
)


@dataclass(frozen=True)
class Target:
    """One prerelease-capable test app.

    `app_subdir` names the directory under `test_apps/`; `local_package` is the repo-relative
    directory that replaces the registry package; `apply_override` rewrites the staged manifest;
    `verify` is a resolve-only command whose combined output must mention the absolute
    `local_package` path, which proves the redirect took effect without compiling anything; `run`
    is the app's normal test command.
    """

    name: str
    app_subdir: str
    local_package: str
    apply_override: Callable[[Path], None]
    verify: str | None
    run: str


# ~keep Each entry names why the ecosystem has no source-path override, so this reads as a
# scoping decision rather than an oversight. Adding one means producing the local artifact
# first: these package managers resolve a built distributable, not a source tree.
UNSUPPORTED: dict[str, str] = {
    "node": "npm resolves a prebuilt NAPI package; needs `napi build --release` output from "
    "crates/ts-pack-core-node and a pnpm `overrides` entry pointing at the packed tarball",
    "wasm": "npm resolves a prebuilt wasm-pack package; needs `wasm-pack build --target web --release` "
    "output from crates/ts-pack-core-wasm and a pnpm `overrides` entry pointing at the packed tarball",
    "java": "Maven has no path dependency; the local equivalent is "
    "`mvn -f packages/java/pom.xml install` into ~/.m2 before the run",
    "csharp": "NuGet has no path dependency; the local equivalent is `dotnet pack packages/csharp` "
    "into a folder feed declared in a staged nuget.config",
    "zig": "build.zig.zon resolves a GitHub release tarball by URL and hash; the local "
    "equivalent needs the release tarball built and rehashed first",
    "c": "download_ffi.sh downloads a prebuilt FFI archive from the GitHub release; the local "
    "equivalent is ALEF_FFI_LOCAL_DIR against `mise run ffi:build` output",
    "kotlin_android": "Gradle resolves a published AAR; the local equivalent is "
    "`gradle assembleRelease` in packages/kotlin-android plus a mavenLocal() publish",
    "php": "already resolves the package from ../../crates/ts-pack-core-php via a composer "
    "psr-4 path autoload, so it never depended on packagist and needs no override",
}


def _write_rust_override(staged: Path) -> None:
    """Redirect crates.io to the in-repo crate via a cargo config-level `[patch]`.

    A `[patch.crates-io]` block in `.cargo/config.toml` is additive to the generated
    `Cargo.toml`, so the alef-stamped manifest stays byte-identical to what alef wrote.
    """
    local = ROOT / "crates" / "ts-pack-core"
    cargo_dir = staged / ".cargo"
    cargo_dir.mkdir(exist_ok=True)
    (cargo_dir / "config.toml").write_text(f'[patch.crates-io]\ntree-sitter-language-pack = {{ path = "{local}" }}\n')


def _write_go_override(staged: Path) -> None:
    """Add a workspace-level `replace` so the module resolves to `packages/go`.

    A `go.work` beside the app takes precedence over the repo-root workspace, and a directory
    `replace` needs no checksum, so `go.sum` stays valid unmodified.
    """
    local = ROOT / "packages" / "go"
    module = "github.com/xberg-io/tree-sitter-language-pack/packages/go"
    (staged / "go.work").write_text(f"go 1.26\n\nuse .\n\nreplace {module} => {local}\n")


def _write_python_override(staged: Path) -> None:
    """Point uv at `packages/python` for the pinned distribution."""
    local = ROOT / "packages" / "python"
    pyproject = staged / "pyproject.toml"
    text = pyproject.read_text()
    if "[tool.uv.sources]" in text:
        raise SystemExit("python test app already declares [tool.uv.sources]; override would conflict")
    pyproject.write_text(
        f'{text.rstrip(chr(10))}\n\n[tool.uv.sources]\ntree-sitter-language-pack = {{ path = "{local}" }}\n'
    )
    (staged / "uv.lock").unlink(missing_ok=True)


def _write_ruby_override(staged: Path) -> None:
    """Swap the version-pinned gem for a `path:` gem and drop the now-unsatisfiable lock."""
    local = ROOT / "packages" / "ruby"
    gemfile = staged / "Gemfile"
    text, count = re.subn(
        r"^gem ['\"]tree_sitter_language_pack['\"].*$",
        f"gem 'tree_sitter_language_pack', path: '{local}'",
        gemfile.read_text(),
        flags=re.MULTILINE,
    )
    if count != 1:
        raise SystemExit(f"expected exactly one tree_sitter_language_pack gem line, found {count}")
    gemfile.write_text(text)
    (staged / "Gemfile.lock").unlink(missing_ok=True)


def _write_dart_override(staged: Path) -> None:
    """Append a `dependency_overrides` block, which outranks the pinned `dependencies` entry."""
    local = ROOT / "packages" / "dart"
    pubspec = staged / "pubspec.yaml"
    text = pubspec.read_text()
    if "dependency_overrides:" in text:
        raise SystemExit("dart test app already declares dependency_overrides; override would conflict")
    pubspec.write_text(
        f"{text.rstrip(chr(10))}\n\ndependency_overrides:\n  tree_sitter_language_pack:\n    path: {local}\n"
    )
    (staged / "pubspec.lock").unlink(missing_ok=True)


def _write_elixir_override(staged: Path) -> None:
    """Swap the Hex dep for a `path:` dep and drop the now-unsatisfiable lock."""
    local = ROOT / "packages" / "elixir"
    mix_exs = staged / "mix.exs"
    text, count = re.subn(
        r"\{:tree_sitter_language_pack,[^}]*\}",
        f'{{:tree_sitter_language_pack, path: "{local}"}}',
        mix_exs.read_text(),
    )
    if count != 1:
        raise SystemExit(f"expected exactly one tree_sitter_language_pack dep, found {count}")
    mix_exs.write_text(text)
    (staged / "mix.lock").unlink(missing_ok=True)


def _write_swift_override(staged: Path) -> None:
    """Swap the SCM dependency for a local path package.

    SwiftPM derives a path package's identity from its directory basename, not from the name in
    its manifest, so every `.product(package:)` reference has to move to `swift` at the same time
    or resolution fails with an unknown-package error.
    """
    local = ROOT / "packages" / "swift"
    manifest = staged / "Package.swift"
    text, count = re.subn(
        r"\.package\(url: \"https://github\.com/xberg-io/tree-sitter-language-pack\"[^)]*\)",
        f'.package(path: "{local}")',
        manifest.read_text(),
    )
    if count != 1:
        raise SystemExit(f"expected exactly one tree-sitter-language-pack SCM dependency, found {count}")
    manifest.write_text(text.replace('package: "tree-sitter-language-pack"', 'package: "swift"'))
    (staged / "Package.resolved").unlink(missing_ok=True)


TARGETS: dict[str, Target] = {
    target.name: target
    for target in (
        Target(
            name="rust",
            app_subdir="rust",
            local_package="crates/ts-pack-core",
            apply_override=_write_rust_override,
            verify="cargo metadata --format-version 1 --quiet",
            run="cargo test",
        ),
        Target(
            name="go",
            app_subdir="go",
            local_package="packages/go",
            apply_override=_write_go_override,
            verify=(
                "go list -m -f '{{.Path}} => {{with .Replace}}{{.Dir}}{{end}}' "
                "github.com/xberg-io/tree-sitter-language-pack/packages/go"
            ),
            run="go test ./... -count=1",
        ),
        Target(
            name="python",
            app_subdir="python",
            local_package="packages/python",
            apply_override=_write_python_override,
            verify="uv lock && cat uv.lock",
            run="uv sync && uv run pytest",
        ),
        Target(
            name="ruby",
            app_subdir="ruby",
            local_package="packages/ruby",
            apply_override=_write_ruby_override,
            verify="bundle lock && cat Gemfile.lock",
            run="bundle install && bundle exec rspec spec/",
        ),
        Target(
            name="dart",
            app_subdir="dart",
            local_package="packages/dart",
            apply_override=_write_dart_override,
            verify="dart pub get --offline",
            run="dart pub get && dart test",
        ),
        Target(
            name="elixir",
            app_subdir="elixir",
            local_package="packages/elixir",
            apply_override=_write_elixir_override,
            verify="mix deps.get && mix deps",
            run="mix deps.get && mix test",
        ),
        Target(
            name="swift",
            app_subdir="swift_e2e",
            local_package="packages/swift",
            apply_override=_write_swift_override,
            verify="swift package show-dependencies --format json",
            run="swift test",
        ),
    )
}


def resolve_targets(selection: str | None) -> list[Target]:
    """Map a comma-separated `--lang` selection onto targets, rejecting unknown names."""
    if selection is None:
        return list(TARGETS.values())
    chosen = []
    for name in (part.strip() for part in selection.split(",") if part.strip()):
        if name in UNSUPPORTED:
            raise SystemExit(f"'{name}' has no prerelease override: {UNSUPPORTED[name]}")
        if name not in TARGETS:
            raise SystemExit(f"unknown target '{name}'; known: {', '.join(sorted(TARGETS))}")
        chosen.append(TARGETS[name])
    return chosen


def stage(target: Target) -> Path:
    """Copy the generated test app into `.prerelease/` and apply its resolution override."""
    source = ROOT / "test_apps" / target.app_subdir
    if not source.is_dir():
        raise SystemExit(f"missing test app {source}")
    local_package = ROOT / target.local_package
    if not local_package.is_dir():
        raise SystemExit(f"missing local package {local_package} for target '{target.name}'")

    staged = STAGING_ROOT / target.app_subdir
    if staged.exists():
        shutil.rmtree(staged)
    staged.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(source, staged, ignore=IGNORED_WHEN_STAGING, symlinks=True)
    target.apply_override(staged)
    return staged


def run_in(staged: Path, command: str) -> int:
    """Run one shell command inside a staged app, streaming its output."""
    print(f"$ (cd {staged.relative_to(ROOT)}) {command}", flush=True)
    return subprocess.run(command, shell=True, cwd=staged, check=False).returncode


def verify_resolution(target: Target, staged: Path) -> bool:
    """Resolve dependencies in `staged` and assert the local package path shows up.

    Resolver output is captured rather than streamed: several of these commands emit megabytes
    of JSON, and the only thing worth reporting is whether the pinned dependency now points at
    the working tree. A zero exit code alone is not enough — a resolver that quietly fell back
    to the registry also exits zero. ~keep
    """
    expected = str(ROOT / target.local_package)
    completed = subprocess.run(
        target.verify,
        shell=True,
        cwd=staged,
        check=False,
        capture_output=True,
        text=True,
    )
    output = completed.stdout + completed.stderr
    if completed.returncode != 0:
        print(f"{target.name}: FAIL (resolver exited {completed.returncode})")
        print(output[-2000:], file=sys.stderr)
        return False
    if expected not in output:
        print(f"{target.name}: FAIL (resolved output never mentions {expected})")
        print(output[-2000:], file=sys.stderr)
        return False
    print(f"{target.name}: resolves to {target.local_package}")
    return True


def command_status(_: argparse.Namespace) -> int:
    """Report which targets have a prerelease override and which do not, with reasons."""
    for target in TARGETS.values():
        present = "ok" if (ROOT / target.local_package).is_dir() else "MISSING"
        print(f"{target.name:<16} override -> {target.local_package} [{present}]")
    for name, reason in sorted(UNSUPPORTED.items()):
        print(f"{name:<16} no override: {reason}")
    return 0


def command_stage(args: argparse.Namespace) -> int:
    """Stage the selected targets without running them."""
    for target in resolve_targets(args.lang):
        staged = stage(target)
        print(f"staged {target.name} -> {staged.relative_to(ROOT)}")
    return 0


def command_verify(args: argparse.Namespace) -> int:
    """Stage, then run each target's resolve-only check so a redirect is proven before a build."""
    failed: list[str] = []
    for target in resolve_targets(args.lang):
        staged = stage(target)
        if target.verify is None:
            print(f"{target.name}: no resolve-only check; use `run`")
            continue
        if not verify_resolution(target, staged):
            failed.append(target.name)
    if failed:
        print(f"resolve check failed: {', '.join(failed)}", file=sys.stderr)
        return 1
    return 0


def command_run(args: argparse.Namespace) -> int:
    """Stage and run the selected targets' full test commands."""
    failed: list[str] = []
    for target in resolve_targets(args.lang):
        staged = stage(target)
        if run_in(staged, target.run) != 0:
            failed.append(target.name)
    if failed:
        print(f"prerelease test apps failed: {', '.join(failed)}", file=sys.stderr)
        return 1
    return 0


def command_clean(_: argparse.Namespace) -> int:
    """Remove the whole staging tree."""
    if STAGING_ROOT.exists():
        shutil.rmtree(STAGING_ROOT)
        print(f"removed {STAGING_ROOT.relative_to(ROOT)}")
    return 0


def main() -> int:
    """Parse arguments and dispatch to the requested subcommand."""
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    subparsers = parser.add_subparsers(dest="command", required=True)
    for name, handler in (
        ("status", command_status),
        ("stage", command_stage),
        ("verify", command_verify),
        ("run", command_run),
        ("clean", command_clean),
    ):
        sub = subparsers.add_parser(name, help=handler.__doc__)
        if name not in {"status", "clean"}:
            sub.add_argument("--lang", help="comma-separated target names (default: all supported)")
        sub.set_defaults(handler=handler)
    args = parser.parse_args()
    return int(args.handler(args))


if __name__ == "__main__":
    raise SystemExit(main())
