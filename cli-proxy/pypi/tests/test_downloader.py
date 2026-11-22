"""Pinned-version and checksum behaviour of the runtime downloader."""

from __future__ import annotations

import hashlib
import io
import tarfile

import pytest
from ts_pack_cli import __version__, downloader


def _tarball(binary: bytes) -> bytes:
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode="w:gz") as archive:
        info = tarfile.TarInfo("ts-pack")
        info.size = len(binary)
        archive.addfile(info, io.BytesIO(binary))
    return buffer.getvalue()


def test_archive_name_matches_release_asset_naming() -> None:
    assert downloader._archive_name("aarch64-apple-darwin") == "ts-pack-aarch64-apple-darwin.tar.gz"
    assert downloader._archive_name("x86_64-pc-windows-msvc") == "ts-pack-x86_64-pc-windows-msvc.zip"


def test_asset_url_addresses_the_exact_tag_never_latest() -> None:
    url = downloader._asset_url("1.28.0-rc.1", "ts-pack-aarch64-apple-darwin.tar.gz")
    assert url == (
        "https://github.com/xberg-io/tree-sitter-language-pack/releases/download/"
        "v1.28.0-rc.1/ts-pack-aarch64-apple-darwin.tar.gz"
    )
    assert "latest" not in url


def test_verify_accepts_matching_digest() -> None:
    data = b"payload"
    downloader._verify_archive(data, "a.tar.gz", f"{hashlib.sha256(data).hexdigest()}  a.tar.gz\n")


def test_verify_rejects_digest_mismatch() -> None:
    sums = f"{hashlib.sha256(b'other').hexdigest()}  a.tar.gz\n"
    with pytest.raises(RuntimeError, match="checksum mismatch"):
        downloader._verify_archive(b"payload", "a.tar.gz", sums)


def test_verify_rejects_archive_missing_from_sums() -> None:
    sums = f"{hashlib.sha256(b'payload').hexdigest()}  b.tar.gz\n"
    with pytest.raises(RuntimeError, match="no checksum entry"):
        downloader._verify_archive(b"payload", "a.tar.gz", sums)


def test_verify_fails_closed_without_sums_unless_allowed() -> None:
    with pytest.raises(RuntimeError, match="refusing to install an unverified binary"):
        downloader._verify_archive(b"payload", "a.tar.gz", None)
    downloader._verify_archive(b"payload", "a.tar.gz", None, allow_unverified=True)


def _stub_download(monkeypatch, tmp_path, sums: str | None, archive: bytes):
    requested: list[str] = []

    def fake_get(url: str) -> bytes:
        requested.append(url)
        if url.endswith("/SHA256SUMS"):
            if sums is None:
                raise RuntimeError(f"HTTP 404 for {url}")
            return sums.encode()
        return archive

    monkeypatch.setattr(downloader, "_http_get", fake_get)
    monkeypatch.setattr(downloader, "_target_triple", lambda: "x86_64-unknown-linux-gnu")
    monkeypatch.setattr(downloader, "_binary_name", lambda: "ts-pack")
    monkeypatch.setattr(downloader.Path, "home", lambda: tmp_path)
    monkeypatch.delenv("TS_PACK_BINARY", raising=False)
    monkeypatch.delenv("TS_PACK_CLI_ALLOW_UNVERIFIED", raising=False)
    return requested


def test_ensure_binary_downloads_only_the_package_version(monkeypatch, tmp_path) -> None:
    archive = _tarball(b"#!/bin/sh\n")
    sums = f"{hashlib.sha256(archive).hexdigest()}  ts-pack-x86_64-unknown-linux-gnu.tar.gz\n"
    requested = _stub_download(monkeypatch, tmp_path, sums, archive)

    path = downloader.ensure_binary()

    base = f"https://github.com/xberg-io/tree-sitter-language-pack/releases/download/v{__version__}/"
    assert requested == [base + "ts-pack-x86_64-unknown-linux-gnu.tar.gz", base + "SHA256SUMS"]
    assert path.endswith("ts-pack")


def test_ensure_binary_refuses_on_checksum_mismatch(monkeypatch, tmp_path) -> None:
    archive = _tarball(b"#!/bin/sh\n")
    sums = f"{'0' * 64}  ts-pack-x86_64-unknown-linux-gnu.tar.gz\n"
    _stub_download(monkeypatch, tmp_path, sums, archive)

    with pytest.raises(RuntimeError, match="checksum mismatch"):
        downloader.ensure_binary()
    assert not list(tmp_path.rglob("ts-pack"))


def test_ensure_binary_refuses_when_release_has_no_sums(monkeypatch, tmp_path) -> None:
    _stub_download(monkeypatch, tmp_path, None, _tarball(b"#!/bin/sh\n"))

    with pytest.raises(RuntimeError, match="refusing to install an unverified binary"):
        downloader.ensure_binary()
