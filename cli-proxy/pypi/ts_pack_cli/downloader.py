"""Download, verify, and run the native ts-pack binary.

The package version is the release version: the archive is fetched from the
``v<version>`` release of this exact package, never from ``latest``, and checked
against that release's SHA256SUMS. All diagnostics go to stderr.
"""

from __future__ import annotations

import hashlib
import os
import platform
import shutil
import subprocess
import sys
import tarfile
import tempfile
import zipfile
from pathlib import Path, PurePosixPath
from typing import cast
from urllib.error import HTTPError, URLError
from urllib.parse import quote, urlsplit
from urllib.request import HTTPRedirectHandler, Request, build_opener

from . import __version__

REPO = "xberg-io/tree-sitter-language-pack"
BIN_NAME = "ts-pack"
PKG_NAME = "ts-pack-cli"
ALLOW_UNVERIFIED_ENV = "TS_PACK_CLI_ALLOW_UNVERIFIED"
SUMS_NAME = "SHA256SUMS"
_USER_AGENT = "ts-pack-cli-python-proxy"


def _target_triple() -> str:
    system = platform.system().lower()
    machine = platform.machine().lower()

    if system == "windows":
        if machine in {"amd64", "x86_64"}:
            return "x86_64-pc-windows-msvc"
        raise RuntimeError(f"unsupported Windows arch: {machine}")
    if system == "linux":
        if machine in {"amd64", "x86_64"}:
            return "x86_64-unknown-linux-gnu"
        if machine in {"aarch64", "arm64"}:
            return "aarch64-unknown-linux-gnu"
        raise RuntimeError(f"unsupported Linux arch: {machine}")
    if system == "darwin":
        if machine in {"aarch64", "arm64"}:
            return "aarch64-apple-darwin"
        if machine in {"amd64", "x86_64"}:
            return "x86_64-apple-darwin"
        raise RuntimeError(f"unsupported macOS arch: {machine}")
    raise RuntimeError(f"unsupported platform: {system} {machine}")


def _binary_name() -> str:
    return f"{BIN_NAME}.exe" if platform.system().lower() == "windows" else BIN_NAME


class _HttpsOnlyRedirectHandler(HTTPRedirectHandler):
    """Reject any redirect whose target is not https (downgrade/SSRF guard)."""

    def redirect_request(self, req, fp, code, msg, headers, newurl):  # type: ignore[no-untyped-def]  # noqa: ANN001, ANN202, PLR0913
        if urlsplit(newurl).scheme.lower() != "https":
            raise URLError(f"refusing non-https redirect to: {newurl}")
        return super().redirect_request(req, fp, code, msg, headers, newurl)


_opener = build_opener(_HttpsOnlyRedirectHandler())


def _http_get(url: str) -> bytes:
    if urlsplit(url).scheme.lower() != "https":
        raise RuntimeError(f"refusing non-https URL: {url}")
    request = Request(url, headers={"User-Agent": _USER_AGENT})  # noqa: S310 - scheme validated https-only above
    try:
        with _opener.open(request, timeout=60) as response:
            if response.status != 200:
                raise RuntimeError(f"HTTP {response.status} for {url}")
            return cast("bytes", response.read())
    except HTTPError as exc:
        raise RuntimeError(f"HTTP {exc.code} for {url}") from exc
    except URLError as exc:
        raise RuntimeError(f"failed to fetch {url}: {exc}") from exc


def _archive_name(triple: str) -> str:
    return f"{BIN_NAME}-{triple}.{'zip' if 'windows' in triple else 'tar.gz'}"


def _asset_url(version: str, name: str) -> str:
    return f"https://github.com/{REPO}/releases/download/v{quote(version, safe='')}/{name}"


def _expected_digest(text: str, asset_name: str) -> str | None:
    for line in text.splitlines():
        stripped = line.strip()
        if not stripped:
            continue
        parts = stripped.split()
        if len(parts) < 2:
            continue
        name = parts[-1].lstrip("*")
        if name == asset_name:
            return parts[0].lower()
    return None


def _verify_archive(data: bytes, asset_name: str, sums_text: str | None, *, allow_unverified: bool = False) -> None:
    """Check ``data`` against the release's SHA256SUMS; fail closed when it is missing."""
    if sums_text is None:
        if allow_unverified:
            print(
                f"WARNING: no {SUMS_NAME} asset found for {asset_name}; installing unverified "
                f"because {ALLOW_UNVERIFIED_ENV}=1 is set.",
                file=sys.stderr,
            )
            return
        raise RuntimeError(
            f"no {SUMS_NAME} asset found in the release for {asset_name} - refusing to install an "
            f"unverified binary. Set {ALLOW_UNVERIFIED_ENV}=1 to override."
        )
    expected = _expected_digest(sums_text, asset_name)
    if not expected:
        raise RuntimeError(f"no checksum entry for {asset_name} in {SUMS_NAME} - refusing to install unverified binary")
    digest = hashlib.sha256(data).hexdigest().lower()
    if digest != expected:
        raise RuntimeError(f"checksum mismatch for {asset_name} (expected {expected}, got {digest})")


