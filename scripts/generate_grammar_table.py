#!/usr/bin/env python3
"""Generate a markdown table of all supported grammars from language_definitions.json.

Outputs docs/languages.md (or a custom path via --output).
Supports --stdout to print to stdout instead of writing to disk.
"""

import argparse
import json
import sys
from pathlib import Path


def _display_name(lang_id: str) -> str:
    """Convert a language identifier to a human-readable display name.

    Args:
        lang_id: Snake_case or lowercase language identifier.

    Returns:
        Title-cased display name with known acronyms preserved.
    """
    acronyms = {
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

    if lang_id in acronyms:
        return acronyms[lang_id]

    # Title-case with underscore/hyphen word splitting
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
    # Extract the last two path segments as the display label
    parts = repo_url.rstrip("/").split("/")
    label = "/".join(parts[-2:]) if len(parts) >= 2 else repo_url  # noqa: PLR2004
    return f"[{label}]({repo_url})"


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


def generate_table(definitions: dict[str, dict[str, object]]) -> str:
    """Build the full markdown document content.

    Args:
        definitions: Parsed language_definitions.json as a dict.

    Returns:
        Complete markdown string including header, summary, and table.
    """
    lang_count = len(definitions)

    lines: list[str] = [
        "# Supported Languages",
        "",
        f"tree-sitter-language-pack supports **{lang_count}** languages.",
        "",
        "| Language | Extensions | Repository |",
        "|----------|------------|------------|",
    ]

    for lang_id, name in sorted(
        ((lid, _display_name(lid)) for lid in definitions),
        key=lambda x: x[1].lower(),
    ):
        entry = definitions[lang_id]
        raw_extensions = entry.get("extensions", [])
        extensions: list[str] = raw_extensions if isinstance(raw_extensions, list) else []
        raw_repo = entry.get("repo", "")
        repo: str = raw_repo if isinstance(raw_repo, str) else ""
        ext_cell = _extensions_cell(extensions)
        repo_cell = _repo_link(repo)
        lines.append(f"| {name} | {ext_cell} | {repo_cell} |")

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
  # Generate docs/languages.md (default)
  python scripts/generate_grammar_table.py

  # Write to a custom path
  python scripts/generate_grammar_table.py --output path/to/output.md

  # Print to stdout instead of writing a file
  python scripts/generate_grammar_table.py --stdout
        """,
    )

    parser.add_argument(
        "--output",
        metavar="PATH",
        help="Output file path (default: docs/languages.md)",
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
    except (FileNotFoundError, ValueError) as exc:
        print(f"Error: {exc}", file=sys.stderr)
        return 1

    content = generate_table(definitions)

    if args.stdout:
        print(content, end="")
        return 0

    output_path = Path(args.output) if args.output else project_root / "docs" / "languages.md"

    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(content, encoding="utf-8")
    print(f"Generated: {output_path.relative_to(project_root)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
