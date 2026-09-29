"""Publish pinned grammar redistribution notices (issue #191).

Builds a digest-pinned provenance record that maps every canonical language in
``sources/language_definitions.json`` to the exact repository, revision and
source directory its grammar was vendored from, retaining the applicable
LICENSE/NOTICE text. Identical license texts are stored once, keyed by their
SHA-256, and referenced by digest.

The committed output (``sources/grammar_notices.json``) is the source of truth
for the release artifacts, so release-time emission needs no network. The
network is only used by ``--update`` (regenerate the record) and ``--verify``
(re-fetch the pinned sources and fail on any digest mismatch).

Modes (mutually exclusive except ``--emit --verify``):

    --update   Re-fetch every language's license text and rewrite
               ``sources/grammar_notices.json``.
    --check    Validate coverage and digests from the committed files, offline.
    --emit     Write the versioned release artifacts from the committed record.
    --verify   Re-fetch the pinned sources and compare against the committed
               digests.

Exit codes:
    0  complete / valid
    1  unresolved coverage or digest mismatch
    2  usage or I/O error
"""

from __future__ import annotations

import argparse
import concurrent.futures
import hashlib
import json
import re
import subprocess
import sys
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import tomllib

_PROJECT_ROOT = Path(__file__).resolve().parent.parent
_DEFINITIONS_PATH = _PROJECT_ROOT / "sources" / "language_definitions.json"
_NOTICES_PATH = _PROJECT_ROOT / "sources" / "grammar_notices.json"
_OVERRIDES_PATH = _PROJECT_ROOT / "sources" / "grammar_notice_overrides.json"
_LICENSE_CACHE_PATH = _PROJECT_ROOT / "sources" / "license_cache.json"
_CARGO_TOML = _PROJECT_ROOT / "Cargo.toml"

SCHEMA = "tree-sitter-language-pack/grammar-notices/v1"
MAX_WORKERS = 16

_LICENSE_NAME_PREFIXES = ("LICENSE", "LICENCE", "COPYING", "COPYRIGHT", "NOTICE")
_GITHUB_PREFIXES = ("https://github.com/", "http://github.com/", "git@github.com:")
_GITLAB_PREFIXES = ("https://gitlab.com/", "http://gitlab.com/", "git@gitlab.com:")


class NoticesError(RuntimeError):
    """A recoverable coverage, provenance or I/O failure."""


@dataclass
class LanguageResult:
    """The resolved provenance for one language, or the reason it failed."""

    name: str
    entry: dict[str, Any] | None = None
    error: str | None = None
    texts: dict[str, str] = field(default_factory=dict)


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _strip_suffix(url: str) -> str:
    return url.rstrip("/").removesuffix(".git")


def _split_forge(url: str) -> tuple[str, str] | None:
    """Return ``(forge, owner/repo)`` for a GitHub or GitLab repository URL."""
    stripped = _strip_suffix(url)
    for prefix in _GITHUB_PREFIXES:
        if stripped.startswith(prefix):
            return "github", stripped[len(prefix) :]
    for prefix in _GITLAB_PREFIXES:
        if stripped.startswith(prefix):
            return "gitlab", stripped[len(prefix) :]
    return None


# Each entry maps all-of-these-lowercased-substrings to a SPDX id. Ordered so a
# more specific text wins over a generic one it contains.
_SPDX_RULES: tuple[tuple[tuple[str, ...], str], ...] = (
    (("apache license", "version 2.0"), "Apache-2.0"),
    (("mozilla public license version 2.0",), "MPL-2.0"),
    (("boost software license",), "BSL-1.0"),
    (("this is free and unencumbered software released into the public domain",), "Unlicense"),
    (("cc0 1.0",), "CC0-1.0"),
    (
        ("permission to use, copy, modify, and/or distribute this software for any purpose with or without fee",),
        "ISC",
    ),
    (("mit license",), "MIT"),
    (("permission is hereby granted, free of charge",), "MIT"),
    (("zlib license",), "Zlib"),
    (("altered source versions must be plainly marked",), "Zlib"),
)


