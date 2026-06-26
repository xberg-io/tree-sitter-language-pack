# /// script
# requires-python = ">=3.14"
# dependencies = ["anyio", "gitpython"]
# ///
import asyncio
import hashlib
import os
import platform
import re
import sys
from functools import partial
from json import dumps, loads
from pathlib import Path
from shutil import move, rmtree, which
from typing import NotRequired, TypedDict

from anyio import Path as AsyncPath
from anyio import run_process
from anyio.to_thread import run_sync
from git import Repo

# ---------------------------------------------------------------------------
# Configuration via environment variables
# ---------------------------------------------------------------------------
# TSLP_CACHE_DIR    — Override the default parsers directory location.
# TSLP_NO_CACHE     — Set to "1" or "true" to force a full re-clone, ignoring
#                      the cache manifest.
# TSLP_VENDOR_DIR   — Override the default vendor directory location.
# TSLP_CLONE_CONCURRENCY    — Max concurrent repo clones (default 16). Clones are
#                      network/IO-bound and memory-light.
# TSLP_GENERATE_CONCURRENCY — Max concurrent `tree-sitter generate` runs
#                      (default 3). Generation is memory-heavy; running too many
#                      in parallel exhausts the 7 GB RAM on GitHub-hosted runners
#                      and gets the job OOM-killed (SIGTERM), so it is capped well
#                      below the clone concurrency independently.
# ---------------------------------------------------------------------------

_project_root = Path(__file__).parent.parent

vendor_directory = Path(os.environ.get("TSLP_VENDOR_DIR", _project_root / "vendor"))
parsers_directory = Path(os.environ.get("TSLP_CACHE_DIR", _project_root / "parsers"))

# Concurrency limits. `tree-sitter generate` peaks at ~1 GB+ RSS for large
# grammars, so a high generate fan-out OOM-kills the 7 GB CI runners; clones are
# cheap and stay wide. Both are env-tunable for constrained environments.
CLONE_CONCURRENCY = int(os.environ.get("TSLP_CLONE_CONCURRENCY", "16"))
GENERATE_CONCURRENCY = int(os.environ.get("TSLP_GENERATE_CONCURRENCY", "3"))

CACHE_MANIFEST_FILE = parsers_directory / ".cache_manifest.json"

# Sharding lets CI split the ~306-grammar clone+generate across parallel jobs so
# no single job runs long enough to be reaped by hosted-runner reclamation
# (exit 143). A shard takes the strided slice sorted_names[index::count]; striding
# (not contiguous chunks) spreads alphabetically-clustered heavy grammars evenly.
SHARD_INDEX = int(os.environ.get("TSLP_SHARD_INDEX", "0"))
SHARD_COUNT = int(os.environ.get("TSLP_SHARD_COUNT", "1"))

COMMON_RE_PATTERN = re.compile(r"\.\.[/\\](?:\.\.[/\\])*common[/\\]")


def _no_cache() -> bool:
    """Check if caching is disabled via environment variable."""
    val = os.environ.get("TSLP_NO_CACHE", "").lower()
    return val in ("1", "true", "yes")


class LanguageDict(TypedDict):
    """Language configuration for tree-sitter repositories."""

    repo: str
    rev: str
    branch: NotRequired[str]
    directory: NotRequired[str]
    generate: NotRequired[bool]
    rewrite_targets: NotRequired[bool]
    abi_version: NotRequired[int]


