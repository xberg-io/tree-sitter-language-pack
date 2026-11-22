"""The wheel build hook repackages the released CLI archive for one target."""

from __future__ import annotations

import io
import tarfile
from pathlib import Path

import pytest
from hatch_build import CustomBuildHook

_TARGET = "x86_64-unknown-linux-gnu"


def _hook(root: Path) -> CustomBuildHook:
    return CustomBuildHook(str(root), {}, None, None, str(root), "wheel")


def _stage_archive(root: Path, target: str, member: str) -> None:
    dist = root.parent.parent / "dist"
    dist.mkdir(parents=True, exist_ok=True)
    with tarfile.open(dist / f"ts-pack-{target}.tar.gz", "w:gz") as archive:
        info = tarfile.TarInfo(member)
        info.size = 4
        archive.addfile(info, io.BytesIO(b"bin\n"))


def test_no_target_is_a_noop(tmp_path, monkeypatch) -> None:
    monkeypatch.delenv("TS_PACK_CLI_TARGET", raising=False)
    data: dict = {}
    _hook(tmp_path).initialize("1.0.0", data)
    assert data == {}


def test_target_without_archive_refuses_an_empty_wheel(tmp_path, monkeypatch) -> None:
    monkeypatch.setenv("TS_PACK_CLI_TARGET", _TARGET)
    with pytest.raises(RuntimeError, match="refusing to build an empty platform wheel"):
        _hook(tmp_path / "a" / "b").initialize("1.0.0", {})


def test_target_bundles_the_binary_and_tags_the_wheel(tmp_path, monkeypatch) -> None:
    root = tmp_path / "cli-proxy" / "pypi"
    root.mkdir(parents=True)
    _stage_archive(root, _TARGET, "ts-pack")
    monkeypatch.setenv("TS_PACK_CLI_TARGET", _TARGET)
    data: dict = {}
    _hook(root).initialize("1.0.0", data)
    assert list(data["force_include"].values()) == [f"ts_pack_cli/bin/{_TARGET}/ts-pack"]
    assert data["tag"] == "py3-none-manylinux_2_39_x86_64"
    assert data["pure_python"] is False
