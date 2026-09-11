"""Clone, generate and vendor the pinned upstream tree-sitter grammars.

Trust model — read before running this against a grammar entry you have not reviewed.

Every `rev` in `sources/language_definitions.json` is validated as a full 40-hex commit SHA
against an allowlist of `github.com`/`gitlab.com` (`_vendor_sources.validate_rev` and
`validate_repo_url`), enforced both at load time and again in `clone_repository`. That makes a
refresh *reproducible*: the same pins fetch the same bytes. It does not make it *safe*. A commit
pinned by SHA can contain anything, and pinning is not review.

For entries with `generate: true`, `tree-sitter generate` evaluates the upstream repository's
`grammar.js` as JavaScript under Node, with the full privileges of whoever ran this script, and
that `grammar.js` may `require()` anything present in `node_modules`. `--ignore-scripts` on the
npm step blocks install-time lifecycle scripts, which is a different and narrower vector; it does
not constrain `grammar.js` itself.

The highest-privilege context this runs in is therefore a maintainer's own machine during a pin
bump — CI jobs that invoke this script hold only `contents: read` and reference no secrets. Treat
adding or bumping a grammar as running that project's build tooling, because that is what it is:
review the diff of `grammar.js` and any new dependency, or run the refresh in a disposable
sandbox.
"""

import asyncio
import hashlib
import os
import platform
import re
import signal
import subprocess
import sys
import tarfile
from collections.abc import Iterator
from contextlib import contextmanager, suppress
from functools import cache, partial
from json import dumps, loads
from pathlib import Path
from shutil import copyfileobj, copytree, move, rmtree, which
from tempfile import mkdtemp
from typing import IO, NotRequired, TypedDict
from urllib.request import urlopen

from _vendor_sources import validate_branch, validate_language_definitions, validate_repo_url, validate_rev
from anyio import Path as AsyncPath
from anyio import run_process
from anyio.to_thread import run_sync
from git import Repo

_project_root = Path(__file__).parent.parent

vendor_directory = Path(os.environ.get("TSLP_VENDOR_DIR", _project_root / "vendor"))
parsers_directory = Path(os.environ.get("TSLP_CACHE_DIR", _project_root / "parsers"))

CLONE_CONCURRENCY = int(os.environ.get("TSLP_CLONE_CONCURRENCY", "16"))
GENERATE_CONCURRENCY = int(os.environ.get("TSLP_GENERATE_CONCURRENCY", "3"))

LANGUAGES_FILTER = set(os.environ.get("TSLP_LANGUAGES", "").split(",")) if os.environ.get("TSLP_LANGUAGES") else set()
GENERATE_TIMEOUT = int(os.environ.get("TSLP_GENERATE_TIMEOUT", "480"))
MIN_COMPATIBLE_ABI = int(os.environ.get("TSLP_MIN_COMPATIBLE_ABI", "13"))
# ~keep Ceiling on what the vendored `tree-sitter` crate can *load*, independent of any
# grammar's requested generation target (`abi_version`, which defaults to 14). Grammars
# exempted from regeneration ship whatever ABI upstream already committed, not a
# regenerated one, so their compatibility bound is the runtime's load ceiling, not the
# generation target. 15 matches tree-sitter 0.26's supported LANGUAGE_VERSION range.
MAX_COMPATIBLE_ABI = int(os.environ.get("TSLP_MAX_COMPATIBLE_ABI", "15"))
ABI_EXEMPT_PARSER_BYTES = int(os.environ.get("TSLP_ABI_EXEMPT_PARSER_BYTES", str(24 * 1024 * 1024)))

CACHE_MANIFEST_FILE = parsers_directory / ".cache_manifest.json"

SHARD_INDEX = int(os.environ.get("TSLP_SHARD_INDEX", "0"))
SHARD_COUNT = int(os.environ.get("TSLP_SHARD_COUNT", "1"))

COMMON_RE_PATTERN = re.compile(r"\.\.[/\\](?:\.\.[/\\])*common[/\\]")

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
_lfs_bundle_lock = asyncio.Lock()


def _no_cache() -> bool:
    """Check if caching is disabled via environment variable."""
    val = os.environ.get("TSLP_NO_CACHE", "").lower()
    return val in ("1", "true", "yes")


class LanguageDict(TypedDict):
    """Language configuration for tree-sitter repositories."""

    repo: NotRequired[str]
    rev: NotRequired[str]
    local: NotRequired[str]
    branch: NotRequired[str]
    directory: NotRequired[str]
    generate: NotRequired[bool]
    rewrite_targets: NotRequired[bool]
    abi_version: NotRequired[int]


def _load_cache_manifest() -> dict[str, str]:
    """Load the cache manifest mapping language names to their cached revisions.

    Returns an empty dict if no manifest exists or caching is disabled.
    """
    if _no_cache():
        return {}

    if CACHE_MANIFEST_FILE.exists():
        try:
            return loads(CACHE_MANIFEST_FILE.read_text())
        except (ValueError, OSError):
            return {}
    return {}


