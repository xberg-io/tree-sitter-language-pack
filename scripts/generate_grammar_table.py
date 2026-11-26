#!/usr/bin/env python3
"""Generate a markdown table of all supported grammars from language_definitions.json.

Two modes are supported:

- ``languages`` (default): a ``Language | Extensions | Repository`` table written
  to ``docs-site/src/content/docs/languages.md``.
- ``queries``: a per-grammar query-bundle table written to
  ``templates/readme/partials/grammar_table.md``, with one row per grammar and a
  column per standard query type (``highlights``, ``injections``, ``locals``,
  ``indents``, ``folds``, ``tags``). Presence is determined live from
  ``parsers/<lang>/queries/<type>.scm``.

Supports --stdout to print to stdout instead of writing to disk.
"""

import argparse
import json
import re
import sys
from pathlib import Path
from types import MappingProxyType

DEFAULT_TARGET_ABI = 14

# Base URL for linking in-repo (vendored) grammar directories. A GitHub URL keeps
# the link valid regardless of where the generated table is rendered (docs site,
# README partials), unlike a repo-relative path.
VENDORED_GRAMMAR_BASE_URL = "https://github.com/xberg-io/tree-sitter-language-pack/tree/main"

ABI_EXEMPT_PARSER_BYTES = 24 * 1024 * 1024

_ABI_MARKER_SCAN_BYTES = 4096

_LANGUAGE_VERSION_RE = re.compile(r"LANGUAGE_VERSION\s+(\d+)")


_DISPLAY_NAMES = MappingProxyType(
    {
        "al": "AL",
        "asm": "ASM",
        "bsl": "BSL",
        "capnp": "Cap'n Proto",
        "css": "CSS",
        "csv": "CSV",
        "cuda": "CUDA",
        "dart": "Dart",
        "dtd": "DTD",
        "ecma": "ECMAScript",
        "elisp": "Emacs Lisp",
        "elm": "Elm",
        "fsharp": "F#",
        "gitattributes": "gitattributes",
        "gitcommit": "gitcommit",
        "gitignore": "gitignore",
        "glsl": "GLSL",
        "gn": "GN",
        "gnuplot": "gnuplot",
        "hcl": "HCL",
        "hlsl": "HLSL",
        "html": "HTML",
        "http": "HTTP",
        "java": "Java",
        "javascript": "JavaScript",
        "jinja": "Jinja",
        "jinja2": "Jinja2",
        "json": "JSON",
        "json5": "JSON5",
        "jsonnet": "Jsonnet",
        "jsx": "JSX",
        "kdl": "KDL",
        "latex": "LaTeX",
        "llvm": "LLVM",
        "lua": "Lua",
        "make": "Make",
        "matlab": "MATLAB",
        "meson": "Meson",
        "nasm": "NASM",
        "nginx": "nginx",
        "nim": "Nim",
        "nix": "Nix",
        "ocaml": "OCaml",
        "ocaml_interface": "OCaml Interface",
        "pascal": "Pascal",
        "perl": "Perl",
        "pgn": "PGN",
        "php": "PHP",
        "php_only": "PHP (only)",
        "po": "PO",
        "pod": "POD",
        "prisma": "Prisma",
        "proto": "Protocol Buffers",
        "psv": "PSV",
        "puppet": "Puppet",
        "purescript": "PureScript",
        "python": "Python",
        "ql": "QL",
        "qmljs": "QML",
        "r": "R",
        "rbs": "RBS",
        "re2c": "re2c",
        "regex": "Regex",
        "rego": "Rego",
        "rst": "reStructuredText",
        "ruby": "Ruby",
        "rust": "Rust",
        "scala": "Scala",
        "scss": "SCSS",
        "smali": "Smali",
        "smithy": "Smithy",
        "solidity": "Solidity",
        "sql": "SQL",
        "ssh_client_config": "SSH Client Config",
        "svelte": "Svelte",
        "swift": "Swift",
        "tcl": "Tcl",
        "terraform": "Terraform",
        "toml": "TOML",
        "tsv": "TSV",
        "tsx": "TSX",
        "typescript": "TypeScript",
        "udev": "udev",
        "unison": "Unison",
        "verilog": "Verilog",
        "vhdl": "VHDL",
        "vim": "Vim",
        "viml": "VimL",
        "vue": "Vue",
        "wgsl": "WGSL",
        "xml": "XML",
        "yaml": "YAML",
        "zig": "Zig",
    }
)