def detect_spdx(text: str) -> str | None:
    """Best-effort SPDX identifier for a retained license text.

    Deliberately conservative: an unrecognised text returns ``None`` and the
    caller falls back to an inline declaration or requires an override, rather
    than guessing a license that is not there.
    """
    low = text.lower()
    if "gnu lesser general public license" in low:
        return "LGPL-3.0-only" if "version 3" in low else "LGPL-2.1-only"
    if "gnu general public license" in low:
        return "GPL-3.0-only" if "version 3" in low else "GPL-2.0-only"
    if "permission to use, copy, modify, and/or distribute this software" in low and "0bsd" in low:
        return "0BSD"
    if "redistribution and use in source and binary forms" in low:
        if "3. neither the name" in low or "3. the name" in low:
            return "BSD-3-Clause"
        return "BSD-2-Clause"
    for needles, spdx in _SPDX_RULES:
        if all(needle in low for needle in needles):
            return spdx
    return None


def _is_license_name(name: str) -> bool:
    if name in {"COPYING", "COPYRIGHT", "NOTICE"}:
        return True
    return name.upper().startswith(_LICENSE_NAME_PREFIXES)


def _cache_hint(owner_repo: str | None) -> str | None:
    """The SPDX value the mutable repo-wide cache holds, if any.

    Only ever used as a last-resort identifier hint for a language whose text
    *is* retained and pinned; never as textual evidence. See issue #191.
    """
    if owner_repo is None or not _LICENSE_CACHE_PATH.exists():
        return None
    try:
        cache = json.loads(_LICENSE_CACHE_PATH.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    value = cache.get(owner_repo)
    return value if isinstance(value, str) and value and value != "NOASSERTION" else None


def _run_gh(args: list[str]) -> tuple[int, str]:
    result = subprocess.run(["gh", *args], capture_output=True, text=True, check=False)
    return result.returncode, result.stdout


def _gh_list(owner_repo: str, directory: str | None, rev: str) -> list[str]:
    contents = f"contents/{directory}" if directory else "contents"
    code, out = _run_gh(["api", f"repos/{owner_repo}/{contents}?ref={rev}", "--jq", ".[].name"])
    if code != 0:
        return []
    return [line for line in out.splitlines() if line.strip()]


def _gh_raw(owner_repo: str, path: str, rev: str) -> bytes | None:
    code, out = _run_gh(
        [
            "api",
            f"repos/{owner_repo}/contents/{path}?ref={rev}",
            "-H",
            "Accept: application/vnd.github.raw",
        ]
    )
    return out.encode("utf-8") if code == 0 else None


def _gitlab_list(owner_repo: str, directory: str | None, rev: str) -> list[str]:
    project = urllib.parse.quote(owner_repo, safe="")
    url = f"https://gitlab.com/api/v4/projects/{project}/repository/tree?ref={rev}&per_page=100"
    if directory:
        url += f"&path={urllib.parse.quote(directory, safe='')}"
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            entries = json.loads(response.read().decode("utf-8"))
    except (urllib.error.URLError, TimeoutError, json.JSONDecodeError):
        return []
    return [entry["name"] for entry in entries if isinstance(entry, dict) and "name" in entry]


def _gitlab_raw(owner_repo: str, path: str, rev: str) -> bytes | None:
    url = f"https://gitlab.com/{owner_repo}/-/raw/{rev}/{path}"
    try:
        with urllib.request.urlopen(url, timeout=30) as response:
            return response.read()
    except (urllib.error.URLError, TimeoutError):
        return None


def _forge_list(forge: str, owner_repo: str, directory: str | None, rev: str) -> list[str]:
    return _gh_list(owner_repo, directory, rev) if forge == "github" else _gitlab_list(owner_repo, directory, rev)


def _forge_raw(forge: str, owner_repo: str, path: str, rev: str) -> bytes | None:
    return _gh_raw(owner_repo, path, rev) if forge == "github" else _gitlab_raw(owner_repo, path, rev)


def _resolve_spdx(inline: str | None, text: str, owner_repo: str | None) -> tuple[str, str]:
    if inline:
        return inline, "inline"
    detected = detect_spdx(text)
    if detected:
        return detected, "detected"
    hint = _cache_hint(owner_repo)
    if hint:
        return hint, "cache-hint"
    return "NOASSERTION", "unresolved"


def _local_license_files(local: str) -> list[tuple[str, bytes]]:
    # `local` is a repo-relative path such as ``grammars/graphql``.
    base = _PROJECT_ROOT / local
    return [
        (str(path.relative_to(_PROJECT_ROOT)), path.read_bytes())
        for path in sorted(base.glob("*"))
        if path.is_file() and _is_license_name(path.name)
    ]


_MANIFEST_NAMES = ("tree-sitter.json", "Cargo.toml", "pyproject.toml", "package.json")


def _parse_manifest_license(text: str) -> str | None:
    match = re.search(r'"license"\s*:\s*"([^"]+)"', text) or re.search(
        r'^\s*license\s*=\s*"([^"]+)"', text, re.MULTILINE
    )
    return match.group(1) if match else None


def _manifest_license(forge: str, owner_repo: str, directory: str | None, rev: str) -> tuple[str, bytes, str] | None:
    """Fallback when no LICENSE file exists: a pinned manifest's ``license`` grant."""
    for name in _MANIFEST_NAMES:
        for path in ([f"{directory}/{name}"] if directory else []) + [name]:
            data = _forge_raw(forge, owner_repo, path, rev)
            if data is None:
                continue
            spdx = _parse_manifest_license(data.decode("utf-8", errors="replace"))
            if spdx:
                return path, data, spdx
    return None


def _remote_license_files(forge: str, owner_repo: str, directory: str | None, rev: str) -> list[tuple[str, bytes]]:
    candidates: list[str] = []
    if directory:
        candidates += [
            f"{directory}/{name}" for name in _forge_list(forge, owner_repo, directory, rev) if _is_license_name(name)
        ]
    # Root-level notices still apply to a grammar vendored from a subdirectory.
    candidates += [name for name in _forge_list(forge, owner_repo, None, rev) if _is_license_name(name)]

    files: list[tuple[str, bytes]] = []
    seen: set[str] = set()
    for path in candidates:
        if path in seen:
            continue
        seen.add(path)
        data = _forge_raw(forge, owner_repo, path, rev)
        if data is not None:
            files.append((path, data))
    return files


def _discover_files(
    local: str | None, repo: str | None, rev: str | None, directory: str | None
) -> tuple[list[tuple[str, bytes]], str, str | None, str | None]:
    """Locate a language's pinned license files.

    Returns ``(files, resolution, owner_repo, manifest_spdx)``. ``resolution``
    names how the files were found; ``manifest_spdx`` is set only when the grant
    came from a pinned manifest rather than a LICENSE/NOTICE file.
    """
    if local:
        return _local_license_files(local), "local-license", None, None
    if not (repo and rev):
        return [], "no-source", None, None
    split = _split_forge(repo)
    if split is None:
        return [], "unsupported-forge", None, None
    forge, owner_repo = split
    files = _remote_license_files(forge, owner_repo, directory, rev)
    if files:
        return files, f"{forge}-license", owner_repo, None
    manifest = _manifest_license(forge, owner_repo, directory, rev)
    if manifest is not None:
        path, data, spdx = manifest
        return [(path, data)], f"{forge}-manifest", owner_repo, spdx
    return [], f"{forge}-license", owner_repo, None


def _collect_texts(files: list[tuple[str, bytes]]) -> tuple[dict[str, str], list[dict[str, str]], str | None]:
    """Decode retained files into a digest->text map plus their provenance list."""
    texts: dict[str, str] = {}
    license_files: list[dict[str, str]] = []
    for path, data in files:
        try:
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            return {}, [], f"{path}: not valid UTF-8"
        digest = _sha256(data)
        texts[digest] = text
        license_files.append({"path": path, "sha256": digest})
    return texts, sorted(license_files, key=lambda item: item["path"]), None


def resolve_language(name: str, definition: dict[str, Any], override: dict[str, Any] | None) -> LanguageResult:
    """Resolve one language's retained license files and SPDX identifier."""
    files, source_kind, owner_repo, manifest_spdx = _discover_files(
        definition.get("local"), definition.get("repo"), definition.get("rev"), definition.get("directory")
    )
    if not files:
        if override is None:
            return LanguageResult(name=name, error=f"{source_kind}: no license text at pinned source")
        override_path = _PROJECT_ROOT / override["file"]
        if not override_path.is_file():
            return LanguageResult(name=name, error=f"override file missing: {override['file']}")
        files = [(override["file"], override_path.read_bytes())]
        source_kind = "override"

    texts, license_files, error = _collect_texts(files)
    if error is not None:
        return LanguageResult(name=name, error=error)

    if source_kind == "override":
        spdx, spdx_source = override["spdx"], "override"
    elif manifest_spdx is not None:
        spdx, spdx_source = manifest_spdx, "manifest"
    else:
        spdx, spdx_source = _resolve_spdx(definition.get("license"), "\n".join(texts.values()), owner_repo)

    entry: dict[str, Any] = {
        "repo": definition.get("repo"),
        "rev": definition.get("rev"),
        "branch": definition.get("branch"),
        "directory": definition.get("directory"),
        "local": definition.get("local"),
        "resolution": source_kind,
        "spdx": spdx,
        "spdx_source": spdx_source,
        "license_files": license_files,
    }
    if source_kind == "override":
        entry["override"] = {
            "source": override.get("source"),
            "ref": override.get("ref"),
            "reason": override.get("reason"),
        }
    return LanguageResult(name=name, entry=entry, texts=texts)


def build(definitions: dict[str, Any], overrides: dict[str, Any], only: set[str] | None) -> dict[str, Any]:
    """Resolve every language concurrently into the committed notice record."""
    names = sorted(definitions) if not only else sorted(only)
    unknown = [name for name in names if name not in definitions]
    if unknown:
        raise NoticesError(f"--languages names not in definitions: {sorted(unknown)}")

    results: dict[str, LanguageResult] = {}
    with concurrent.futures.ThreadPoolExecutor(max_workers=MAX_WORKERS) as pool:
        futures = {pool.submit(resolve_language, name, definitions[name], overrides.get(name)): name for name in names}
        for future in concurrent.futures.as_completed(futures):
            result = future.result()
            results[result.name] = result

    failures = {name: result.error for name, result in results.items() if result.entry is None}
    if failures:
        detail = "\n".join(f"  {name}: {error}" for name, error in sorted(failures.items()))
        raise NoticesError(f"unresolved coverage for {len(failures)} language(s):\n{detail}")

    licenses: dict[str, str] = {}
    languages: dict[str, Any] = {}
    for name in sorted(results):
        languages[name] = results[name].entry
        for digest, text in results[name].texts.items():
            licenses.setdefault(digest, text)

    return {
        "schema": SCHEMA,
        "definitions_sha256": _sha256(_DEFINITIONS_PATH.read_bytes()),
        "languages": languages,
        "licenses": {digest: licenses[digest] for digest in sorted(licenses)},
    }


def _load_committed() -> dict[str, Any]:
    if not _NOTICES_PATH.is_file():
        raise NoticesError(f"{_NOTICES_PATH} is missing; run scripts/build_grammar_notices.py --update")
    return json.loads(_NOTICES_PATH.read_text(encoding="utf-8"))


def _validate(notices: dict[str, Any], definitions: dict[str, Any], overrides: dict[str, Any]) -> None:
    """Offline structural + coverage gate. Raises ``NoticesError`` on any gap."""
    if notices.get("schema") != SCHEMA:
        raise NoticesError(f"unexpected schema {notices.get('schema')!r}, expected {SCHEMA!r}")

    current_hash = _sha256(_DEFINITIONS_PATH.read_bytes())
    if notices.get("definitions_sha256") != current_hash:
        raise NoticesError(
            "sources/grammar_notices.json is stale: language_definitions.json changed. "
            "Re-run scripts/build_grammar_notices.py --update"
        )

    languages = notices.get("languages", {})
    licenses = notices.get("licenses", {})
    expected, produced = set(definitions), set(languages)
    if expected != produced:
        missing = sorted(expected - produced)
        extra = sorted(produced - expected)
        raise NoticesError(f"coverage mismatch: missing={missing} extra={extra}")

    used_digests: set[str] = set()
    for name in sorted(languages):
        entry = languages[name]
        if not entry.get("spdx"):
            raise NoticesError(f"{name}: empty SPDX identifier")
        files = entry.get("license_files") or []
        if not files:
            raise NoticesError(f"{name}: no retained license text")
        for item in files:
            digest = item.get("sha256")
            if digest not in licenses:
                raise NoticesError(f"{name}: license digest {digest!r} is not present under licenses")
            if _sha256(licenses[digest].encode("utf-8")) != digest:
                raise NoticesError(f"{name}: retained text for {digest!r} does not match its digest")
            used_digests.add(digest)

    orphaned = sorted(set(licenses) - used_digests)
    if orphaned:
        raise NoticesError(f"orphaned license texts (referenced by no language): {orphaned}")

    for name, override in sorted(overrides.items()):
        if name not in definitions:
            raise NoticesError(f"override for unknown language {name!r}")
        path = _PROJECT_ROOT / override["file"]
        if not path.is_file():
            raise NoticesError(f"override for {name!r} points at missing file {override['file']!r}")
        digest = _sha256(path.read_bytes())
        entry = languages.get(name, {})
        referenced = {item["sha256"] for item in entry.get("license_files", [])}
        if digest not in referenced:
            raise NoticesError(f"override text for {name!r} (sha256 {digest}) is not the retained copy")

    print(
        f"grammar notices: {len(languages)}/{len(definitions)} languages covered, "
        f"{len(licenses)} unique license text(s)"
    )


def _default_version() -> str:
    with _CARGO_TOML.open("rb") as handle:
        return tomllib.load(handle)["workspace"]["package"]["version"]


def _write_sha256(path: Path) -> None:
    digest = _sha256(path.read_bytes())
    path.with_name(path.name + ".sha256").write_text(f"{digest}  {path.name}\n", encoding="utf-8")


def _render_markdown(notices: dict[str, Any], version: str) -> str:
    languages = notices["languages"]
    licenses = notices["licenses"]
    used_by: dict[str, list[str]] = {}
    for name, entry in languages.items():
        for item in entry["license_files"]:
            used_by.setdefault(item["sha256"], []).append(name)

    lines = [
        f"# Grammar redistribution notices — v{version}",
        "",
        "This file accompanies the parser binaries and parser-source bundle. It maps every",
        "canonical language to the repository, revision and source directory its grammar was",
        "vendored from, and retains the applicable license text. Identical texts are listed",
        "once, keyed by SHA-256.",
        "",
        "## Languages",
        "",
        "| Language | Repository | Revision | Directory | SPDX | Source |",
        "| --- | --- | --- | --- | --- | --- |",
    ]
    for name in sorted(languages):
        entry = languages[name]
        repo = entry.get("repo") or f"in-repo:{entry.get('local')}"
        rev = (entry.get("rev") or "")[:12] or "—"
        directory = entry.get("directory") or "—"
        source = entry.get("resolution") or "—"
        lines.append(f"| `{name}` | {repo} | `{rev}` | `{directory}` | {entry['spdx']} | {source} |")

    lines += ["", "## License texts", ""]
    for digest in sorted(licenses):
        spdx = next(
            (languages[name]["spdx"] for name in used_by.get(digest, []) if name in languages),
            "NOASSERTION",
        )
        names = ", ".join(f"`{name}`" for name in sorted(used_by.get(digest, [])))
        lines += [
            f"### {spdx} — `{digest}`",
            "",
            f"Used by: {names}",
            "",
            "````text",
            licenses[digest].rstrip("\n"),
            "````",
            "",
        ]
    return "\n".join(lines) + "\n"


def emit(notices: dict[str, Any], version: str, output_dir: Path) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)
    provenance = dict(notices)
    provenance["version"] = version
    json_path = output_dir / f"grammar-provenance-{version}.json"
    json_path.write_text(
        json.dumps(provenance, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
        encoding="utf-8",
    )
    _write_sha256(json_path)

    markdown_path = output_dir / f"GRAMMAR_NOTICES-{version}.md"
    markdown_path.write_text(_render_markdown(notices, version), encoding="utf-8")
    _write_sha256(markdown_path)
    print(f"wrote {json_path.name}, {json_path.name}.sha256, {markdown_path.name}, {markdown_path.name}.sha256")


def verify(notices: dict[str, Any], definitions: dict[str, Any]) -> None:
    """Re-fetch pinned sources and compare every retained digest."""
    mismatches: list[str] = []
    for name in sorted(notices["languages"]):
        entry = notices["languages"][name]
        definition = definitions[name]
        if entry.get("override"):
            continue
        local = definition.get("local")
        split = _split_forge(definition["repo"]) if definition.get("repo") else None
        if not local and split is None:
            continue
        for item in entry["license_files"]:
            path = item["path"]
            if local or path.startswith("sources/"):
                local_path = _PROJECT_ROOT / path
                data = local_path.read_bytes() if local_path.is_file() else None
            else:
                forge, owner_repo = split
                data = _forge_raw(forge, owner_repo, path, definition["rev"])
            if data is None:
                mismatches.append(f"{name}: {path} no longer resolvable at {definition.get('rev')}")
            elif _sha256(data) != item["sha256"]:
                mismatches.append(f"{name}: {path} digest changed")
    if mismatches:
        raise NoticesError("digest verification failed:\n" + "\n".join(f"  {item}" for item in mismatches))
    print(f"grammar notices: verified {len(notices['languages'])} language(s) against pinned sources")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--update",
        action="store_true",
        help="re-fetch and rewrite sources/grammar_notices.json",
    )
    parser.add_argument("--check", action="store_true", help="validate the committed record offline")
    parser.add_argument(
        "--emit",
        action="store_true",
        help="write release artifacts from the committed record",
    )
    parser.add_argument("--verify", action="store_true", help="re-fetch and compare committed digests")
    parser.add_argument("--output-dir", type=Path, default=_PROJECT_ROOT / "dist")
    parser.add_argument("--version", default=None, help="version stamped into --emit artifacts")
    parser.add_argument("--languages", default=None, help="comma-separated subset for --update")
    args = parser.parse_args(argv)

    if not (args.update or args.check or args.emit or args.verify):
        parser.error("one of --update, --check, --emit, or --verify is required")
    if args.update and (args.emit or args.verify):
        parser.error("--update cannot be combined with --emit or --verify")

    definitions = json.loads(_DEFINITIONS_PATH.read_text(encoding="utf-8"))
    overrides = json.loads(_OVERRIDES_PATH.read_text(encoding="utf-8"))
    only = {name for name in args.languages.split(",") if name} if args.languages else None

    try:
        notices: dict[str, Any] | None = None
        if args.update:
            notices = build(definitions, overrides, only)
            _validate(notices, definitions, overrides)
            _NOTICES_PATH.write_text(
                json.dumps(notices, indent=2, ensure_ascii=False, sort_keys=True) + "\n",
                encoding="utf-8",
            )
            print(f"wrote {_NOTICES_PATH.relative_to(_PROJECT_ROOT)}")
        if (args.check or args.emit or args.verify) and notices is None:
            notices = _load_committed()
            _validate(notices, definitions, overrides)
        if args.verify or args.emit:
            assert notices is not None  # set by the load branch above
        if args.verify:
            verify(notices, definitions)
        if args.emit:
            emit(notices, args.version or _default_version(), args.output_dir)
    except NoticesError as error:
        print(f"error: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
