"""Git LFS pointer recovery from the published parser-sources bundle.

Shared by ``clone_vendors.py``; imported as a sibling module because the script is always run by path.
"""

from __future__ import annotations

import hashlib
import os
import re
import subprocess
import tarfile
from contextlib import contextmanager
from functools import cache
from pathlib import Path
from shutil import copyfileobj, which
from tempfile import mkdtemp
from typing import IO, TYPE_CHECKING
from urllib.request import urlopen

if TYPE_CHECKING:
    from collections.abc import Iterator

# ~keep A grammar whose parser.c lives in Git LFS becomes uncloneable the moment its upstream
# ~keep account runs out of LFS budget — the content is gone for everyone, so no retry, mirror
# ~keep or credential helps. Our own published parser-sources bundle already carries a copy of
# ~keep those bytes, and an LFS pointer file names the exact sha256 the content must have, so
# ~keep the recovery needs no trust in the bundle: verify against the pointer and the fallback
# ~keep is as safe as the original fetch. Point TSLP_LFS_FALLBACK_URL at a newer bundle when a
# ~keep later release is known to contain the affected grammar.
DEFAULT_LFS_FALLBACK_RELEASE = "1.15.10"
LFS_FALLBACK_URL = os.environ.get(
    "TSLP_LFS_FALLBACK_URL",
    f"https://github.com/xberg-io/tree-sitter-language-pack/releases/download/"
    f"v{DEFAULT_LFS_FALLBACK_RELEASE}/parser-sources-{DEFAULT_LFS_FALLBACK_RELEASE}.tar.zst",
)
LFS_FALLBACK_TIMEOUT_SECONDS = int(os.environ.get("TSLP_LFS_FALLBACK_TIMEOUT", "300"))
LFS_POINTER_MAGIC = b"version https://git-lfs.github.com/spec/v1"
LFS_POINTER_MAX_BYTES = 1024
LFS_POINTER_OID_PATTERN = re.compile(r"^oid sha256:([0-9a-f]{64})$", re.MULTILINE)
LFS_POINTER_SIZE_PATTERN = re.compile(r"^size (\d+)$", re.MULTILINE)


def _parse_lfs_pointer(path: Path) -> tuple[str, int] | None:
    """Parse a Git LFS pointer file into the sha256 oid and byte size it records.

    Args:
        path: A candidate file inside a checkout made with ``GIT_LFS_SKIP_SMUDGE=1``.

    Returns:
        ``(oid, size)`` for a well-formed pointer, or None if the file is not one.
    """
    try:
        if path.stat().st_size > LFS_POINTER_MAX_BYTES:
            return None
        raw = path.read_bytes()
    except OSError:
        return None

    if not raw.startswith(LFS_POINTER_MAGIC):
        return None

    text = raw.decode("utf-8", errors="replace")
    oid_match = LFS_POINTER_OID_PATTERN.search(text)
    size_match = LFS_POINTER_SIZE_PATTERN.search(text)
    if oid_match is None or size_match is None:
        return None
    return oid_match.group(1), int(size_match.group(1))


def _find_lfs_pointers(root: Path) -> dict[Path, tuple[str, int]]:
    """Collect every Git LFS pointer file in a checkout.

    Args:
        root: The checkout root to walk.

    Returns:
        Mapping of pointer file path to the ``(oid, size)`` it records.
    """
    pointers: dict[Path, tuple[str, int]] = {}
    for candidate in root.rglob("*"):
        if ".git" in candidate.parts or not candidate.is_file():
            continue
        parsed = _parse_lfs_pointer(candidate)
        if parsed is not None:
            pointers[candidate] = parsed
    return pointers


@cache
def _download_lfs_fallback_bundle() -> Path:
    """Download the parser-sources release bundle used to rehydrate LFS objects.

    Cached for the life of the process so that N pointers across N repositories cost one
    download.

    Returns:
        Path to the downloaded ``.tar.zst`` inside a temporary directory.

    Raises:
        RuntimeError: If the configured URL is not HTTPS, or the download fails.
    """
    if not LFS_FALLBACK_URL.startswith("https://"):
        raise RuntimeError(f"TSLP_LFS_FALLBACK_URL must be an https:// URL, got {LFS_FALLBACK_URL!r}")

    destination = Path(mkdtemp(prefix="tslp-lfs-fallback-")) / "parser-sources.tar.zst"
    print(f"[clone_vendors] downloading LFS fallback bundle from {LFS_FALLBACK_URL}", flush=True)
    try:
        with (
            urlopen(LFS_FALLBACK_URL, timeout=LFS_FALLBACK_TIMEOUT_SECONDS) as response,
            destination.open("wb") as sink,
        ):
            copyfileobj(response, sink)
    except (OSError, ValueError) as e:
        raise RuntimeError(f"failed to download LFS fallback bundle from {LFS_FALLBACK_URL}: {e}") from e

    print(f"[clone_vendors] cached LFS fallback bundle ({destination.stat().st_size} bytes)", flush=True)
    return destination