def _display_name(lang_id: str) -> str:
    """Convert a language identifier to a human-readable display name.

    Args:
        lang_id: Snake_case or lowercase language identifier.

    Returns:
        Title-cased display name with known acronyms preserved.
    """

    if lang_id in _DISPLAY_NAMES:
        return _DISPLAY_NAMES[lang_id]

    return " ".join(word.capitalize() for word in lang_id.replace("-", "_").split("_"))


def _repo_link(repo_url: str) -> str:
    """Format a GitHub URL as a markdown link showing only the org/repo path.

    Args:
        repo_url: Full repository URL.

    Returns:
        Markdown link string, e.g. [org/repo](https://...).
    """
    if not repo_url:
        return "—"
    parts = repo_url.rstrip("/").split("/")
    label = "/".join(parts[-2:]) if len(parts) >= 2 else repo_url
    return f"[{label}]({repo_url})"


def _source_cell(entry: dict[str, object]) -> str:
    """Render the source column for a grammar entry.

    In-repo (local) grammars link to their in-tree directory; upstream grammars
    link to their GitHub repository.
    """
    local = entry.get("local")
    if isinstance(local, str) and local:
        return f"[{local}]({VENDORED_GRAMMAR_BASE_URL}/{local}) (vendored)"
    raw_repo = entry.get("repo", "")
    return _repo_link(raw_repo if isinstance(raw_repo, str) else "")


def _extensions_cell(extensions: list[str]) -> str:
    """Format a list of file extensions as comma-separated code spans.

    Args:
        extensions: List of extension strings without leading dots.

    Returns:
        Formatted cell content, or "—" if empty.
    """
    if not extensions:
        return "—"
    return ", ".join(f"`.{ext}`" for ext in extensions)


def generate_table(project_root: Path, definitions: dict[str, dict[str, object]]) -> str:
    """Build the full markdown document content.

    Args:
        project_root: Repository root path (used to compute per-grammar ABI).
        definitions: Parsed language_definitions.json as a dict.

    Returns:
        Complete markdown string including header, summary, table, and an ABI
        compatibility section.
    """
    lang_count = len(definitions)

    lines: list[str] = [
        "---",
        "title: Supported Languages",
        (
            f'description: "The full list of {lang_count} tree-sitter grammars bundled by '
            "tree-sitter-language-pack, with file extensions, source repository, and "
            'ABI version."'
        ),
        "---",
        "",
        f"tree-sitter-language-pack supports **{lang_count}** languages.",
        "",
        "| Language | Extensions | Repository | ABI |",
        "|----------|------------|------------|-----|",
    ]

    abi_exceptions: list[tuple[str, int]] = []

    for lang_id, name in sorted(
        ((lid, _display_name(lid)) for lid in definitions),
        key=lambda x: x[1].lower(),
    ):
        entry = definitions[lang_id]
        raw_extensions = entry.get("extensions", [])
        extensions: list[str] = raw_extensions if isinstance(raw_extensions, list) else []
        ext_cell = _extensions_cell(extensions)
        repo_cell = _source_cell(entry)
        abi = _grammar_abi(project_root, lang_id)
        if abi != DEFAULT_TARGET_ABI:
            abi_exceptions.append((name, abi))
        lines.append(f"| {name} | {ext_cell} | {repo_cell} | {abi} |")

    lines.append("")
    lines.extend(_abi_compatibility_section(abi_exceptions))
    return "\n".join(lines)


def _abi_compatibility_section(abi_exceptions: list[tuple[str, int]]) -> list[str]:
    """Build the ABI compatibility section for the languages document.

    Args:
        abi_exceptions: (display_name, abi) pairs for grammars whose shipped ABI
            differs from DEFAULT_TARGET_ABI.

    Returns:
        Markdown lines describing ABI compatibility and, if any, the list of
        grammars that ship at a higher ABI.
    """
    lines = [
        "## ABI Compatibility",
        "",
        (
            f"The pack ships parsers at tree-sitter ABI {DEFAULT_TARGET_ABI}. These load on any "
            "consumer tree-sitter runtime from 0.21 through 0.26, so `get_language` passthrough "
            "works with a bring-your-own-runtime setup across that range."
        ),
        "",
    ]

    if not abi_exceptions:
        return lines

    lines.extend(
        [
            (
                "The following grammars ship at a higher ABI because their committed `parser.c` "
                "is too large to regenerate. They require tree-sitter >=0.25:"
            ),
            "",
        ]
    )
    lines.extend(f"- {name} (ABI {abi})" for name, abi in sorted(abi_exceptions, key=lambda item: item[0].lower()))
    lines.append("")
    return lines


QUERY_TYPES: tuple[str, ...] = ("highlights", "injections", "locals", "indents", "folds", "tags")

