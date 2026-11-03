"""Verify that all included tree-sitter grammars use permissive licenses.

Reads language_definitions.json and resolves an SPDX identifier for every
grammar, in order: the inline ``license`` field, the GitHub License API, then
the ``license`` field of the repository's own manifests (``tree-sitter.json``,
``Cargo.toml``, ``pyproject.toml``, ``package.json``) at the pinned revision.

The manifest fallback exists because the GitHub License API only reports a
license when the repository has a LICENSE *file*; several grammars ship no such
file but do declare ``license = "MIT"`` in a manifest, which is itself a
license grant.

Fails if any grammar is copyleft, or if no license can be resolved at all.

Usage:
    python scripts/lint_grammar_licenses.py [--update-cache] [--no-cache]

Exit codes:
    0  All grammars have permissive licenses
    1  One or more grammars have disallowed or undetected licenses
"""

import json
import subprocess
import sys
import tomllib
from pathlib import Path
from typing import Any

_project_root = Path(__file__).parent.parent
_definitions_path = _project_root / "sources" / "language_definitions.json"
_cache_path = _project_root / "sources" / "license_cache.json"

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


MANIFEST_FILENAMES: tuple[str, ...] = ("tree-sitter.json", "Cargo.toml", "pyproject.toml", "package.json")
"""Manifests probed for a `license` field, most authoritative first.

`package.json` is last because the tree-sitter CLI scaffolds it with a default
`"license": "ISC"` that frequently disagrees with the grammar's real license
declared in `Cargo.toml` / `tree-sitter.json` (e.g. brightscript, cooklang). ~keep
"""


def _gh_api_raw_file(owner_repo: str, path: str, rev: str | None) -> str | None:
    """Fetch a file's raw contents from a GitHub repository at a revision."""
    endpoint = f"repos/{owner_repo}/contents/{path}"
    if rev:
        endpoint = f"{endpoint}?ref={rev}"
    try:
        result = subprocess.run(
            ["gh", "api", endpoint, "-H", "Accept: application/vnd.github.raw"],
            capture_output=True,
            text=True,
            timeout=15,
            check=False,
        )
    except (subprocess.TimeoutExpired, FileNotFoundError):
        return None
    return result.stdout if result.returncode == 0 else None


def _license_from_json_manifest(text: str) -> str | None:
    """Extract an SPDX id from a `tree-sitter.json` / `package.json` body."""
    try:
        data: Any = json.loads(text)
    except json.JSONDecodeError:
        return None
    if not isinstance(data, dict):
        return None
    declared = data.get("license")
    if isinstance(declared, str):
        return declared.strip() or None
    # ~keep Legacy npm shape: {"license": {"type": "MIT", "url": ...}}.
    if isinstance(declared, dict) and isinstance(declared.get("type"), str):
        return declared["type"].strip() or None
    return None


def _license_from_toml_manifest(text: str, filename: str) -> str | None:
    """Extract an SPDX id from a `Cargo.toml` / `pyproject.toml` body."""
    try:
        data = tomllib.loads(text)
    except tomllib.TOMLDecodeError:
        return None
    table = "package" if filename == "Cargo.toml" else "project"
    section = data.get(table)
    if not isinstance(section, dict):
        return None
    declared = section.get("license")
    if isinstance(declared, str):
        return declared.strip() or None
    # ~keep Pre-PEP-639 pyproject shape: [project.license] with `text = "MIT"`.
    if isinstance(declared, dict) and isinstance(declared.get("text"), str):
        return declared["text"].strip() or None
    return None


def _manifest_license(owner_repo: str, rev: str | None, directory: str | None) -> str | None:
    """Resolve an SPDX id from the repository's own manifests.

    A manifest `license` field is a license grant even when the repository ships
    no LICENSE file, which is the only thing the GitHub License API can see. ~keep

    Args:
        owner_repo: The `owner/name` GitHub slug.
        rev: The pinned revision, or None to read the default branch.
        directory: Grammar subdirectory within the repo, probed before the root.

    Returns:
        The declared SPDX identifier, or None if no manifest declares one.
    """
    candidates = [f"{directory}/{name}" for name in MANIFEST_FILENAMES] if directory else []
    candidates += list(MANIFEST_FILENAMES)

    for path in candidates:
        body = _gh_api_raw_file(owner_repo, path, rev)
        if body is None:
            continue
        filename = path.rsplit("/", maxsplit=1)[-1]
        declared = (
            _license_from_json_manifest(body)
            if filename.endswith(".json")
            else _license_from_toml_manifest(body, filename)
        )
        if declared:
            return declared
    return None


def _load_cache() -> dict[str, str]:
    if _cache_path.exists():
        result: dict[str, str] = json.loads(_cache_path.read_text())
        return result
    return {}


def _save_cache(cache: dict[str, str]) -> None:
    _cache_path.write_text(json.dumps(cache, indent=2, sort_keys=True) + "\n")


def _classify_license(spdx: str | None) -> str:
    """Return 'permissive', 'copyleft', or 'unknown'.

    Handles SPDX choice expressions ("MIT OR Apache-2.0", "MIT/Apache-2.0"):
    a dual license is permissive as long as one alternative is, because the
    consumer picks the alternative. ~keep
    """
    if not spdx or spdx == "NOASSERTION":
        return "unknown"

    alternatives = [part.strip().strip("()") for part in spdx.replace("/", " OR ").split(" OR ")]
    alternatives = [part for part in alternatives if part]
    if any(part in PERMISSIVE_LICENSES for part in alternatives):
        return "permissive"
    if any(part in COPYLEFT_LICENSES for part in alternatives):
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
        # Trust an inline `license` field on ANY entry, not just in-repo (local)
        # grammars. It is the declared SPDX documented in the grammar's own LICENSE
        # and is still validated against PERMISSIVE_LICENSES below. This is required
        # for two cases the GitHub License API cannot resolve correctly: ~keep
        #   - vendored `local` grammars with no upstream repo, and
        #   - non-GitHub forges (GitLab/Codeberg/sourcehut) the `gh api` cannot reach,
        #     plus monorepos whose per-grammar subdirectory license differs from the
        #     repo-level license GitHub reports (e.g. ebnf: MIT subdir under a GPL repo).
        declared = info.get("license")
        if declared:
            category = _classify_license(declared)
            if category == "permissive":
                permissive.append(lang)
            elif category == "copyleft":
                rejected.append((lang, declared))
            else:
                source = info.get("local") or info.get("repo", "")
                unknown.append((lang, f"{source} (declared: {declared})"))
            continue

        if info.get("local"):
            unknown.append((lang, f"local:{info['local']} (no declared license)"))
            continue

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
            if not spdx:
                # ~keep No LICENSE file for the API to read; the manifests still grant a license.
                spdx = _manifest_license(owner_repo, info.get("rev"), info.get("directory"))
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

    if unknown or errors:
        print(
            "\nFAILED: could not resolve a permissive license for "
            f"{len(unknown) + len(errors)} grammar(s). Add a LICENSE file or a `license` field upstream, "
            "record the SPDX id in the entry's `license` field in sources/language_definitions.json, "
            "or drop the grammar. Run with --update-cache to refresh cached lookups."
        )
        return 1

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
