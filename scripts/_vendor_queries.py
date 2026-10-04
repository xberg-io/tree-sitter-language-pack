"""Query-file discovery and scoring for vendored grammar repositories.

Shared by ``clone_vendors.py``; imported as a sibling module because the script is always run by path.
"""

from __future__ import annotations

from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from pathlib import Path

QUERY_TYPES = ["highlights.scm", "injections.scm", "locals.scm", "indents.scm", "folds.scm", "tags.scm"]


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


def _collect_query_candidates(vendor_repo: Path) -> dict[str, list[Path]]:
    """Group every eligible ``*.scm`` file in the vendor repo by standard query type."""
    all_scm_files: dict[str, list[Path]] = {qtype: [] for qtype in QUERY_TYPES}

    for scm_file in vendor_repo.rglob("*.scm"):
        if not _is_query_like_path(scm_file):
            continue

        if _is_skip_path(scm_file):
            continue

        filename = scm_file.name
        if filename in QUERY_TYPES:
            all_scm_files[filename].append(scm_file)
    return all_scm_files


def _scoped_query_subdirs(all_scm_files: dict[str, list[Path]], vendor_repo: Path) -> frozenset[str]:
    """Collect every `<x>` seen in a `queries/<x>/<file>` candidate, ignoring editor directories."""
    return frozenset(
        candidate.relative_to(vendor_repo).parts[-2]
        for candidates in all_scm_files.values()
        for candidate in candidates
        if len(candidate.relative_to(vendor_repo).parts) >= 3
        and candidate.relative_to(vendor_repo).parts[-3] == "queries"
        and not _get_editor_score(candidate.relative_to(vendor_repo).parts[-2])
    )


def _has_own_queries_dir(all_scm_files: dict[str, list[Path]], vendor_repo: Path, directory: str | None) -> bool:
    """Report whether upstream gives this grammar its own queries directory."""
    return bool(directory) and any(
        _score_query_candidate(candidate.relative_to(vendor_repo), directory) == 1
        for candidates in all_scm_files.values()
        for candidate in candidates
    )


def _select_best_query(
    candidates: list[Path],
    query_type: str,
    *,
    language_name: str,
    directory: str | None,
    vendor_repo: Path,
    scoped_subdirs: frozenset[str],
    has_own_queries_dir: bool,
) -> Path | None:
    """Pick the highest-priority candidate for one query type, or None when none may be used."""
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
        return None

    # ~keep When upstream gives this grammar its own queries directory, that directory is
    # ~keep authoritative: a kind missing from it is missing deliberately, and the repo-root
    # ~keep set belongs to the sibling grammar. Falling back to root regardless is how
    # ~keep fsharp_signature ended up with fsharp's locals/indents, which name node types
    # ~keep the signature grammar does not have and so fail to compile.
    if has_own_queries_dir and all(_score_query_candidate(rel, directory) != 1 for _, rel in eligible):
        print(f"Skipping {language_name} {query_type}: not provided by {directory}/queries")
        return None

    scored = [(candidate, _score_query_candidate(rel, directory)) for candidate, rel in eligible]
    scored.sort(key=lambda x: x[1])
    return scored[0][0]