# ---------------------------------------------------------------------------
# Cache manifest helpers
# ---------------------------------------------------------------------------


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

    Includes repo URL, rev, branch, directory, generate flag, and ABI version
    so that any configuration change invalidates the cache entry.
    """
    parts = [
        language_definition["repo"],
        language_definition.get("rev", ""),
        language_definition.get("branch", ""),
        language_definition.get("directory", ""),
        str(language_definition.get("generate", False)),
        str(language_definition.get("abi_version", 14)),
    ]
    return hashlib.sha256("|".join(parts).encode()).hexdigest()[:16]


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

    # Verify the parser directory actually contains files
    parser_dir = parsers_directory / language_name / "src"
    return parser_dir.exists() and any(parser_dir.iterdir())


# ---------------------------------------------------------------------------
# Language definitions
# ---------------------------------------------------------------------------


def get_language_definitions() -> tuple[dict[str, LanguageDict], list[str]]:
    """Get the language definitions."""
    print("Loading language definitions")
    language_definitions: dict[str, LanguageDict] = loads(
        (_project_root / "sources" / "language_definitions.json").read_text()
    )

    language_names = list(language_definitions.keys())
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


# ---------------------------------------------------------------------------
# Clone / generate / move
# ---------------------------------------------------------------------------


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


async def clone_repository(repo_url: str, branch: str | None, language_name: str, rev: str | None = None) -> None:
    """Clone a repository with retry on transient network errors.

    Args:
        repo_url: The repository URL.
        branch: The branch to clone.
        language_name: The name of the repository.
        rev: The revision to clone.  If passed, perform  a non-shallow clone.

    Raises:
        RuntimeError: If cloning fails

    Returns:
        Repo: The cloned repository.
    """
    print(f"Cloning {repo_url}")
    clone_target = vendor_directory / language_name

    # Clean up any stale vendor directory for this language
    if clone_target.exists():
        await run_sync(rmtree, clone_target)

    kwargs = {"url": repo_url, "to_path": clone_target}
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
            if _is_transient_git_error(error_str) and attempt < max_attempts - 1:
                delay = backoff_delays[attempt]
                print(f"[clone_vendors] retry {attempt + 1}/{max_attempts} for {repo_url} after error: {e}", flush=True)
                await asyncio.sleep(delay)
                # Clean up failed clone attempt for next retry
                if clone_target.exists():
                    await run_sync(rmtree, clone_target)
            else:
                raise RuntimeError(f"failed to clone repo {repo_url} error: {e}") from e


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

    # `npm install` + `tree-sitter generate` are the memory-heavy steps; hold the
    # generate semaphore across both so at most GENERATE_CONCURRENCY run at once
    # (a wide fan-out OOM-kills 7 GB CI runners). Clones already finished outside.
    async with generate_semaphore:
        print(f"Generating {language_name} using tree-sitter-cli")
        # Some grammar.js files `require()` JS dependencies — a shared `dsl`/helper
        # package, or a sibling grammar they extend (e.g. cpp→tree-sitter-c,
        # templ→tree-sitter-go). `tree-sitter generate` loads grammar.js with Node,
        # so those deps must be installed first. Install at the repo root (where
        # package.json lives) when present; a no-op for self-contained grammars.
        npm_root = vendor_directory / language_name
        if which("npm") and (npm_root / "package.json").exists():
            # `--ignore-scripts` skips the dependencies' native `node-gyp` builds
            # (which can fail and are irrelevant here) — generate only needs the JS
            # grammar modules a `require()` pulls in (e.g. tree-sitter-cpp/-go).
            npm_args = ["install", "--no-audit", "--no-fund", "--ignore-scripts"]
            npm_cmd = ["cmd", "/c", "npm", *npm_args] if platform.system() == "Windows" else ["npm", *npm_args]
            try:
                await run_process(npm_cmd, cwd=str(npm_root), check=False)
            except Exception as e:  # noqa: BLE001 - npm deps are best-effort; generate reports the real failure
                print(f"npm install for {language_name} failed (continuing): {e}")

        if platform.system() == "Windows":
            cmd = ["cmd", "/c", "tree-sitter", "generate", "--abi", str(abi_version)]
        else:
            cmd = ["tree-sitter", "generate", "--abi", str(abi_version)]

        try:
            await run_process(cmd, cwd=str(target_dir), check=False)
            print(f"Generated {language_name} parser successfully")
        except Exception as e:
            raise RuntimeError(f"failed to clone {language_name} due to an exception: {e}") from e


async def move_src_folder(language_name: str, directory: str | None) -> None:
    """Move the src folder to the parsers directory.

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
    # Clean existing parser dir to avoid "already exists" errors on re-runs
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

    # Copy queries/ directory if present in the vendor repo
    queries_source_dir = (
        (vendor_directory / language_name / directory / "queries").resolve()
        if directory
        else (vendor_directory / language_name / "queries").resolve()
    )
    if await AsyncPath(queries_source_dir).exists():
        print(f"Copying {language_name} queries")
        target_queries = target_source_dir / "queries"
        if target_queries.exists():
            await run_sync(rmtree, target_queries)
        await run_sync(move, queries_source_dir, target_source_dir)
        print(f"Copied {language_name} queries successfully")


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
    await clone_repository(
        repo_url=language_definition["repo"],
        branch=language_definition.get("branch"),
        language_name=language_name,
        rev=language_definition.get("rev"),
    )
    if language_definition.get("generate", False):
        await handle_generate(
            language_name=language_name,
            directory=language_definition.get("directory"),
            abi_version=language_definition.get("abi_version", 14),
            generate_semaphore=generate_semaphore,
        )
    await move_src_folder(language_name=language_name, directory=language_definition.get("directory"))

    # Free the vendor clone immediately: its parser src/common/queries are now in
    # parsers/, leaving only the .git history, grammar.js, and node_modules (the
    # bulk). Without this, all 306 clones + ~140 node_modules accumulate to ~8 GB
    # and crowd the ~14 GB CI runner disk; deleting per-grammar keeps the peak to
    # the in-flight working set.
    clone_dir = vendor_directory / language_name
    if await AsyncPath(clone_dir).exists():
        await run_sync(partial(rmtree, ignore_errors=True), clone_dir)


async def main() -> None:
    """Main function."""
    # Line-buffer stdout so per-grammar progress is visible live in CI logs
    # (block buffering otherwise hides all progress until exit — useless when a
    # run is killed mid-way and the buffer is lost).
    sys.stdout.reconfigure(line_buffering=True)

    parsers_directory.mkdir(exist_ok=True, parents=True)

    language_definitions, language_names = get_language_definitions()
    language_names = _apply_shard(language_names)
    manifest = _load_cache_manifest()

    # Partition languages into cached (skip) and stale (need processing)
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

    # Clones are wide (network-bound); generation is throttled separately because
    # it is memory-heavy and a wide fan-out OOM-kills the 7 GB CI runners.
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

    # Update manifest with newly processed languages
    for name in to_process:
        manifest[name] = _language_cache_key(language_definitions[name])

    # Remove manifest entries for languages no longer in definitions
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
    # Only clean vendor (temporary clones); parsers directory is the cache
    elif vendor_directory.exists():
        print("Cleaning vendor directory")
        rmtree(vendor_directory)

    asyncio.run(main())
