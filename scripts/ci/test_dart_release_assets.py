"""Verify Dart release archives match the generated downloader contract."""

from __future__ import annotations

import hashlib
import subprocess
import sys
import tarfile
import tempfile
import unittest
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / "scripts/publish/dart/package_native_assets.py"
VERSION = "1.17.0"
PLATFORMS = {
    "linux-x64": ("linux-x86_64", "libtree_sitter_language_pack_dart.so"),
    "macos-arm64": ("macos-aarch64", "libtree_sitter_language_pack_dart.dylib"),
    "macos-x64": ("macos-x86_64", "libtree_sitter_language_pack_dart.dylib"),
    "windows-x64": ("windows-x86_64", "tree_sitter_language_pack_dart.dll"),
}


class DartReleaseAssetTests(unittest.TestCase):
    def test_published_archive_set_covers_the_build_matrix(self) -> None:
        workflow = yaml.safe_load((ROOT / ".github/workflows/publish.yaml").read_text())
        jobs = workflow["jobs"]
        matrix = jobs["build-dart-native"]["strategy"]["matrix"]["include"]
        assert set(PLATFORMS) == {entry["label"] for entry in matrix}
        assembly = jobs["assemble-dart-package"]
        assert assembly["permissions"]["contents"] == "write"
        upload = next(step for step in assembly["steps"] if step.get("name") == "Upload Dart native download assets")
        assert upload["with"]["working-directory"] == "dist/dart"
        assert upload["with"]["assets"].split() == ["*.tar.gz", "*.sha256"]

    def test_archives_include_exact_native_payload_and_checksum(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for rid, (_, filename) in PLATFORMS.items():
                native = root / "artifacts" / f"dart-native-{rid}" / filename
                native.parent.mkdir(parents=True)
                native.write_bytes(rid.encode())
            result = subprocess.run(
                [sys.executable, str(SCRIPT), str(root / "artifacts"), str(root / "dist"), VERSION],
                capture_output=True,
                text=True,
                check=False,
            )
            assert result.returncode == 0, result.stderr
            assert len(list((root / "dist").iterdir())) == len(PLATFORMS) * 2
            for rid, (blob, filename) in PLATFORMS.items():
                archive = root / "dist" / f"tree-sitter-language-pack-dart-v{VERSION}-{blob}.tar.gz"
                digest = hashlib.sha256(archive.read_bytes()).hexdigest()
                assert Path(f"{archive}.sha256").read_text() == f"{digest}  {archive.name}\n"
                with tarfile.open(archive) as package:
                    assert package.getnames() == [filename]
                    native = package.extractfile(filename)
                    assert native is not None
                    assert native.read() == rid.encode()

    def test_missing_platform_fails_before_creating_any_archive(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            result = subprocess.run(
                [sys.executable, str(SCRIPT), str(root / "artifacts"), str(root / "dist"), VERSION],
                capture_output=True,
                text=True,
                check=False,
            )
            assert result.returncode != 0
            assert not (root / "dist").exists()


if __name__ == "__main__":
    unittest.main()
