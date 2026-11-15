#!/usr/bin/env python3
"""Build a standalone ``libtree-sitter`` shared library.

``get_language()`` returns the host runtime's native tree-sitter ``Language``
(a raw ``TSLanguage *``). Host bindings that do NOT bundle their own tree-sitter
C runtime (jtreesitter for Java, TreeSitter.DotNet for C#) ``dlopen`` a
``libtree-sitter`` shared library at runtime to resolve ``ts_parser_*`` /
``ts_node_*`` / ``ts_tree_*``. Our FFI cdylib exports only ``ts_pack_*`` and
dead-strips the unused runtime, so those hosts need a separate ``libtree-sitter``.

This compiles the tree-sitter C runtime (``src/lib.c``) from the exact
``tree-sitter`` crate version already locked in ``Cargo.lock`` — located via
``cargo metadata`` so there is no hard-coded registry path — into a shared
library named exactly ``libtree-sitter.{dylib,so,dll}`` (the name
``System.mapLibraryName("tree-sitter")`` / ``[DllImport("tree-sitter")]`` resolve).

Compiling the C directly (rather than through a Rust cdylib) avoids the
platform-specific symbol-export problem where a Rust cdylib's version script
hides statically linked C symbols on Linux.

Usage:
    python scripts/ci/build_libtree_sitter.py            # -> target/release
    python scripts/ci/build_libtree_sitter.py --out DIR  # -> DIR
    python scripts/ci/build_libtree_sitter.py --profile debug
"""

import argparse
import json
import os
import platform
import subprocess
import sys
from pathlib import Path

LIB_BASENAME = "tree-sitter"


def repo_root() -> Path:
    env = os.environ.get("PROJECT_ROOT")
    if env:
        return Path(env)
    return (Path(__file__).parent / ".." / "..").resolve()


def tree_sitter_source(root: Path) -> Path:
    """Locate the tree-sitter crate source via cargo metadata (no hard-coded path)."""
    meta = json.loads(subprocess.check_output(["cargo", "metadata", "--format-version", "1"], cwd=root))
    candidates = [p for p in meta["packages"] if p["name"] == "tree-sitter"]
    if not candidates:
        sys.exit("error: tree-sitter crate not found in cargo metadata")
    candidates.sort(key=lambda p: p["version"])
    src = Path(candidates[-1]["manifest_path"]).parent
    lib_c = src / "src" / "lib.c"
    if not lib_c.exists():
        sys.exit(f"error: tree-sitter C source not found at {lib_c}")
    return src


def platform_spec() -> tuple[str, list[str], list[str]]:
    """Return (output filename, extra compile flags, link flags) for this OS."""
    system = platform.system()
    if system == "Darwin":
        out = f"lib{LIB_BASENAME}.dylib"
        return out, ["-dynamiclib"], [f"-Wl,-install_name,@rpath/{out}"]
    if system == "Windows":
        return f"{LIB_BASENAME}.dll", ["-shared"], []
    out = f"lib{LIB_BASENAME}.so"
    return out, ["-shared"], [f"-Wl,-soname,{out}"]


def main() -> int:
    parser = argparse.ArgumentParser(description="Build standalone libtree-sitter")
    parser.add_argument("--out", help="output directory (default: target/<profile>)")
    parser.add_argument("--profile", default="release", help="cargo profile dir name")
    parser.add_argument("--cc", default=os.environ.get("CC", "cc"), help="C compiler")
    args = parser.parse_args()

    root = repo_root()
    src = tree_sitter_source(root)
    out_dir = Path(args.out) if args.out else root / "target" / args.profile
    out_dir.mkdir(parents=True, exist_ok=True)

    out_name, shared_flags, link_flags = platform_spec()
    out_path = out_dir / out_name

    cmd = [
        args.cc,
        "-O2",
        "-std=c11",
        *(["-D_DEFAULT_SOURCE"] if platform.system() == "Linux" else []),
        "-fPIC",
        *shared_flags,
        f"-I{src / 'include'}",
        f"-I{src / 'src'}",
        str(src / "src" / "lib.c"),
        "-o",
        str(out_path),
        *link_flags,
    ]
    print(f"building {out_path} from {src.name}")
    subprocess.run(cmd, check=True)
    print(f"wrote {out_path} ({out_path.stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
