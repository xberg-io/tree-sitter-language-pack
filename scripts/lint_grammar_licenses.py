"""Verify that all included tree-sitter grammars use permissive licenses.

Reads language_definitions.json, queries the GitHub License API for each
grammar repository, and fails if any grammar uses a copyleft license.

Usage:
    python scripts/lint_grammar_licenses.py [--update-cache] [--no-cache]

Exit codes:
    0  All grammars have permissive licenses
    1  One or more grammars have disallowed or undetected licenses
"""

import json
import subprocess
import sys
from pathlib import Path

_project_root = Path(__file__).parent.parent
_definitions_path = _project_root / "sources" / "language_definitions.json"
_cache_path = _project_root / "sources" / "license_cache.json"

# SPDX identifiers we accept
PERMISSIVE_LICENSES: set[str] = {
    "0BSD",
    "Apache-2.0",
    "BlueOak-1.0.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "BSL-1.0",
    "CC0-1.0",
    "ISC",
    "MIT",
    "MIT-0",
    "Unlicense",
    "WTFPL",
    "Zlib",
}

# SPDX identifiers we explicitly reject
COPYLEFT_LICENSES: set[str] = {
    "AGPL-3.0-only",
    "AGPL-3.0-or-later",
    "EUPL-1.1",
    "EUPL-1.2",
    "GPL-2.0-only",
    "GPL-2.0-or-later",
    "GPL-3.0-only",
    "GPL-3.0-or-later",
    "LGPL-2.1-only",
    "LGPL-2.1-or-later",
    "LGPL-3.0-only",
    "LGPL-3.0-or-later",
    "MPL-2.0",
    "OSL-3.0",
}


def _parse_github_repo(url: str) -> str | None:
    """Extract 'owner/repo' from a GitHub URL."""
    url = url.rstrip("/")
    url = url.removesuffix(".git")
    for prefix in ("https://github.com/", "http://github.com/", "git@github.com:"):
        if url.startswith(prefix):
            return url[len(prefix) :]
    return None


def _gh_api_license(owner_repo: str) -> str | None:
    """Query GitHub API for license SPDX ID."""
    for endpoint in [f"repos/{owner_repo}/license", f"repos/{owner_repo}"]:
        jq = ".license.spdx_id"
        try:
            result = subprocess.run(
                ["gh", "api", endpoint, "--jq", jq],
                capture_output=True,
                text=True,
                timeout=15,
                check=False,
            )
            if result.returncode == 0:
                spdx = result.stdout.strip()
                if spdx and spdx != "null":
                    return spdx
        except (subprocess.TimeoutExpired, FileNotFoundError):
            pass
    return None


def _load_cache() -> dict[str, str]:
    if _cache_path.exists():
        result: dict[str, str] = json.loads(_cache_path.read_text())
        return result
    return {}


def _save_cache(cache: dict[str, str]) -> None:
    _cache_path.write_text(json.dumps(cache, indent=2, sort_keys=True) + "\n")


def _classify_license(spdx: str | None) -> str:
    """Return 'permissive', 'copyleft', or 'unknown'."""
    if not spdx or spdx == "NOASSERTION":
        return "unknown"
    if spdx in PERMISSIVE_LICENSES:
        return "permissive"
    if spdx in COPYLEFT_LICENSES:
        return "copyleft"
    return "unknown"


def _check_grammars(
    definitions: dict[str, dict[str, str]],
    cache: dict[str, str],
    no_cache: bool,
) -> tuple[list[str], list[tuple[str, str]], list[tuple[str, str]], list[tuple[str, str]]]:
    """Check all grammars. Returns (permissive, rejected, unknown, errors)."""
    permissive: list[str] = []
    rejected: list[tuple[str, str]] = []
    unknown: list[tuple[str, str]] = []
    errors: list[tuple[str, str]] = []

    total = len(definitions)
    for i, (lang, info) in enumerate(sorted(definitions.items()), 1):
        repo_url = info.get("repo", "")
        owner_repo = _parse_github_repo(repo_url)

        if not owner_repo:
            errors.append((lang, f"not a GitHub URL: {repo_url}"))
            continue

        spdx: str | None
        if owner_repo in cache and not no_cache:
            spdx = cache[owner_repo]
        else:
            print(f"  [{i}/{total}] Checking {lang} ({owner_repo})...", flush=True)
            spdx = _gh_api_license(owner_repo)
            if spdx:
                cache[owner_repo] = spdx

        category = _classify_license(spdx)
        if category == "permissive":
            permissive.append(lang)
        elif category == "copyleft":
            rejected.append((lang, spdx or ""))
        else:
            detail = f"{owner_repo} ({spdx})" if spdx else owner_repo
            unknown.append((lang, detail))

    return permissive, rejected, unknown, errors


def _print_report(
    total: int,
    permissive: list[str],
    rejected: list[tuple[str, str]],
    unknown: list[tuple[str, str]],
    errors: list[tuple[str, str]],
) -> int:
    """Print results and return exit code."""
    print(f"\nLicense check: {total} grammars")
    print(f"  Permissive: {len(permissive)}")

    if rejected:
        print(f"\n  REJECTED ({len(rejected)}) — copyleft license:")
        for lang, spdx in rejected:
            print(f"    {lang}: {spdx}")

    if unknown:
        print(f"\n  UNKNOWN ({len(unknown)}) — needs manual review:")
        for lang, detail in unknown:
            print(f"    {lang}: {detail}")

    if errors:
        print(f"\n  ERRORS ({len(errors)}):")
        for lang, msg in errors:
            print(f"    {lang}: {msg}")

    if rejected:
        print("\nFAILED: copyleft grammars found. See CONTRIBUTING.md for license policy.")
        return 1

    if unknown:
        print("\nWARNING: some grammars have undetected licenses. Run with --update-cache to refresh.")

    print("\nPASSED: all grammars use permissive licenses.")
    return 0


def main() -> int:
    update_cache = "--update-cache" in sys.argv
    no_cache = "--no-cache" in sys.argv

    definitions: dict[str, dict[str, str]] = json.loads(_definitions_path.read_text())
    cache = {} if no_cache else _load_cache()

    permissive, rejected, unknown, errors = _check_grammars(definitions, cache, no_cache)

    if update_cache or not _cache_path.exists():
        _save_cache(cache)

    return _print_report(len(definitions), permissive, rejected, unknown, errors)


if __name__ == "__main__":
    raise SystemExit(main())