_PRESENT = "✅"
_ABSENT = "❌"

_GENERATED_HEADER = "<!-- generated by scripts/generate_grammar_table.py — do not edit -->"


def verify_parsers_present(project_root: Path, definitions: dict[str, dict[str, object]]) -> None:
    """Fail unless every declared language has a cloned parser on disk.

    Both the ABI column and the bundled-query columns are read live from
    ``parsers/<lang_id>/``. A language missing from that directory does not read
    as missing -- it reads as a grammar that targets the default ABI and bundles
    no queries, which is indistinguishable from a legitimate row. Generating
    against a partial ``parsers/`` therefore writes plausible but wrong data into
    files CI diffs, so treat an incomplete checkout as a hard error rather than
    silently degrading.

    ``TSLP_LANGUAGES`` makes this easy to hit: ``clone_vendors.py`` prunes
    languages outside the subset rather than leaving them alone, so any subset
    clone leaves ``parsers/`` incomplete for this script's purposes.

    Args:
        project_root: Repository root path.
        definitions: Parsed language_definitions.json as a dict.

    Raises:
        FileNotFoundError: If any declared language has no ``src/parser.c``.
    """
    missing = [
        lang_id
        for lang_id in sorted(definitions)
        if not (project_root / "parsers" / lang_id / "src" / "parser.c").is_file()
    ]
    if not missing:
        return
    shown = ", ".join(missing[:10])
    if len(missing) > 10:
        shown += f", ... ({len(missing) - 10} more)"
    raise FileNotFoundError(
        f"{len(missing)} of {len(definitions)} languages have no parsers/<lang>/src/parser.c: {shown}. "
        "Refusing to generate: the ABI and query columns are read from parsers/, so a partial "
        "checkout produces wrong values rather than an obvious failure. Run `mise run clone` "
        "(full, no TSLP_LANGUAGES) and retry."
    )


def _grammar_abi(project_root: Path, lang_id: str) -> int:
    """Compute the tree-sitter ABI the shipped parser.c targets.

    Grammars are regenerated at the pack's default target ABI. The exception is
    "monster" grammars whose committed ``src/parser.c`` is larger than
    ABI_EXEMPT_PARSER_BYTES: those ship the committed file as-is, so its ABI is
    read from the ``LANGUAGE_VERSION`` marker near the top of the file.

    Args:
        project_root: Repository root path.
        lang_id: Language identifier matching the parsers subdirectory name.

    Returns:
        The ABI version. Returns DEFAULT_TARGET_ABI only for parsers at or below
        the exemption threshold, which are regenerated at that ABI by
        construction -- never as a fallback for a parser it could not read.

    Raises:
        FileNotFoundError: If ``parser.c`` is absent (see
            :func:`verify_parsers_present`).
        OSError: If ``parser.c`` exists but cannot be read.
        ValueError: If an exempt parser carries no ``LANGUAGE_VERSION`` marker,
            so its shipped ABI cannot be determined.
    """
    parser_path = project_root / "parsers" / lang_id / "src" / "parser.c"

    if not parser_path.is_file():
        raise FileNotFoundError(f"{lang_id}: {parser_path} is missing; run `mise run clone` (full, no TSLP_LANGUAGES)")
    if parser_path.stat().st_size <= ABI_EXEMPT_PARSER_BYTES:
        return DEFAULT_TARGET_ABI
    with parser_path.open("rb") as handle:
        head = handle.read(_ABI_MARKER_SCAN_BYTES)

    match = _LANGUAGE_VERSION_RE.search(head.decode("utf-8", errors="ignore"))
    if match is None:
        raise ValueError(
            f"{lang_id}: parser.c is above the {ABI_EXEMPT_PARSER_BYTES}-byte regeneration "
            f"threshold so it ships as-is, but no LANGUAGE_VERSION marker was found in its "
            f"first {_ABI_MARKER_SCAN_BYTES} bytes -- its ABI cannot be determined"
        )
    return int(match.group(1))


def _bundled_query_types(project_root: Path, lang_id: str) -> set[str]:
    """Determine which standard query types a grammar bundles.

    Presence is read live from ``parsers/<lang_id>/queries/<type>.scm`` and from
    ``query-overlays/<lang_id>/<type>.scm``, which the build bundles in place of (or
    in addition to) the vendored files, so the result stays correct as the bundled
    query set changes.

    Args:
        project_root: Repository root path.
        lang_id: Language identifier matching the parsers subdirectory name.

    Returns:
        Set of query type names (subset of QUERY_TYPES) present on disk.
    """
    query_dirs = (
        project_root / "parsers" / lang_id / "queries",
        project_root / "query-overlays" / lang_id,
    )
    return {
        query_type
        for query_type in QUERY_TYPES
        if any((query_dir / f"{query_type}.scm").is_file() for query_dir in query_dirs)
    }