def _stdlib_zstd_file() -> type | None:
    """Return the stdlib ``ZstdFile`` class, or None on interpreters that lack it.

    The import is deliberately function-local: ``compression.zstd`` does not exist before
    Python 3.14, so a module-level import would break the script on every older
    interpreter for a code path most runs never reach. ~keep
    """
    try:
        from compression.zstd import ZstdFile  # noqa: PLC0415
    except ImportError:
        return None
    return ZstdFile


@contextmanager
def _open_zstd_stream(archive: Path) -> Iterator[IO[bytes]]:
    """Open a zstd-compressed file as a readable, sequential binary stream.

    ``compression.zstd`` only exists on Python 3.14+ and this script runs on 3.12 through
    3.14 across the workflows, so fall back to piping through the ``zstd`` binary rather
    than depending on a package that is not declared anywhere. ~keep

    Args:
        archive: Path to the ``.tar.zst`` file.

    Yields:
        A binary stream of the decompressed bytes.

    Raises:
        RuntimeError: If neither a stdlib decompressor nor a ``zstd`` binary is available.
    """
    zstd_file = _stdlib_zstd_file()
    if zstd_file is not None:
        with zstd_file(archive, "rb") as stream:
            yield stream
        return

    zstd_binary = which("zstd")
    if zstd_binary is None:
        raise RuntimeError(
            "cannot decompress the LFS fallback bundle: this interpreter has no compression.zstd "
            "module and no 'zstd' binary is on PATH"
        )
    process = subprocess.Popen([zstd_binary, "-d", "-c", str(archive)], stdout=subprocess.PIPE)
    if process.stdout is None:
        process.kill()
        raise RuntimeError(f"failed to open a pipe to {zstd_binary} for the LFS fallback bundle")
    try:
        yield process.stdout
    finally:
        process.stdout.close()
        process.kill()
        process.wait()


def _extract_lfs_objects(archive: Path, wanted: dict[str, int]) -> dict[str, bytes]:
    """Pull LFS object contents out of the fallback bundle, keyed by their verified sha256.

    Members are pre-filtered on exact byte size and then hashed, so the bundle's own paths and
    layout are never trusted: a member is accepted only because its content hashes to an oid
    some pointer file asked for. That is what makes an arbitrary fallback source safe. ~keep

    Args:
        archive: The downloaded ``.tar.zst`` bundle.
        wanted: Mapping of pointer oid to the byte size that pointer recorded.

    Returns:
        Mapping of oid to matching content, for every wanted oid present in the bundle.
    """
    wanted_sizes = set(wanted.values())
    found: dict[str, bytes] = {}
    with _open_zstd_stream(archive) as stream, tarfile.open(fileobj=stream, mode="r|") as bundle:
        for member in bundle:
            if not member.isfile() or member.size not in wanted_sizes:
                continue
            handle = bundle.extractfile(member)
            if handle is None:
                continue
            content = handle.read()
            digest = hashlib.sha256(content).hexdigest()
            if wanted.get(digest) == len(content):
                found[digest] = content
            if len(found) == len(wanted):
                break
    return found


def _write_verified_lfs_object(path: Path, content: bytes, oid: str, size: int, language_name: str) -> None:
    """Write recovered LFS content, but only when it matches the pointer file exactly.

    Args:
        path: The pointer file to replace with real content.
        content: The candidate bytes recovered from the fallback bundle.
        oid: The sha256 the pointer file recorded.
        size: The byte size the pointer file recorded.
        language_name: The grammar being restored, for the error message.

    Raises:
        RuntimeError: If the content does not hash to ``oid`` or is not ``size`` bytes long.
    """
    digest = hashlib.sha256(content).hexdigest()
    if digest != oid or len(content) != size:
        raise RuntimeError(
            f"{language_name}: refusing to write {path.name} from the LFS fallback bundle "
            f"{LFS_FALLBACK_URL} — the pointer file expects sha256 {oid} ({size} bytes) but the "
            f"recovered content is sha256 {digest} ({len(content)} bytes)"
        )
    path.write_bytes(content)
    print(f"[clone_vendors] {language_name}: restored {path.name} ({size} bytes, sha256 {oid})", flush=True)
