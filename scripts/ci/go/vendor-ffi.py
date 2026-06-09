#!/usr/bin/env python3
"""
Vendor FFI artifacts into Go package for distribution.

Copies the pre-built FFI header and static library into the Go binding's
include/ and lib/ directories so that `go build` works without the
`tspack_dev` build tag.

Usage:
    python scripts/ci/go/vendor-ffi.py          # vendor from target/release
    python scripts/ci/go/vendor-ffi.py --clean   # remove vendored artifacts
"""

import argparse
import os
import platform
import shutil
import sys
from pathlib import Path


def get_repo_root() -> Path:
    """Get repository root directory."""
    repo_root_env = os.environ.get("PROJECT_ROOT")
    if repo_root_env:
        return Path(repo_root_env)

    script_dir = Path(__file__).parent.absolute()
    return (script_dir / ".." / ".." / "..").resolve()


def lib_filename() -> str:
    """Return the platform-specific static library filename."""
    system = platform.system()
    if system == "Windows":
        return "ts_pack_ffi.lib"
    return "libts_pack_ffi.a"


def vendor(repo_root: Path) -> None:
    """Copy FFI header and static library into Go package."""
    go_dir = repo_root / "packages" / "go"
    ffi_include = repo_root / "crates" / "ts-pack-ffi" / "include"
    ffi_lib = repo_root / "target" / "release" / lib_filename()

    # Validate sources exist
    header = ffi_include / "ts_pack.h"
    if not header.exists():
        print(f"Error: FFI header not found at {header}", file=sys.stderr)
        print("Run `cargo build -p ts-pack-ffi --release` first.", file=sys.stderr)
        sys.exit(1)

    if not ffi_lib.exists():
        print(f"Error: FFI static library not found at {ffi_lib}", file=sys.stderr)
        print("Run `cargo build -p ts-pack-ffi --release` first.", file=sys.stderr)
        sys.exit(1)

    # Create target directories
    include_dir = go_dir / "include"
    lib_dir = go_dir / "lib"
    include_dir.mkdir(parents=True, exist_ok=True)
    lib_dir.mkdir(parents=True, exist_ok=True)

    # Copy header
    shutil.copy2(header, include_dir / "ts_pack.h")
    print(f"Copied {header.name} -> {include_dir}")

    # Copy static library
    dest_lib = lib_dir / lib_filename()
    shutil.copy2(ffi_lib, dest_lib)
    size_mb = dest_lib.stat().st_size / (1024 * 1024)
    print(f"Copied {lib_filename()} -> {lib_dir} ({size_mb:.1f} MB)")

    print(f"\nVendoring complete. Go package at {go_dir} is ready for `go build`.")


def clean(repo_root: Path) -> None:
    """Remove vendored FFI artifacts from Go package."""
    go_dir = repo_root / "packages" / "go"

    for subdir in ("include", "lib"):
        path = go_dir / subdir
        if path.exists():
            shutil.rmtree(path)
            print(f"Removed {path}")

    print("Clean complete.")


def main() -> None:
    parser = argparse.ArgumentParser(description="Vendor FFI artifacts into Go package")
    parser.add_argument("--clean", action="store_true", help="Remove vendored artifacts")
    args = parser.parse_args()

    repo_root = get_repo_root()

    if args.clean:
        clean(repo_root)
    else:
        vendor(repo_root)


if __name__ == "__main__":
    main()