def _save_cache_manifest(manifest: dict[str, str]) -> None:
    """Persist the cache manifest to disk."""
    CACHE_MANIFEST_FILE.parent.mkdir(parents=True, exist_ok=True)
    CACHE_MANIFEST_FILE.write_text(dumps(manifest, indent=2, sort_keys=True) + "\n")


def _language_cache_key(language_definition: LanguageDict) -> str:
    """Produce a deterministic cache key for a language definition.

    Includes repo URL, rev, local path, branch, directory, generate flag, and
    ABI version so that any configuration change invalidates the cache entry.
    For local grammars (no upstream rev), the grammar source content is hashed
    so that editing the in-repo grammar invalidates the cache.
    """
    local = language_definition.get("local", "")
    parts = [
        language_definition.get("repo", ""),
        language_definition.get("rev", ""),
        local,
        language_definition.get("branch", ""),
        language_definition.get("directory", ""),
        str(language_definition.get("generate", False)),
        str(language_definition.get("abi_version", 14)),
        _local_grammar_content_hash(local) if local else "",
    ]
    return hashlib.sha256("|".join(parts).encode()).hexdigest()[:16]


def _local_grammar_content_hash(local: str) -> str:
    """Hash the source files of an in-repo grammar so edits invalidate the cache.

    Hashes every tracked source file under the local grammar directory except
    the generated ``src/`` tree, which is reproduced from ``grammar.js`` anyway.
    """
    grammar_dir = (_project_root / local).resolve()
    hasher = hashlib.sha256()
    for path in sorted(grammar_dir.rglob("*")):
        if not path.is_file() or "src" in path.relative_to(grammar_dir).parts:
            continue
        hasher.update(path.relative_to(grammar_dir).as_posix().encode())
        hasher.update(path.read_bytes())
    return hasher.hexdigest()[:16]


def _is_language_cached(language_name: str, language_definition: LanguageDict, manifest: dict[str, str]) -> bool:
    """Check whether a language's parser files are already cached and up-to-date."""
    if _no_cache():
        return False

    cached_key = manifest.get(language_name)
    if not cached_key:
        return False

    expected_key = _language_cache_key(language_definition)
    if cached_key != expected_key:
        return False

    parser_dir = parsers_directory / language_name / "src"
    return parser_dir.exists() and any(parser_dir.iterdir())


def get_language_definitions() -> tuple[dict[str, LanguageDict], list[str]]:
    """Get the language definitions.

    If TSLP_LANGUAGES is set, only return definitions for those languages.
    """
    print("Loading language definitions")
    language_definitions: dict[str, LanguageDict] = loads(
        (_project_root / "sources" / "language_definitions.json").read_text()
    )
    validate_language_definitions(language_definitions)

    language_names = list(language_definitions.keys())

    if LANGUAGES_FILTER:
        language_names = [name for name in language_names if name in LANGUAGES_FILTER]
        if not language_names:
            print(f"WARNING: TSLP_LANGUAGES={os.environ.get('TSLP_LANGUAGES')} matched no languages")

    return language_definitions, language_names


def _apply_shard(language_names: list[str]) -> list[str]:
    """Return the subset of language names assigned to this shard.

    Partitioning is deterministic: names are sorted, then a strided slice
    ``sorted_names[SHARD_INDEX::SHARD_COUNT]`` is taken. The union of all shards
    equals the full set and shards are pairwise disjoint.

    Raises:
        ValueError: If the shard configuration is invalid.
    """
    if SHARD_COUNT < 1:
        raise ValueError(f"TSLP_SHARD_COUNT must be >= 1, got {SHARD_COUNT}")
    if not 0 <= SHARD_INDEX < SHARD_COUNT:
        raise ValueError(f"TSLP_SHARD_INDEX must be in [0, {SHARD_COUNT}), got {SHARD_INDEX}")
    if SHARD_COUNT == 1:
        return language_names
    shard = sorted(language_names)[SHARD_INDEX::SHARD_COUNT]
    print(f"Shard {SHARD_INDEX + 1}/{SHARD_COUNT}: {len(shard)} of {len(language_names)} language(s)")
    return shard


def _is_transient_git_error(error_str: str) -> bool:
    """Check if a git error looks transient and retryable."""
    transient_patterns = [
        r"Connection reset",
        r"early EOF",
        r"RPC failed",
        r"sideband",
        r"invalid index-pack",
        r"exit code\(128\)",
    ]
    return any(re.search(pattern, error_str, re.IGNORECASE) for pattern in transient_patterns)


def _is_lfs_smudge_error(error_str: str) -> bool:
    """Check if a git error came from Git LFS failing to fetch object content.

    Matches the smudge/transfer signatures rather than any one upstream reason: an exhausted
    LFS budget, a rate limit and a plain outage all reach git through the same failing smudge
    filter and differ only in the message the server returned. ~keep

    Args:
        error_str: The stringified git error.

    Returns:
        True if the failure is an LFS content download failure.
    """
    lfs_patterns = [
        r"smudge filter",
        r"smudge error",
        r"git-lfs",
        r"error downloading object",
        r"batch response",
    ]
    return any(re.search(pattern, error_str, re.IGNORECASE) for pattern in lfs_patterns)


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


