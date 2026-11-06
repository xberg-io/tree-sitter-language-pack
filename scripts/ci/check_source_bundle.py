#!/usr/bin/env python3
"""Fail if a parser-sources bundle ships build outputs or a grammar without a licence file.

The ``parser-sources-<version>.tar.zst`` release asset is consumed as a source
resource by downstream packagers (Homebrew, distro builds). It must carry
sources only, and every grammar directory must carry the licence that covers it.

Usage::

    scripts/ci/check_source_bundle.py dist/parser-sources-1.25.0.tar.zst
    scripts/ci/check_source_bundle.py parsers-staging-dir/

A directory argument is checked as if it were the bundle root. Exit status is 0
only when at least one grammar directory was inspected, no member is a compiled
object or library, and every ``parsers/<language>/`` holds a top-level file whose
name starts with LICENSE, LICENCE, COPYING, COPYRIGHT or NOTICE.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path, PurePosixPath

BUILD_OUTPUT_SUFFIXES = (".o", ".obj", ".a", ".lib", ".so", ".dylib", ".dll")
LICENSE_NAME_PREFIXES = ("LICENSE", "LICENCE", "COPYING", "COPYRIGHT", "NOTICE")


def _bundle_members(bundle: Path) -> list[str]:
    if bundle.is_dir():
        return [path.relative_to(bundle).as_posix() for path in sorted(bundle.rglob("*")) if path.is_file()]
    result = subprocess.run(["tar", "--zstd", "-tf", str(bundle)], capture_output=True, text=True, check=False)
    if result.returncode != 0:
        raise SystemExit(f"error: cannot list {bundle}: {result.stderr.strip()}")
    return [line.removeprefix("./") for line in result.stdout.splitlines() if line and not line.endswith("/")]


def _is_license_name(name: str) -> bool:
    return name.upper().startswith(LICENSE_NAME_PREFIXES)


def check(members: list[str]) -> tuple[int, list[str]]:
    """Return ``(grammar_count, problems)`` for a bundle's member file paths."""
    problems: list[str] = []
    grammars: set[str] = set()
    licensed: set[str] = set()
    for member in members:
        path = PurePosixPath(member)
        if path.suffix.lower() in BUILD_OUTPUT_SUFFIXES:
            problems.append(f"build output in bundle: {member}")
        if len(path.parts) < 3 or path.parts[0] != "parsers":
            continue
        grammars.add(path.parts[1])
        if len(path.parts) == 3 and _is_license_name(path.name):
            licensed.add(path.parts[1])
    problems.extend(f"no licence file in parsers/{name}/" for name in sorted(grammars - licensed))
    if not grammars:
        problems.append("no parsers/<language>/ directories found; nothing was checked")
    return len(grammars), problems


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("bundle", type=Path, help="a parser-sources .tar.zst, or a directory laid out like one")
    args = parser.parse_args(argv)

    members = _bundle_members(args.bundle)
    grammar_count, problems = check(members)
    for problem in problems:
        print(f"error: {problem}", file=sys.stderr)
    print(f"{args.bundle}: {len(members)} file(s), {grammar_count} grammar(s), {len(problems)} problem(s)")
    return 1 if problems else 0


if __name__ == "__main__":
    raise SystemExit(main())
