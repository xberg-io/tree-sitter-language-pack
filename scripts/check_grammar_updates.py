import argparse
import asyncio
import json
import os
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from anyio import run_process

DEFINITIONS_PATH = Path(__file__).parent.parent / "sources" / "language_definitions.json"
MAX_WORKERS_DEFAULT = 64


@dataclass
class StaleGrammar:
    """A grammar whose pinned rev differs from upstream HEAD."""

    language: str
    repo: str
    current_rev: str
    latest_rev: str


@dataclass
class CheckReport:
    """Result of checking all grammars for updates."""

    stale: list[StaleGrammar] = field(default_factory=list)
    up_to_date: int = 0
    failed: list[str] = field(default_factory=list)

    @property
    def stale_count(self) -> int:
        return len(self.stale)

    def to_dict(self) -> dict[str, Any]:
        return {
            "stale": [
                {
                    "language": s.language,
                    "repo": s.repo,
                    "current_rev": s.current_rev,
                    "latest_rev": s.latest_rev,
                }
                for s in sorted(self.stale, key=lambda s: s.language)
            ],
            "up_to_date": self.up_to_date,
            "stale_count": self.stale_count,
            "failed": sorted(self.failed),
        }


async def get_latest_commit_hash(repo_url: str, branch: str | None = None) -> str | None:
    """Fetch the HEAD commit hash from a remote repo using git ls-remote.

    Args:
        repo_url: The repository URL.
        branch: Optional branch name; defaults to HEAD if not provided.

    Returns:
        The commit hash string, or None if the query failed.
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
        print(f"  ERROR fetching {repo_url}: {e}", file=sys.stderr)
        return None


async def check_language(
    language: str,
    definition: dict[str, Any],
    semaphore: asyncio.Semaphore,
) -> StaleGrammar | None | str:
    """Check a single language grammar for updates.

    Returns:
        - StaleGrammar if the grammar has a newer commit available.
        - None if the grammar is up to date.
        - The language name (str) if the check failed.
    """
    async with semaphore:
        if "rev" not in definition:
            # No pinned rev — skip; pin_vendors.py handles initial pinning.
            return None

        repo_url = definition["repo"]
        branch = definition.get("branch")
        current_rev = definition["rev"]

        latest_rev = await get_latest_commit_hash(repo_url, branch)

        if latest_rev is None:
            print(f"  FAIL {language}", file=sys.stderr)
            return language  # Sentinel: failed check

        if latest_rev != current_rev:
            print(f"  STALE {language}: {current_rev[:12]} -> {latest_rev[:12]}")
            return StaleGrammar(
                language=language,
                repo=repo_url,
                current_rev=current_rev,
                latest_rev=latest_rev,
            )

        return None  # Up to date


async def build_report(
    language_definitions: dict[str, Any],
    max_workers: int,
    languages: list[str] | None = None,
) -> CheckReport:
    """Check all (or selected) grammars and build a report.

    Args:
        language_definitions: The full contents of language_definitions.json.
        max_workers: Maximum concurrent git ls-remote calls.
        languages: If provided, only check these language names.

    Returns:
        A CheckReport summarising stale, up-to-date, and failed grammars.
    """
    subset = (
        {lang: language_definitions[lang] for lang in languages if lang in language_definitions}
        if languages
        else language_definitions
    )

    semaphore = asyncio.Semaphore(max_workers)
    tasks = [check_language(lang, defn, semaphore) for lang, defn in subset.items()]
    results = await asyncio.gather(*tasks)

    report = CheckReport()
    for result in results:
        if result is None:
            report.up_to_date += 1
        elif isinstance(result, str):
            report.failed.append(result)
        else:
            report.stale.append(result)

    return report


def apply_updates(
    language_definitions: dict[str, Any],
    stale: list[StaleGrammar],
    max_updates: int | None,
) -> tuple[dict[str, Any], list[StaleGrammar]]:
    """Apply updated revs to the language definitions dict.

    Args:
        language_definitions: Original definitions, will not be mutated.
        stale: List of stale grammars with their new revs.
        max_updates: If set, apply at most this many updates.

    Returns:
        A tuple of (updated definitions, list of actually applied updates).
    """
    updated = dict(language_definitions)
    to_apply = stale[:max_updates] if max_updates is not None else stale

    for entry in to_apply:
        if entry.language in updated:
            updated[entry.language] = dict(updated[entry.language])
            updated[entry.language]["rev"] = entry.latest_rev

    return updated, to_apply


async def main(args: argparse.Namespace) -> None:
    """Entry point."""
    max_workers = args.workers or min(MAX_WORKERS_DEFAULT, (os.cpu_count() or 4) * 4)

    language_definitions: dict[str, Any] = json.loads(DEFINITIONS_PATH.read_text())
    languages: list[str] | None = [lang.strip() for lang in args.languages.split(",")] if args.languages else None

    total = len(languages) if languages else len(language_definitions)
    print(f"Checking {total} grammar(s) for updates (concurrency={max_workers})...")

    report = await build_report(language_definitions, max_workers, languages)

    print(f"\nResults: {report.stale_count} stale, {report.up_to_date} up-to-date, {len(report.failed)} failed")

    report_dict = report.to_dict()

    # --dry-run: print what would change without writing anything
    if args.dry_run:
        if report.stale:
            limit = args.max_updates
            to_show = report.stale[:limit] if limit else report.stale
            print(f"\nWould update {len(to_show)} grammar(s):")
            for entry in sorted(to_show, key=lambda s: s.language):
                print(f"  {entry.language}: {entry.current_rev[:12]} -> {entry.latest_rev[:12]}")
            if limit and len(report.stale) > limit:
                skipped = len(report.stale) - limit
                print(f"  ... and {skipped} more (limited by --max-updates {limit})")
        else:
            print("\nAll grammars are up to date.")
        # Still emit the JSON report so callers can consume it
        print(json.dumps(report_dict, indent=2))
        return

    # --write: update language_definitions.json in place
    if args.write and report.stale:
        updated_definitions, applied = apply_updates(language_definitions, report.stale, args.max_updates)
        DEFINITIONS_PATH.write_text(json.dumps(updated_definitions, indent="\t") + "\n")
        print(f"\nUpdated {len(applied)} grammar revision(s) in {DEFINITIONS_PATH}")
        if args.max_updates and len(report.stale) > args.max_updates:
            remaining = len(report.stale) - args.max_updates
            print(f"  ({remaining} further update(s) skipped due to --max-updates {args.max_updates})")

    # --report: write JSON report to file
    if args.report:
        report_path = Path(args.report)
        report_path.write_text(json.dumps(report_dict, indent=2) + "\n")
        print(f"Report written to {report_path}")

    # Always emit JSON report to stdout so the workflow can consume it
    print(json.dumps(report_dict, indent=2))

    # Exit with non-zero code if there are stale grammars (useful for CI gating)
    if report.stale and not args.write and not args.dry_run:
        sys.exit(1)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Check tree-sitter grammar repositories for upstream updates.",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Check all grammars and print JSON report
  python scripts/check_grammar_updates.py

  # Check specific languages
  python scripts/check_grammar_updates.py --languages python,rust,go

  # Preview updates without writing
  python scripts/check_grammar_updates.py --dry-run --max-updates 10

  # Update up to 20 grammars and save report
  python scripts/check_grammar_updates.py --write --max-updates 20 --report grammar-updates.json
        """,
    )
    parser.add_argument(
        "--languages",
        type=str,
        help="Comma-separated list of languages to check (default: all)",
    )
    parser.add_argument(
        "--workers",
        type=int,
        help=f"Maximum concurrent git ls-remote calls (default: CPU count * 4, capped at {MAX_WORKERS_DEFAULT})",
    )
    parser.add_argument(
        "--write",
        action="store_true",
        help="Update language_definitions.json in place with new revisions",
    )
    parser.add_argument(
        "--max-updates",
        type=int,
        dest="max_updates",
        help="Limit the number of grammar revisions updated per run",
    )
    parser.add_argument(
        "--report",
        type=str,
        metavar="FILE",
        help="Write JSON report to FILE in addition to stdout",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        dest="dry_run",
        help="Show what would change without writing anything",
    )

    parsed = parser.parse_args()
    asyncio.run(main(parsed))