async def _hydrate_lfs_pointers(clone_target: Path, language_name: str) -> None:
    """Replace the LFS pointer files in a checkout with verified object content.

    Args:
        clone_target: The checkout made with ``GIT_LFS_SKIP_SMUDGE=1``.
        language_name: The grammar being restored.

    Raises:
        RuntimeError: If the fallback bundle holds no object matching a pointer's oid.
    """
    pointers = await run_sync(_find_lfs_pointers, clone_target)
    if not pointers:
        print(f"[clone_vendors] {language_name}: no LFS pointer files in the checkout", flush=True)
        return

    wanted = dict(pointers.values())
    print(f"[clone_vendors] {language_name}: recovering {len(pointers)} LFS object(s)", flush=True)
    async with _lfs_bundle_lock:
        archive = await run_sync(_download_lfs_fallback_bundle)
    objects = await run_sync(partial(_extract_lfs_objects, archive, wanted))

    for path, (oid, size) in pointers.items():
        content = objects.get(oid)
        if content is None:
            raise RuntimeError(
                f"{language_name}: the LFS fallback bundle {LFS_FALLBACK_URL} contains no object "
                f"with sha256 {oid} ({size} bytes) for {path.relative_to(clone_target)} — refusing "
                "to continue with a pointer file standing in for the real content"
            )
        await run_sync(partial(_write_verified_lfs_object, path, content, oid, size, language_name))


async def _clone_with_lfs_fallback(
    clone_kwargs: dict[str, object],
    rev: str | None,
    language_name: str,
    clone_target: Path,
) -> None:
    """Re-clone with LFS smudging disabled and restore the object content ourselves.

    Args:
        clone_kwargs: The ``Repo.clone_from`` keyword arguments the failed attempt used.
        rev: The revision to check out, if the definition pins one.
        language_name: The grammar being cloned.
        clone_target: The working-tree path of the clone.

    Raises:
        RuntimeError: If the pointer-only clone fails, or an object cannot be recovered.
    """
    if clone_target.exists():
        await run_sync(rmtree, clone_target)

    skip_smudge = {"GIT_LFS_SKIP_SMUDGE": "1"}
    try:
        repo = await run_sync(partial(Repo.clone_from, env=skip_smudge, **clone_kwargs))  # type: ignore[arg-type]
        repo.git.update_environment(**skip_smudge)
        if rev:
            await run_sync(lambda r=repo: r.git.checkout(rev))
    except Exception as e:
        raise RuntimeError(f"failed to clone {language_name} with LFS smudging disabled: {e}") from e

    await _hydrate_lfs_pointers(clone_target, language_name)


async def clone_repository(repo_url: str, branch: str | None, language_name: str, rev: str | None = None) -> None:
    """Clone a repository with retry on transient network errors.

    A clone that fails because Git LFS could not supply object content falls back to a
    pointer-only clone plus content-addressed recovery — see ``_clone_with_lfs_fallback``.
    Nothing about the successful path changes.

    Args:
        repo_url: The repository URL.
        branch: The branch to clone.
        language_name: The name of the repository.
        rev: The revision to clone.  If passed, perform  a non-shallow clone.

    Raises:
        InvalidLanguageSourceError: If the URL, branch or revision is not allowlisted.
        RuntimeError: If cloning fails

    Returns:
        Repo: The cloned repository.
    """
    # ~keep Re-validate at the boundary: these three values are handed to git verbatim,
    # and git parses a leading '-' as an option rather than as a URL/ref.
    validate_repo_url(language_name, repo_url)
    if branch is not None:
        validate_branch(language_name, branch)
    if rev is not None:
        validate_rev(language_name, rev)

    print(f"Cloning {repo_url}")
    clone_target = vendor_directory / language_name

    if clone_target.exists():
        await run_sync(rmtree, clone_target)

    kwargs: dict[str, object] = {"url": repo_url, "to_path": clone_target}
    if branch:
        kwargs["branch"] = branch
    if not rev:
        kwargs["depth"] = 1

    max_attempts = 3
    backoff_delays = [2, 4, 8]

    for attempt in range(max_attempts):
        try:
            repo = await run_sync(partial(Repo.clone_from, **kwargs))  # type: ignore[arg-type]
            print(f"Cloned {repo_url} successfully")
            if rev:
                cloned_repo = repo
                await run_sync(lambda r=cloned_repo: r.git.checkout(rev))
                print(f"Checked out {rev}")
            return
        except Exception as e:  # noqa: PERF203
            error_str = str(e)
            # ~keep An LFS smudge failure is not transient — the object is unavailable to
            # ~keep everyone until upstream fixes their account — so recover instead of
            # ~keep retrying. It has to be tested before the transient check, which its
            # ~keep `exit code(128)` would otherwise match and burn two more full clones on.
            if _is_lfs_smudge_error(error_str):
                print(
                    f"[clone_vendors] {language_name}: Git LFS could not supply object content "
                    f"({e}); re-cloning with pointer files and restoring from the fallback bundle",
                    flush=True,
                )
                await _clone_with_lfs_fallback(kwargs, rev, language_name, clone_target)
                return
            if _is_transient_git_error(error_str) and attempt < max_attempts - 1:
                delay = backoff_delays[attempt]
                print(
                    f"[clone_vendors] retry {attempt + 1}/{max_attempts} for {repo_url} after error: {e}",
                    flush=True,
                )
                await asyncio.sleep(delay)
                if clone_target.exists():
                    await run_sync(rmtree, clone_target)
            else:
                raise RuntimeError(f"failed to clone repo {repo_url} error: {e}") from e


