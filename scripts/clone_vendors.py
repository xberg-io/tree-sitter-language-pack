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
import sys
from contextlib import suppress
from functools import partial
from json import dumps, loads
from pathlib import Path
from shutil import copytree, move, rmtree, which
from typing import NotRequired, TypedDict

from _vendor_lfs import (
    LFS_FALLBACK_URL,
    _download_lfs_fallback_bundle,
    _extract_lfs_objects,
    _find_lfs_pointers,
    _write_verified_lfs_object,
)
from _vendor_queries import (
    QUERY_TYPES,
    _collect_query_candidates,
    _has_own_queries_dir,
    _scoped_query_subdirs,
    _select_best_query,
)
from _vendor_sources import (
    validate_branch,
    validate_language_definitions,
    validate_repo_url,
    validate_rev,
)
from anyio import Path as AsyncPath
from anyio import run_process
from anyio.to_thread import run_sync
from git import Repo

_project_root = Path(__file__).parent.parent

# ~keep The repository-location subset of `git rev-parse --local-env-vars` (the GIT_CONFIG*
# ones are left alone so injected config such as auth or insteadOf survives). A git hook
# exports these for the committing repo; inherited by vendor clones they redirect every `git`
# call there, so the pre-commit hook's build overwrote the committing repo's index.
_REPO_LOCAL_GIT_ENV = (
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_OBJECT_DIRECTORY",
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_IMPLICIT_WORK_TREE",
    "GIT_GRAFT_FILE",
    "GIT_INDEX_FILE",
    "GIT_NO_REPLACE_OBJECTS",
    "GIT_REPLACE_REF_BASE",
    "GIT_PREFIX",
    "GIT_SHALLOW_FILE",
    "GIT_COMMON_DIR",
)


def _scrub_repo_local_git_env() -> None:
    for name in _REPO_LOCAL_GIT_ENV:
        os.environ.pop(name, None)


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
    all_scm_files = _collect_query_candidates(vendor_repo)
    scoped_subdirs = _scoped_query_subdirs(all_scm_files, vendor_repo)
    has_own_queries_dir = _has_own_queries_dir(all_scm_files, vendor_repo, directory)

    for query_type in QUERY_TYPES:
        candidates = all_scm_files[query_type]
        if not candidates:
            continue

        best_candidate = _select_best_query(
            candidates,
            query_type,
            language_name=language_name,
            directory=directory,
            vendor_repo=vendor_repo,
            scoped_subdirs=scoped_subdirs,
            has_own_queries_dir=has_own_queries_dir,
        )
        if best_candidate is None:
            continue

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
    _scrub_repo_local_git_env()

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
