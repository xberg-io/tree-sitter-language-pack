"""Tests for scripts/ci/check_source_bundle.py against real .tar.zst bundles."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).resolve().parent))

import check_source_bundle as bundle_check


def _pack(root: Path, files: dict[str, str], tmp_path: Path) -> Path:
    for relative, text in files.items():
        path = root / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)
    output = tmp_path / "parser-sources-0.0.0.tar.zst"
    subprocess.run(["tar", "--zstd", "-cf", str(output), "-C", str(root), "parsers"], check=True)
    return output


CLEAN = {
    "parsers/python/LICENSE": "MIT License\n",
    "parsers/python/src/parser.c": "int x;\n",
    "parsers/python/queries/highlights.scm": "(identifier) @variable\n",
    "parsers/brightscript/NOTICE": "SPDX-License-Identifier: MIT\n",
    "parsers/brightscript/src/parser.c": "int y;\n",
}


def test_clean_bundle_passes(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    bundle = _pack(tmp_path / "root", CLEAN, tmp_path)
    assert bundle_check.main([str(bundle)]) == 0
    assert "5 file(s), 2 grammar(s), 0 problem(s)" in capsys.readouterr().out


def test_object_file_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    bundle = _pack(
        tmp_path / "root", {**CLEAN, "parsers/vhs/LICENSE": "MIT\n", "parsers/vhs/src/parser.o": "\0"}, tmp_path
    )
    assert bundle_check.main([str(bundle)]) == 1
    assert "build output in bundle: parsers/vhs/src/parser.o" in capsys.readouterr().err


@pytest.mark.parametrize("name", ["libx.a", "x.so", "x.dylib", "x.dll", "x.obj", "x.lib"])
def test_every_build_output_suffix_fails(name: str) -> None:
    _, problems = bundle_check.check(["parsers/c/LICENSE", f"parsers/c/src/{name}"])
    assert problems == [f"build output in bundle: parsers/c/src/{name}"]


def test_grammar_without_licence_fails(tmp_path: Path, capsys: pytest.CaptureFixture[str]) -> None:
    bundle = _pack(tmp_path / "root", {**CLEAN, "parsers/clarity/src/parser.c": "int z;\n"}, tmp_path)
    assert bundle_check.main([str(bundle)]) == 1
    assert "no licence file in parsers/clarity/" in capsys.readouterr().err


def test_licence_must_sit_at_the_grammar_root() -> None:
    _, problems = bundle_check.check(["parsers/c/src/LICENSE", "parsers/c/src/parser.c"])
    assert problems == ["no licence file in parsers/c/"]


def test_empty_bundle_fails_instead_of_passing_vacuously(tmp_path: Path) -> None:
    root = tmp_path / "root"
    (root / "parsers").mkdir(parents=True)
    (root / "parsers" / ".keep").write_text("")
    bundle = _pack(root, {}, tmp_path)
    assert bundle_check.main([str(bundle)]) == 1


def test_directory_layout_is_checked_like_a_bundle(tmp_path: Path) -> None:
    root = tmp_path / "root"
    _pack(root, CLEAN, tmp_path)
    assert bundle_check.main([str(root)]) == 0
    (root / "parsers" / "python" / "src" / "parser.o").write_bytes(b"\0")
    assert bundle_check.main([str(root)]) == 1