def _committed_parser_abi(target_dir: Path) -> int | None:
    """Read the `LANGUAGE_VERSION` (ABI) from a grammar's committed src/parser.c.

    Args:
        target_dir: The grammar directory containing `src/parser.c`.

    Returns:
        The ABI version, or None if the file is absent or has no version marker.
    """
    parser_c = target_dir / "src" / "parser.c"
    try:
        head = parser_c.read_text(encoding="utf-8", errors="replace")[:4000]
    except OSError:
        return None
    match = re.search(r"LANGUAGE_VERSION\s+(\d+)", head)
    return int(match.group(1)) if match else None


def _should_regenerate(language_name: str, directory: str | None) -> bool:
    """Whether a `generate: true` grammar should actually be regenerated.

    Monster grammars whose freshly-cloned parser.c exceeds ABI_EXEMPT_PARSER_BYTES are
    exempt: regenerating them needs infeasible RAM/time on CI, so they ship that
    parser.c as-is — but only when its own committed ABI still falls within
    `[MIN_COMPATIBLE_ABI, MAX_COMPATIBLE_ABI]`, the range the vendored `tree-sitter`
    crate can actually load. A grammar bump can land a new upstream rev whose
    parser.c both crosses the size threshold AND carries an ABI the runtime cannot
    load, in the same change; without this check the size branch alone decided the
    outcome and shipped that unloadable parser.c unregenerated and unvalidated —
    silently mixing ABI versions into the pack, so the bump would appear to succeed
    while producing a broken or stale parser. Raise instead of exempting so the bump
    fails loudly and needs a human, the same contract `handle_generate`'s timeout
    fallback already enforces for its own ABI check. ~keep

    Args:
        language_name: The grammar name.
        directory: Optional subdirectory within the cloned repo.

    Returns:
        True to regenerate, False to keep the already-cloned parser.c.

    Raises:
        RuntimeError: The parser.c is too large to regenerate and its own ABI falls
            outside the range the runtime can load, so neither path can produce a
            valid parser.
    """
    if ABI_EXEMPT_PARSER_BYTES <= 0:
        return True
    target_dir = (
        (vendor_directory / language_name / directory).resolve()
        if directory
        else (vendor_directory / language_name).resolve()
    )
    try:
        size = (target_dir / "src" / "parser.c").stat().st_size
    except OSError:
        return True
    if size <= ABI_EXEMPT_PARSER_BYTES:
        return True

    abi = _committed_parser_abi(target_dir)
    if abi is None or not (MIN_COMPATIBLE_ABI <= abi <= MAX_COMPATIBLE_ABI):
        raise RuntimeError(
            f"{language_name}: parser.c is {size / 1_000_000:.0f} MB — too large to regenerate "
            f"on standard runners — and its own ABI ({abi}) is outside the range "
            f"[{MIN_COMPATIBLE_ABI}, {MAX_COMPATIBLE_ABI}] the runtime can load, so it cannot ship "
            "as-is either. Raise TSLP_ABI_EXEMPT_PARSER_BYTES to force regeneration, or pin this "
            "grammar to a revision whose committed parser.c is within the loadable ABI range."
        )
    print(
        f"Skipping regeneration of {language_name}: committed parser.c is "
        f"{size / 1_000_000:.0f} MB (ABI {abi}) — too large to regenerate on standard runners; "
        "shipping committed parser.c as-is.",
        flush=True,
    )
    return False


async def _run_generate_with_timeout(cmd: list[str], cwd: str) -> None:
    """Run `tree-sitter generate`, hard-killing its process group on timeout.

    Monster grammars (notably lean, whose committed parser.c is ~44 MB) make the
    parse-table build thrash for many minutes and balloon past runner RAM. The
    previous ``anyio.run_process`` + ``asyncio.wait_for`` combo raised
    ``TimeoutError`` but left the generate subprocess alive: it kept eating memory
    as an orphan while the next grammar started, defeating
    ``GENERATE_CONCURRENCY=1`` and OOM-killing the runner (exit 143). Own the
    process group so the timeout reaps every child before we fall back to the
    committed parser.c.

    Args:
        cmd: The ``tree-sitter generate`` command to run.
        cwd: Working directory for the generate.

    Raises:
        TimeoutError: If generation exceeds ``GENERATE_TIMEOUT`` seconds.
    """
    if platform.system() == "Windows":
        run = run_process(cmd, cwd=cwd, check=False)
        if GENERATE_TIMEOUT > 0:
            await asyncio.wait_for(run, timeout=GENERATE_TIMEOUT)
        else:
            await run
        return

    proc = await asyncio.create_subprocess_exec(
        *cmd,
        cwd=cwd,
        stdout=asyncio.subprocess.DEVNULL,
        stderr=asyncio.subprocess.DEVNULL,
        start_new_session=True,
    )
    try:
        if GENERATE_TIMEOUT > 0:
            await asyncio.wait_for(proc.wait(), timeout=GENERATE_TIMEOUT)
        else:
            await proc.wait()
    except (TimeoutError, asyncio.TimeoutError):
        with suppress(ProcessLookupError):
            os.killpg(os.getpgid(proc.pid), signal.SIGKILL)
        with suppress(ProcessLookupError):
            await proc.wait()
        raise


