import argparse
import asyncio
import json
import os
from pathlib import Path
from typing import Any

from anyio import run_process


async def get_latest_commit_hash(repo_url: str, branch: str | None = None) -> str | None:
    """Get the latest commit hash using git ls-remote (no clone needed).

    Args:
        repo_url: The repository URL.
        branch: The branch to query (defaults to HEAD).

    Returns:
        The latest commit hash, or None on failure.
    """
    ref = f"refs/heads/{branch}" if branch else "HEAD"
    try:
        result = await run_process(["git", "ls-remote", repo_url, ref], check=True)
        output: str = result.stdout.decode().strip()
        if output:
            commit_hash: str = output.split("\t", maxsplit=1)[0]
            return commit_hash
        return None
    except (OSError, RuntimeError, ValueError) as e:
        print(f"Error fetching commit for {repo_url}: {e}")
        return None


async def process_language(
    language_name: str, language_def: dict[str, Any], *, only_missing: bool
) -> tuple[str, dict[str, Any]]:
    """Process a language repository to get its latest commit."""
    language_def_copy = language_def.copy()

    if only_missing and "rev" in language_def_copy:
        return language_name, language_def_copy

    repo_url = language_def["repo"]
    branch = language_def.get("branch")

    latest_commit = await get_latest_commit_hash(repo_url, branch)

    if latest_commit:
        language_def_copy["rev"] = latest_commit
        print(f"✓ {language_name} → {latest_commit[:12]}")
    else:
        print(f"✗ {language_name} — failed")

    return language_name, language_def_copy


async def main(args: argparse.Namespace) -> None:
    """Main function."""
    max_workers = args.workers or min(64, (os.cpu_count() or 4) * 4)
    print(f"Fetching latest commits with concurrency={max_workers}")

    definitions_path = Path(__file__).parent.parent / "sources" / "language_definitions.json"
    language_definitions: dict[str, Any] = json.loads(definitions_path.read_text())

    if args.languages:
        requested = set(args.languages.split(","))
        subset = {k: v for k, v in language_definitions.items() if k in requested}
        print(f"Processing {len(subset)} specified languages")
    else:
        subset = language_definitions
        print(f"Processing all {len(subset)} languages")

    semaphore = asyncio.Semaphore(max_workers)

    async def bounded(name: str, defn: dict[str, Any]) -> tuple[str, dict[str, Any]]:
        async with semaphore:
            return await process_language(name, defn, only_missing=args.only_missing)

    results = await asyncio.gather(*(bounded(name, defn) for name, defn in subset.items()))

    updated = dict(results)

    if args.languages:
        # Merge back into full definitions
        all_definitions = json.loads(definitions_path.read_text())
        all_definitions.update(updated)
        updated = all_definitions

    definitions_path.write_text(json.dumps(updated, indent=2) + "\n")
    print(f"Done! Updated {len(updated)} language definitions.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Pin tree-sitter language repositories to their latest commits.")
    parser.add_argument("--languages", type=str, help="Comma-separated list of languages to process (default: all)")
    parser.add_argument("--workers", type=int, help="Maximum concurrent fetches (default: CPU count * 4)")
    parser.add_argument("--only-missing", action="store_true", help="Only update languages without an existing rev")

    args = parser.parse_args()
    asyncio.run(main(args))