def _fetch_sums(version: str) -> str | None:
    try:
        return _http_get(_asset_url(version, SUMS_NAME)).decode("utf-8")
    except RuntimeError as exc:
        if "HTTP 404" in str(exc):
            return None
        raise


def _is_within(base: Path, target: Path) -> bool:
    """True if `target`'s resolved path stays inside `base`."""
    base_resolved = base.resolve()
    try:
        target.resolve().relative_to(base_resolved)
    except ValueError:
        return False
    return True


def _reject_unsafe_member(name: str) -> None:
    """Reject absolute paths and any `..` component (zip-slip / tar-slip)."""
    normalized = name.replace("\\", "/")
    pure = PurePosixPath(normalized)
    if pure.is_absolute() or normalized.startswith("/"):
        raise RuntimeError(f"refusing absolute path in archive: {name}")
    if len(normalized) >= 2 and normalized[1] == ":":
        raise RuntimeError(f"refusing absolute path in archive: {name}")
    if any(part == ".." for part in pure.parts):
        raise RuntimeError(f"refusing parent-directory escape in archive: {name}")


def _safe_extract(archive_path: Path, asset_name: str, dest: Path) -> None:
    """Extract a tar/zip, validating every member stays within `dest`.

    Does not rely on extractall() defaults or the 3.12-only ``filter='data'``:
    each member name is rejected up front if it is absolute or contains ``..``,
    and the resolved destination path is re-checked against ``dest``.
    """
    dest = dest.resolve()
    dest.mkdir(parents=True, exist_ok=True)

    if asset_name.lower().endswith(".zip"):
        with zipfile.ZipFile(archive_path) as zf:
            for entry in zf.namelist():
                _reject_unsafe_member(entry)
                if not _is_within(dest, dest / entry):
                    raise RuntimeError(f"refusing archive entry escaping dest: {entry}")
            for entry in zf.namelist():
                zf.extract(entry, dest)
    else:
        with tarfile.open(archive_path, "r:gz") as tf:
            for member in tf.getmembers():
                _reject_unsafe_member(member.name)
                if not _is_within(dest, dest / member.name):
                    raise RuntimeError(f"refusing archive entry escaping dest: {member.name}")
                if member.islnk() or member.issym():
                    link_target = dest / member.name
                    if not _is_within(dest, link_target.parent / member.linkname):
                        raise RuntimeError(f"refusing link escaping dest: {member.name}")
            tf.extractall(dest)  # noqa: S202 (members validated above)


def _find_binary(root: Path, name: str) -> Path | None:
    for candidate in root.rglob(name):
        if candidate.is_file():
            return candidate
    return None


def _cache_dir(tag: str) -> Path:
    cache = Path.home() / ".cache" / PKG_NAME / tag
    cache.mkdir(parents=True, exist_ok=True, mode=0o700)
    return cache


def ensure_binary() -> str:
    """Ensure the native binary for this package version exists locally, downloading if necessary."""
    override = os.getenv("TS_PACK_BINARY")
    if override:
        return override

    version = __version__
    cache = _cache_dir(f"v{version}")
    binary_path = cache / _binary_name()
    if binary_path.exists() and os.access(binary_path, os.X_OK):
        return str(binary_path)

    name = _archive_name(_target_triple())
    print(f"Downloading {BIN_NAME} {version} asset {name}...", file=sys.stderr)
    with tempfile.TemporaryDirectory() as tmpdir:
        tmp = Path(tmpdir)
        data = _http_get(_asset_url(version, name))
        _verify_archive(
            data,
            name,
            _fetch_sums(version),
            allow_unverified=os.getenv(ALLOW_UNVERIFIED_ENV) == "1",
        )
        archive_path = tmp / name
        archive_path.write_bytes(data)
        extract_dir = tmp / "extract"
        extract_dir.mkdir()
        _safe_extract(archive_path, name, extract_dir)
        found = _find_binary(extract_dir, _binary_name())
        if not found:
            raise RuntimeError(f"binary {_binary_name()} not found after extracting {name}")
        shutil.move(str(found), str(binary_path))

    if platform.system().lower() != "windows":
        binary_path.chmod(0o755)
    print(f"{BIN_NAME} installed.", file=sys.stderr)
    return str(binary_path)


def run(args: list[str]) -> int:
    """Run the native binary with the given args, returning its exit code."""
    binary_path = ensure_binary()
    completed = subprocess.run([binary_path, *args], check=False)  # noqa: S603
    return completed.returncode