async def handle_generate(
    language_name: str,
    directory: str | None,
    abi_version: int,
    generate_semaphore: asyncio.Semaphore,
) -> None:
    """Handle the generation of a language.

    Args:
        language_name: The name of the language.
        directory: The directory to generate the language in.
        abi_version: The ABI version to use.
        generate_semaphore: Caps concurrent generation to bound peak memory.

    Raises:
        RuntimeError: if generate fails.

    Returns:
        None
    """
    target_dir = (
        (vendor_directory / language_name / directory).resolve()
        if directory
        else (vendor_directory / language_name).resolve()
    )

    async with generate_semaphore:
        print(f"Generating {language_name} using tree-sitter-cli")
        npm_root = vendor_directory / language_name
        if which("npm") and (npm_root / "package.json").exists():
            # ~keep `npm ci` installs exactly the committed lock file; `npm install` re-resolves
            # ~keep semver ranges, so the same pinned grammar rev could pull different transitive
            # ~keep dependencies on different days. Fall back only when upstream ships no lock
            # ~keep file, where `ci` refuses to run at all.
            npm_subcommand = "ci" if (npm_root / "package-lock.json").exists() else "install"
            npm_args = [npm_subcommand, "--no-audit", "--no-fund", "--ignore-scripts"]
            npm_cmd = ["cmd", "/c", "npm", *npm_args] if platform.system() == "Windows" else ["npm", *npm_args]
            try:
                await run_process(npm_cmd, cwd=str(npm_root), check=False)
            except Exception as e:
                print(f"npm install for {language_name} failed (continuing): {e}")

        if platform.system() == "Windows":
            cmd = ["cmd", "/c", "tree-sitter", "generate", "--abi", str(abi_version)]
        else:
            cmd = ["tree-sitter", "generate", "--abi", str(abi_version)]

        try:
            await _run_generate_with_timeout(cmd, str(target_dir))
            print(f"Generated {language_name} parser successfully")
        except (TimeoutError, asyncio.TimeoutError):
            with suppress(Exception):
                await run_process(
                    ["git", "checkout", "HEAD", "--", "."],
                    cwd=str(vendor_directory / language_name),
                    check=False,
                )
            committed_abi = _committed_parser_abi(target_dir)
            if committed_abi is not None and MIN_COMPATIBLE_ABI <= committed_abi <= abi_version:
                print(
                    f"WARNING: tree-sitter generate for {language_name} timed out after "
                    f"{GENERATE_TIMEOUT}s; using committed parser.c "
                    f"(ABI {committed_abi}, within [{MIN_COMPATIBLE_ABI}, {abi_version}])",
                    flush=True,
                )
            else:
                raise RuntimeError(
                    f"tree-sitter generate for {language_name} timed out after {GENERATE_TIMEOUT}s "
                    f"and its committed parser.c ABI ({committed_abi}) is outside the target "
                    f"[{MIN_COMPATIBLE_ABI}, {abi_version}] — cannot honor the ABI-{abi_version} "
                    f"contract. Raise TSLP_GENERATE_TIMEOUT or use a larger runner so generation completes."
                ) from None
        except Exception as e:
            raise RuntimeError(f"failed to clone {language_name} due to an exception: {e}") from e


def _is_query_like_path(path: Path) -> bool:
    """Check if a path looks like it contains queries.

    Returns True if the path contains a "query"-like directory or is under
    editors/*/ or integrations/*/queries/.
    """
    path_str = str(path)
    return (
        any(component in path_str for component in ("queries", "queries-flavored", "editor_queries", "nvim-queries"))
        or "/editors/" in path_str
        or ("/integrations/" in path_str and "queries" in path_str)
    )


def _is_skip_path(path: Path) -> bool:
    """Check if a path should be skipped.

    Skips paths containing test/example/untested directories.
    """
    parts = path.parts
    skip_segments = {"test", "example", "untested"}

    return any(part in skip_segments for part in parts)


def _get_editor_score(name: str) -> int:
    """Get priority score based on editor name (0 if not editor)."""
    if "nvim" in name or "neovim" in name:
        return 4
    if "helix" in name:
        return 5
    if any(ed in name for ed in ("emacs", "zed", "lapce")):
        return 6
    return 0


def _score_editor_query(parts: tuple[str, ...], path_str: str) -> int | None:
    """Score an editor-flavored query path (nvim/helix/other), or None if not one.

    Args:
        parts: Path components of the candidate.
        path_str: The candidate path as a string.

    Returns:
        4 (nvim), 5 (helix), an editor-specific score, or None if not editor-flavored.
    """
    if "nvim-queries" in parts or ("integrations" in parts and "nvim" in path_str):
        return 4
    if "queries-flavored" in parts:
        idx = parts.index("queries-flavored")
        subdir = parts[idx + 1] if idx + 1 < len(parts) else ""
        return _get_editor_score(subdir) or 3
    if "helix" in path_str and "integrations" in parts:
        return 5
    if "integrations" in parts:
        return _get_editor_score(path_str)
    return None