def generate_query_table(project_root: Path, definitions: dict[str, dict[str, object]]) -> str:
    """Build the per-grammar query-bundle markdown partial.

    Args:
        project_root: Repository root path (used to inspect parsers/ directories).
        definitions: Parsed language_definitions.json as a dict.

    Returns:
        Complete markdown string: a generated-file comment followed by a pipe
        table with one row per grammar and a column per standard query type.
    """
    header_columns = ["Language", "Repository", "ABI", *QUERY_TYPES]

    lines: list[str] = [
        _GENERATED_HEADER,
        "",
        f"| {' | '.join(header_columns)} |",
        f"|{'|'.join(['---'] * len(header_columns))}|",
    ]

    for lang_id, name in sorted(
        ((lid, _display_name(lid)) for lid in definitions),
        key=lambda item: item[1].lower(),
    ):
        entry = definitions[lang_id]
        repo_cell = _source_cell(entry)
        abi_cell = str(_grammar_abi(project_root, lang_id))

        bundled = _bundled_query_types(project_root, lang_id)
        query_cells = [_PRESENT if query_type in bundled else _ABSENT for query_type in QUERY_TYPES]

        row = " | ".join([name, repo_cell, abi_cell, *query_cells])
        lines.append(f"| {row} |")

    lines.append("")
    return "\n".join(lines)


def load_definitions(project_root: Path) -> dict[str, dict[str, object]]:
    """Load language_definitions.json from the sources directory.

    Args:
        project_root: Repository root path.

    Returns:
        Parsed JSON as a dict.

    Raises:
        FileNotFoundError: If the definitions file does not exist.
        ValueError: If the file is not valid JSON or is empty.
    """
    definitions_path = project_root / "sources" / "language_definitions.json"

    if not definitions_path.exists():
        msg = f"language_definitions.json not found: {definitions_path}"
        raise FileNotFoundError(msg)

    with definitions_path.open(encoding="utf-8") as f:
        data: dict[str, dict[str, object]] = json.load(f)

    if not data:
        msg = "language_definitions.json is empty"
        raise ValueError(msg)

    return data


def parse_args() -> argparse.Namespace:
    """Parse command-line arguments.

    Returns:
        Parsed argument namespace.
    """
    parser = argparse.ArgumentParser(
        description="Generate supported languages documentation from language_definitions.json",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Generate docs-site/src/content/docs/languages.md (default)
  python scripts/generate_grammar_table.py

  # Generate the per-grammar query-bundle partial
  python scripts/generate_grammar_table.py --mode queries \\
    --output templates/readme/partials/grammar_table.md

  # Write to a custom path
  python scripts/generate_grammar_table.py --output path/to/output.md

  # Print to stdout instead of writing a file
  python scripts/generate_grammar_table.py --stdout
        """,
    )

    parser.add_argument(
        "--mode",
        choices=("languages", "queries"),
        default="languages",
        help=(
            "Which table to generate: 'languages' (default) writes the "
            "Language/Extensions/Repository table; 'queries' writes the "
            "per-grammar query-bundle table."
        ),
    )

    parser.add_argument(
        "--output",
        metavar="PATH",
        help=(
            "Output file path (default: docs-site/src/content/docs/languages.md for --mode languages, "
            "templates/readme/partials/grammar_table.md for --mode queries)"
        ),
    )

    parser.add_argument(
        "--stdout",
        action="store_true",
        help="Print generated content to stdout instead of writing to disk",
    )

    return parser.parse_args()


def main() -> int:
    """Entry point.

    Returns:
        Exit code: 0 for success, 1 for failure.
    """
    args = parse_args()
    project_root = Path(__file__).resolve().parent.parent

    try:
        definitions = load_definitions(project_root)
        verify_parsers_present(project_root, definitions)
    except (FileNotFoundError, ValueError) as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 1

    if args.mode == "queries":
        content = generate_query_table(project_root, definitions)
        default_output = project_root / "templates" / "readme" / "partials" / "grammar_table.md"
    else:
        content = generate_table(project_root, definitions)
        default_output = project_root / "docs-site" / "src" / "content" / "docs" / "languages.md"

    if args.stdout:
        print(content, end="")
        return 0

    output_path = Path(args.output).resolve() if args.output else default_output

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(content, encoding="utf-8")
    try:
        display_path = output_path.relative_to(project_root)
    except ValueError:
        display_path = output_path
    print(f"Generated: {display_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
