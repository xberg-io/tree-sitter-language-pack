"""Audit every tracked independent Cargo resolution, including excluded bindings."""

import json
import subprocess
import sys
from pathlib import Path

import tomllib

LOCKFILES = (
    "Cargo.lock",
    "e2e/rust/Cargo.lock",
    "packages/elixir/native/tree_sitter_language_pack_nif/Cargo.lock",
    "packages/ruby/ext/ts_pack_core_rb/native/Cargo.lock",
    "test_apps/rust/Cargo.lock",
)


def validate_inventory(root: Path) -> None:
    result = subprocess.run(
        ["git", "ls-files", "-z", "--", "Cargo.lock", "**/Cargo.lock"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True,
    )
    actual = set(result.stdout.rstrip("\0").split("\0"))
    if actual != set(LOCKFILES):
        raise ValueError(f"Cargo lockfile inventory differs: expected {sorted(LOCKFILES)}, found {sorted(actual)}")


def package_count(path: Path) -> int:
    packages = tomllib.loads(path.read_text()).get("package")
    if not isinstance(packages, list) or not packages:
        raise ValueError(f"{path}: lockfile must contain nonzero packages")
    if any(
        not isinstance(package, dict) or not package.get("name") or not package.get("version") for package in packages
    ):
        raise ValueError(f"{path}: lockfile contains an invalid package")
    return len(packages)


def audit_one(root: Path, lock: str) -> dict:
    expected = package_count(root / lock)
    result = subprocess.run(
        ["cargo", "audit", "--file", lock, "--json"], cwd=root, capture_output=True, text=True, check=False
    )
    if result.returncode != 0:
        raise ValueError(f"{lock}: cargo audit failed with exit {result.returncode}: {result.stderr.strip()}")
    report = json.loads(result.stdout)
    if not isinstance(report, dict) or not isinstance(report.get("lockfile"), dict):
        raise ValueError(f"{lock}: cargo audit returned malformed lockfile results")
    count = report.get("lockfile", {}).get("dependency-count")
    if type(count) is not int or count != expected:
        raise ValueError(f"{lock}: audited dependency count {count!r} does not match {expected} lockfile packages")
    vulnerabilities = report.get("vulnerabilities")
    if not isinstance(vulnerabilities, dict) or (
        type(vulnerabilities.get("count")) is not int
        or vulnerabilities["count"] != 0
        or vulnerabilities.get("found") is not False
        or vulnerabilities.get("list") != []
    ):
        raise ValueError(f"{lock}: cargo audit reported vulnerabilities or malformed vulnerability results")
    return {"lock": lock, "packages": count, "vulnerabilities": 0}


def audit_all(root: Path) -> list[dict]:
    validate_inventory(root)
    return [audit_one(root, lock) for lock in LOCKFILES]


def main() -> int:
    try:
        rows = audit_all(Path(__file__).resolve().parents[2])
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(f"Cargo security audit failed: {error}", file=sys.stderr)
        return 1
    print(json.dumps({"lockfiles_audited": len(rows), "rows": rows}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