def _is_foreign_grammar_query(
    candidate: Path,
    directory: str | None,
    language_name: str | None = None,
    scoped_subdirs: frozenset[str] = frozenset(),
) -> bool:
    """Report whether a candidate is scoped to a *different* language than the one we want.

    Two layouts scope a query file to a specific grammar: `<grammar>/queries/<file>` and
    `queries/<grammar>/<file>`. A candidate scoped to some *other* grammar is never a valid
    substitute — its node types belong to a different language, so it either fails to compile
    or, worse, silently produces captures keyed to the wrong grammar.

    This is not limited to monorepos declaring `directory`. Several single-grammar repos vendor
    nvim-treesitter's whole `runtime/queries/<language>/` bundle for unrelated editor-plugin
    reasons, which puts dozens of other languages' query files in scope. `leo` acquired its
    `locals` from `runtime/queries/m68k/` and its `indents` from `runtime/queries/ocaml/` that
    way — every candidate tied on score and traversal order decided it. So the scoped-to-another
    -language test applies whenever we know the name we are looking for, `directory` or not.

    A repo-root `queries/<file>` is scoped to nothing and stays eligible as a fallback. ~keep

    Args:
        candidate: The candidate query file path (relative to the repo root).
        directory: Optional grammar subdirectory within the repo.
        language_name: The language being vendored, used when the repo scopes by language name.
        scoped_subdirs: Every `<x>` seen in a `queries/<x>/<file>` candidate in this repo.

    Returns:
        True if the candidate is scoped to a different grammar and must not be used.
    """
    parts = candidate.parts
    if len(parts) < 3:
        return False

    # ~keep `<x>/queries/<file>`: only a monorepo declaring `directory` scopes queries this
    # ~keep way. Repos that embed another grammar as a submodule use the same shape without
    # ~keep meaning "different language" — arduino/cuda/ispc pull tree-sitter-cpp and -c in,
    # ~keep and those queries do apply, since those grammars are supersets. So this clause
    # ~keep stays keyed on `directory`, not on the language name.
    if parts[-2] == "queries":
        return bool(directory) and parts[-3] != directory.split("/")[-1]

    if parts[-3] != "queries" or _get_editor_score(parts[-2]):
        return False

    # ~keep `queries/<x>/<file>`: whether `<x>` means "a different language" depends on how
    # ~keep many such subdirectories the repo has. One means the repo simply scopes its own
    # ~keep queries under a name that need not equal ours — nushell ships `queries/nu/`, and
    # ~keep rejecting it on a name mismatch loses that grammar's only queries. Several means
    # ~keep it is a per-language bundle: leo vendors nvim-treesitter's whole `runtime/queries/`
    # ~keep tree, and took its locals from m68k and its indents from ocaml purely on traversal
    # ~keep order. Only then is a non-matching name evidence of the wrong language.
    if len(scoped_subdirs) <= 1:
        return False

    own_names = {name for name in (directory.split("/")[-1] if directory else None, language_name) if name}
    return parts[-2] not in own_names


def _score_query_candidate(candidate: Path, directory: str | None) -> int:
    """Score a query file candidate for priority selection.

    Lower score = higher priority. Scores: 1 (grammar-scoped queries), 2 (root queries),
    3 (generic nested), 4-6 (editor-flavored), 100 (fallback).

    Args:
        candidate: The candidate query file path (relative to the repo root).
        directory: Optional grammar subdirectory within the repo.

    Returns:
        The priority score (lower is preferred).
    """
    parts = candidate.parts
    path_str = str(candidate)

    if directory:
        dir_last = directory.split("/")[-1]
        # ~keep `parts` ends with the filename, so the two grammar-scoped layouts both put
        # ~keep it last: `<grammar>/queries/<file>` and `queries/<grammar>/<file>`. The old
        # ~keep check asked for `i + 2 == len(parts)`, which is one short and never held, so
        # ~keep this branch was dead: every sibling grammar's queries tied at score 2 and the
        # ~keep winner was whichever rglob happened to reach first.
        if len(parts) >= 3 and parts[-3] == dir_last and parts[-2] == "queries":
            return 1
        if len(parts) >= 3 and parts[-3] == "queries" and parts[-2] == dir_last:
            return 1

    if "queries" in parts and len(parts) == parts.index("queries") + 2:
        return 2

    editor_score = _score_editor_query(parts, path_str)
    if editor_score is not None:
        return editor_score

    for query_dir in ("queries", "editor_queries"):
        if query_dir in parts:
            idx = parts.index(query_dir)
            if idx + 2 == len(parts) - 1:
                return _get_editor_score(parts[idx + 1]) or 3

    return _get_editor_score(path_str) or 100


