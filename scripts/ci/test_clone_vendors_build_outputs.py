"""Vendoring drops compiled objects that some upstream grammars commit in src/."""

from __future__ import annotations

import asyncio
import sys
from pathlib import Path
from typing import TYPE_CHECKING

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

import clone_vendors as vendor

if TYPE_CHECKING:
    import pytest

BUILD_OUTPUTS = ["parser.o", "scanner.obj", "libx.a", "x.lib", "x.so", "x.dylib", "x.dll"]


def test_move_src_folder_drops_upstream_build_outputs(tmp_path: Path, monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr(vendor, "vendor_directory", tmp_path / "vendor")
    monkeypatch.setattr(vendor, "parsers_directory", tmp_path / "parsers")
    repo = tmp_path / "vendor" / "bitbake"
    (repo / "src" / "tree_sitter").mkdir(parents=True)
    (repo / "common").mkdir()
    (repo / "LICENSE").write_text("MIT\n")
    (repo / "src" / "parser.c").write_text("int x;\n")
    (repo / "src" / "tree_sitter" / "parser.h").write_text("\n")
    for name in BUILD_OUTPUTS:
        (repo / "src" / name).write_bytes(b"\0")
    (repo / "common" / "helper.o").write_bytes(b"\0")

    asyncio.run(vendor.move_src_folder("bitbake", None, language_definition={"repo": "https://github.com/x/y"}))

    target = tmp_path / "parsers" / "bitbake"
    remaining = sorted(path.relative_to(target).as_posix() for path in target.rglob("*") if path.is_file())
    assert remaining == ["LICENSE", "src/parser.c", "src/tree_sitter/parser.h"]
