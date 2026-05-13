#!/usr/bin/env python3
"""Vendor ts-pack-core into Ruby extension crate for isolated builds.

Used by: .task/ruby.yml - vendor task

This script:
1. Reads workspace.dependencies from root Cargo.toml
2. Copies ts-pack-core to crates/ts-pack-ruby/vendor/
3. Replaces workspace = true with explicit versions
4. Generates vendor/Cargo.toml with proper workspace setup
"""

import os
import re
import shutil
import sys
from collections.abc import Callable
from pathlib import Path

if sys.version_info >= (3, 11):
    import tomllib
else:
    import tomli as tomllib  # type: ignore[import-not-found, unused-ignore]


def get_repo_root() -> Path:
    """Get repository root directory."""
    repo_root_env = os.environ.get("PROJECT_ROOT")
    if repo_root_env:
        return Path(repo_root_env)
    script_dir = Path(__file__).parent.absolute()
    return (script_dir / ".." / ".." / "..").resolve()


def read_toml(path: Path) -> dict[str, object]:
    """Read TOML file."""
    with path.open("rb") as f:
        result: dict[str, object] = tomllib.load(f)
        return result


def get_workspace_deps(repo_root: Path) -> dict[str, object]:
    """Extract workspace.dependencies from root Cargo.toml."""
    data = read_toml(repo_root / "Cargo.toml")
    workspace = data.get("workspace")
    if not isinstance(workspace, dict):
        return {}
    deps = workspace.get("dependencies")
    if not isinstance(deps, dict):
        return {}
    return deps


def get_workspace_version(repo_root: Path) -> str:
    """Extract version from workspace.package."""
    data = read_toml(repo_root / "Cargo.toml")
    workspace = data.get("workspace")
    if not isinstance(workspace, dict):
        return "0.0.0"
    package = workspace.get("package")
    if not isinstance(package, dict):
        return "0.0.0"
    version = package.get("version", "0.0.0")
    return str(version)


def format_dependency(name: str, dep_spec: object) -> str:
    """Format a dependency spec for Cargo.toml."""
    if isinstance(dep_spec, str):
        return f'{name} = "{dep_spec}"'
    if isinstance(dep_spec, dict):
        parts: list[str] = []
        if pkg := dep_spec.get("package"):
            parts.append(f'package = "{pkg}"')
        if path := dep_spec.get("path"):
            parts.append(f'path = "{path}"')
        if version := dep_spec.get("version", ""):
            parts.append(f'version = "{version}"')
        if features := dep_spec.get("features", []):
            features_str = ", ".join(f'"{f}"' for f in features)
            parts.append(f"features = [{features_str}]")
        if dep_spec.get("default-features") is False:
            parts.append("default-features = false")
        if dep_spec.get("optional") is True:
            parts.append("optional = true")
        return f"{name} = {{ {', '.join(parts)} }}"
    return f'{name} = "{dep_spec}"'


def _parse_toml_fields(raw: str) -> dict[str, str]:
    """Parse comma-separated TOML inline table fields, respecting brackets."""
    fields: dict[str, str] = {}
    depth = 0
    current = ""
    for char in raw:
        if char == "[":
            depth += 1
            current += char
        elif char == "]":
            depth -= 1
            current += char
        elif char == "," and depth == 0:
            _add_field(current.strip(), fields)
            current = ""
        else:
            current += char
    _add_field(current.strip(), fields)
    return fields


def _add_field(field: str, fields: dict[str, str]) -> None:
    if field and "=" in field:
        key, val = field.split("=", 1)
        fields[key.strip()] = val.strip()


def _make_replacer(dep_name: str, dep_spec: object) -> Callable[[re.Match[str]], str]:
    """Create a regex replacer that merges workspace fields with crate-specific fields."""

    def replace_with_fields(match: re.Match[str]) -> str:
        other_fields_str = match.group(1).strip()
        base_spec = format_dependency(dep_name, dep_spec)

        if " = { " not in base_spec:
            version_val = base_spec.split(" = ", 1)[1].strip('"')
            spec_part = f'version = "{version_val}"'
        else:
            spec_part = base_spec.split(" = { ", 1)[1].rstrip("} ").rstrip("}")

        workspace_fields = _parse_toml_fields(spec_part)
        crate_fields = _parse_toml_fields(other_fields_str)
        merged = {**workspace_fields, **crate_fields}
        merged_spec = ", ".join(f"{k} = {v}" for k, v in merged.items())
        return f"{dep_name} = {{ {merged_spec} }}"

    return replace_with_fields