async def _discover_and_copy_queries(
    language_name: str,
    directory: str | None,
    vendor_repo: Path,
    target_queries_dir: Path,
) -> None:
    """Discover and copy query files from the vendor repo with priority-based selection.

    For each standard query type (highlights.scm, injections.scm, locals.scm,
    indents.scm, folds.scm, tags.scm), finds the best candidate file in the
    vendor repo and copies it to target_queries_dir.

    Args:
        language_name: The name of the language.
        directory: The subdirectory within the vendor repo (if any).
        vendor_repo: The path to the cloned vendor repository.
        target_queries_dir: The target directory to copy queries to.
    """
    query_types = ["highlights.scm", "injections.scm", "locals.scm", "indents.scm", "folds.scm", "tags.scm"]

    all_scm_files: dict[str, list[Path]] = {qtype: [] for qtype in query_types}

    for scm_file in vendor_repo.rglob("*.scm"):
        if not _is_query_like_path(scm_file):
            continue

        if _is_skip_path(scm_file):
            continue

        filename = scm_file.name
        if filename in query_types:
            all_scm_files[filename].append(scm_file)

    scoped_subdirs = frozenset(
        candidate.relative_to(vendor_repo).parts[-2]
        for candidates in all_scm_files.values()
        for candidate in candidates
        if len(candidate.relative_to(vendor_repo).parts) >= 3
        and candidate.relative_to(vendor_repo).parts[-3] == "queries"
        and not _get_editor_score(candidate.relative_to(vendor_repo).parts[-2])
    )

    has_own_queries_dir = bool(directory) and any(
        _score_query_candidate(candidate.relative_to(vendor_repo), directory) == 1
        for candidates in all_scm_files.values()
        for candidate in candidates
    )

    for query_type in query_types:
        candidates = all_scm_files[query_type]
        if not candidates:
            continue

        # ~keep Score on the repo-relative path, which is the contract the scorer documents.
        # ~keep Absolute paths broke it: the vendor checkout directory is itself named after
        # ~keep the language, so a repo-root `queries/` sat at `<lang>/queries/<file>` and
        # ~keep scored as if it were the grammar's own directory, tying with the real one.
        relative = [(candidate, candidate.relative_to(vendor_repo)) for candidate in candidates]
        eligible = [
            (c, r) for c, r in relative if not _is_foreign_grammar_query(r, directory, language_name, scoped_subdirs)
        ]
        if not eligible:
            print(f"Skipping {language_name} {query_type}: only other grammars in this repo provide it")
            continue

        # ~keep When upstream gives this grammar its own queries directory, that directory is
        # ~keep authoritative: a kind missing from it is missing deliberately, and the repo-root
        # ~keep set belongs to the sibling grammar. Falling back to root regardless is how
        # ~keep fsharp_signature ended up with fsharp's locals/indents, which name node types
        # ~keep the signature grammar does not have and so fail to compile.
        if has_own_queries_dir and all(_score_query_candidate(rel, directory) != 1 for _, rel in eligible):
            print(f"Skipping {language_name} {query_type}: not provided by {directory}/queries")
            continue

        scored = [(candidate, _score_query_candidate(rel, directory)) for candidate, rel in eligible]
        scored.sort(key=lambda x: x[1])
        best_candidate = scored[0][0]

        target_file = target_queries_dir / query_type
        try:
            print(f"Copying {language_name} {query_type} from {best_candidate.relative_to(vendor_repo)}")
            await AsyncPath(target_queries_dir).mkdir(parents=True, exist_ok=True)
            await AsyncPath(target_file).write_text(await AsyncPath(best_candidate).read_text())
        except Exception as e:
            print(f"Warning: failed to copy {language_name} {query_type}: {e}")


async def move_src_folder(language_name: str, directory: str | None) -> None:
    """Move the src folder to the parsers directory and discover/copy queries.

    Args:
        language_name: The name of the language.
        directory: The directory to move the src folder from.

    Returns:
        None
    """
    print(f"Moving {language_name} parser files")
    source_dir = (
        (vendor_directory / language_name / directory / "src").resolve()
        if directory
        else (vendor_directory / language_name / "src").resolve()
    )
    target_source_dir = (parsers_directory / language_name).resolve()
    target_src = target_source_dir / "src"
    if target_src.exists():
        await run_sync(rmtree, target_src)
    await AsyncPath(target_source_dir).mkdir(parents=True, exist_ok=True)
    await run_sync(move, source_dir, target_source_dir)
    print(f"Moved {language_name} parser files successfully")

    common_source_dir = vendor_directory / language_name / "common"

    if await AsyncPath(common_source_dir).exists():
        print(f"Moving {language_name} common files")
        target_common = target_source_dir / "common"
        if target_common.exists():
            await run_sync(rmtree, target_common)
        await run_sync(move, common_source_dir, target_source_dir)
        print(f"Moved {language_name} common files successfully")

        for file in target_source_dir.glob("**/*.c"):
            file_contents = await AsyncPath(file).read_text()

            replacement_path = os.path.relpath(target_source_dir / "common", file.parent)

            replacement_path = replacement_path.replace("\\", "/") + "/"

            file_contents = COMMON_RE_PATTERN.sub(replacement_path, file_contents)
            await AsyncPath(file).write_text(file_contents)

    vendor_repo = vendor_directory / language_name
    target_queries = target_source_dir / "queries"
    if target_queries.exists():
        await run_sync(rmtree, target_queries)

    await _discover_and_copy_queries(language_name, directory, vendor_repo, target_queries)


async def copy_local_grammar(local_path: str, language_name: str) -> None:
    """Stage an in-repo grammar into the vendor working directory.

    Local grammars are maintained in-tree (e.g. ``grammars/graphql``) instead of
    being cloned from an upstream repo. Copying them into ``vendor/<lang>`` lets
    the existing generate / move-src / query-discovery pipeline run unchanged.

    Args:
        local_path: Grammar directory relative to the project root.
        language_name: The name of the language.

    Raises:
        RuntimeError: If the local grammar directory does not exist.
    """
    source_dir = (_project_root / local_path).resolve()
    if not source_dir.is_dir():
        raise RuntimeError(f"local grammar directory not found for {language_name}: {source_dir}")
    clone_target = vendor_directory / language_name
    if clone_target.exists():
        await run_sync(rmtree, clone_target)
    print(f"Staging local grammar {local_path} for {language_name}")
    await run_sync(partial(copytree, source_dir, clone_target))


async def process_repo(
    language_name: str,
    language_definition: LanguageDict,
    generate_semaphore: asyncio.Semaphore,
) -> None:
    """Process a repository.

    Args:
        language_name: The name of the language.
        language_definition: The language definition.
        generate_semaphore: Caps concurrent generation to bound peak memory.

    Returns:
        None
    """
    local = language_definition.get("local")
    if local:
        await copy_local_grammar(local_path=local, language_name=language_name)
    else:
        await clone_repository(
            repo_url=language_definition["repo"],
            branch=language_definition.get("branch"),
            language_name=language_name,
            rev=language_definition.get("rev"),
        )
    directory = language_definition.get("directory")
    abi_version = language_definition.get("abi_version", 14)
    if language_definition.get("generate", False) and _should_regenerate(language_name, directory):
        await handle_generate(
            language_name=language_name,
            directory=directory,
            abi_version=abi_version,
            generate_semaphore=generate_semaphore,
        )
    await move_src_folder(language_name=language_name, directory=directory)

    clone_dir = vendor_directory / language_name
    if await AsyncPath(clone_dir).exists():
        await run_sync(partial(rmtree, ignore_errors=True), clone_dir)


async def main() -> None:
    """Main function."""
    sys.stdout.reconfigure(line_buffering=True)

    parsers_directory.mkdir(exist_ok=True, parents=True)

    language_definitions, language_names = get_language_definitions()
    language_names = _apply_shard(language_names)
    manifest = _load_cache_manifest()

    to_process: list[str] = []
    cached_count = 0
    for name in language_names:
        if _is_language_cached(name, language_definitions[name], manifest):
            cached_count += 1
        else:
            to_process.append(name)

    if cached_count > 0:
        print(f"Cache hit: {cached_count} language(s) already up-to-date, skipping")
    if not to_process:
        print("All languages cached — nothing to do")
        return

    print(f"Processing {len(to_process)} language(s)...")

    semaphore = asyncio.Semaphore(CLONE_CONCURRENCY)
    generate_semaphore = asyncio.Semaphore(GENERATE_CONCURRENCY)
    print(f"Concurrency: clone={CLONE_CONCURRENCY}, generate={GENERATE_CONCURRENCY}")

    async def bounded_process(name: str, defn: LanguageDict) -> None:
        async with semaphore:
            await process_repo(
                language_name=name,
                language_definition=defn,
                generate_semaphore=generate_semaphore,
            )

    await asyncio.gather(
        *[
            bounded_process(
                name=language_name,
                defn=language_definitions[language_name],
            )
            for language_name in to_process
        ]
    )

    for name in to_process:
        manifest[name] = _language_cache_key(language_definitions[name])

    # The stale sweep compares the manifest against the languages this run resolved, which only
    # describes the whole tree on an unfiltered run. Under TSLP_LANGUAGES that set is a subset,
    # so sweeping against it deletes every grammar outside the filter: refreshing three grammars
    # pruned a populated tree from 371 entries down to 3. A filtered run owns only its own
    # entries and leaves the rest of the manifest alone. ~keep
    if not LANGUAGES_FILTER:
        for stale in set(manifest) - set(language_names):
            del manifest[stale]
            stale_dir = parsers_directory / stale
            if stale_dir.exists():
                rmtree(stale_dir)
                print(f"Removed stale parser: {stale}")

    _save_cache_manifest(manifest)
    print(f"Cache manifest updated ({len(manifest)} entries)")


if __name__ == "__main__":
    if not which("tree-sitter"):
        sys.exit("tree-sitter is a required system dependency. Please install it with 'npm i -g tree-sitter-cli'")

    if _no_cache():
        print("Caching disabled (TSLP_NO_CACHE=1) — performing full clone")
        if vendor_directory.exists():
            rmtree(vendor_directory)
        if parsers_directory.exists():
            rmtree(parsers_directory)
    elif vendor_directory.exists():
        print("Cleaning vendor directory")
        rmtree(vendor_directory)

    asyncio.run(main())