def replace_workspace_deps_in_toml(toml_path: Path, workspace_deps: dict[str, object]) -> None:
    """Replace workspace = true with explicit versions in a Cargo.toml file."""
    content = toml_path.read_text()

    for name, dep_spec in workspace_deps.items():
        pattern1 = rf"^{re.escape(name)} = \{{ workspace = true \}}$"
        content = re.sub(pattern1, format_dependency(name, dep_spec), content, flags=re.MULTILINE)

        pattern2 = rf"^{re.escape(name)} = \{{ workspace = true, (.+?) \}}$"
        content = re.sub(pattern2, _make_replacer(name, dep_spec), content, flags=re.MULTILINE | re.DOTALL)

    toml_path.write_text(content)


def _clean_vendor(vendor_base: Path, core_vendor: Path) -> None:
    """Clean vendor directory and build artifacts."""
    if core_vendor.exists():
        shutil.rmtree(core_vendor)
    vendor_cargo = vendor_base / "Cargo.toml"
    if vendor_cargo.exists():
        vendor_cargo.unlink()
    print("Cleaned vendor directory")


def _update_workspace_inheritance(core_toml: Path, core_version: str) -> None:
    """Replace workspace inheritance fields with explicit values."""
    content = core_toml.read_text()
    replacements = {
        r"^version\.workspace = true$": f'version = "{core_version}"',
        r"^edition\.workspace = true$": 'edition = "2024"',
        r"^license\.workspace = true$": 'license = "MIT"',
        r"^authors\.workspace = true$": 'authors = ["Na\'aman Hirschfeld <naaman@kreuzberg.dev>"]',
        r"^repository\.workspace = true$": 'repository = "https://github.com/kreuzberg-dev/tree-sitter-language-pack"',
    }
    for pattern, replacement in replacements.items():
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
    core_toml.write_text(content)


def main() -> None:
    """Main vendoring function."""
    repo_root = get_repo_root()
    workspace_deps = get_workspace_deps(repo_root)
    core_version = get_workspace_version(repo_root)

    print("=== Vendoring ts-pack-core into Ruby crate ===")
    print(f"Core version: {core_version}")

    vendor_base = repo_root / "crates" / "ts-pack-ruby" / "vendor"
    core_vendor = vendor_base / "ts-pack-core"

    _clean_vendor(vendor_base, core_vendor)
    vendor_base.mkdir(parents=True, exist_ok=True)

    src = repo_root / "crates" / "ts-pack-core"
    if not src.exists():
        print(f"Error: Source not found: {src}", file=sys.stderr)
        sys.exit(1)

    shutil.copytree(src, core_vendor)
    print("Copied ts-pack-core")

    for artifact_dir in ("target", ".fastembed_cache"):
        artifact = core_vendor / artifact_dir
        if artifact.exists():
            shutil.rmtree(artifact)
    for pattern in ("*.swp", "*.bak", "*.tmp", "*~"):
        for f in core_vendor.rglob(pattern):
            f.unlink()
    print("Cleaned build artifacts")

    core_toml = core_vendor / "Cargo.toml"
    if core_toml.exists():
        _update_workspace_inheritance(core_toml, core_version)
        replace_workspace_deps_in_toml(core_toml, workspace_deps)
        print("Updated ts-pack-core/Cargo.toml")

    deps_lines = [format_dependency(n, s) for n, s in sorted(workspace_deps.items())]
    vendor_cargo = vendor_base / "Cargo.toml"
    vendor_cargo.write_text(
        f'[workspace]\nmembers = ["ts-pack-core"]\n\n'
        f"[workspace.package]\n"
        f'version = "{core_version}"\n'
        f'edition = "2024"\n'
        f'license = "MIT"\n'
        f'authors = ["Na\'aman Hirschfeld <naaman@kreuzberg.dev>"]\n'
        f'repository = "https://github.com/kreuzberg-dev/tree-sitter-language-pack"\n'
        f'homepage = "https://kreuzberg.dev"\n\n'
        f"[workspace.dependencies]\n"
        f"{chr(10).join(deps_lines)}\n"
    )
    print("Generated vendor/Cargo.toml")

    ruby_toml = repo_root / "crates" / "ts-pack-ruby" / "Cargo.toml"
    if ruby_toml.exists():
        content = ruby_toml.read_text()
        content = re.sub(
            r"tree-sitter-language-pack = \{ workspace = true",
            'tree-sitter-language-pack = { path = "vendor/ts-pack-core"',
            content,
        )
        ruby_toml.write_text(content)
        print("Updated ts-pack-ruby/Cargo.toml to use vendored core")

    print(f"\nVendoring complete (version: {core_version})")


if __name__ == "__main__":
    main()
