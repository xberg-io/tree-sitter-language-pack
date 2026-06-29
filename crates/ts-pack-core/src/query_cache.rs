//! Process-wide cache of compiled tree-sitter queries.
//!
//! Compiling a [`tree_sitter::Query`] from a bundled `.scm` source is relatively
//! expensive, and the result is immutable and `Send + Sync`, so it is cached once
//! per `(language, kind)` and shared via an `Arc` for the lifetime of the process.
//!
//! The cache is Rust-only: `Arc<Query>` is an opaque handle that cannot cross the C
//! FFI boundary, so [`get_query`] and [`QueryKind`] are excluded from the generated
//! bindings (`alef(skip)`). Non-Rust callers use the raw query-string accessors
//! ([`crate::get_tags_query`] et al.) and compile with their own tree-sitter runtime.

use std::sync::{Arc, LazyLock, RwLock};

use ahash::AHashMap;
use tree_sitter::Query;

use crate::error::Error;

/// Which bundled `.scm` query to compile and cache for a language.
#[cfg_attr(alef, alef(skip))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryKind {
    /// Syntax-highlighting queries (`highlights.scm`).
    Highlights,
    /// Language-injection queries (`injections.scm`).
    Injections,
    /// Local-scope / variable-resolution queries (`locals.scm`).
    Locals,
    /// Symbol/definition tag queries (`tags.scm`).
    Tags,
    /// Auto-indentation queries (`indents.scm`).
    Indents,
    /// Code-folding queries (`folds.scm`).
    Folds,
}

#[allow(clippy::type_complexity)]
static QUERY_CACHE: LazyLock<RwLock<AHashMap<(String, QueryKind), Option<Arc<Query>>>>> =
    LazyLock::new(|| RwLock::new(AHashMap::new()));

/// The raw `.scm` source string for a language/kind, or `None` if not bundled.
fn source_for(language: &str, kind: QueryKind) -> Option<&'static str> {
    match kind {
        QueryKind::Highlights => crate::queries::get_highlights_query(language),
        QueryKind::Injections => crate::queries::get_injections_query(language),
        QueryKind::Locals => crate::queries::get_locals_query(language),
        QueryKind::Tags => crate::queries::get_tags_query(language),
        QueryKind::Indents => crate::queries::get_indents_query(language),
        QueryKind::Folds => crate::queries::get_folds_query(language),
    }
}

/// Get a compiled, process-cached tree-sitter query for `language` / `kind`.
///
/// Returns `Ok(None)` when no `.scm` source is bundled for that language and kind —
/// the negative result is cached, so a missing overlay is checked exactly once. The
/// compiled [`Query`] is shared via an `Arc` and lives for the process lifetime.
///
/// Language aliases are resolved (e.g. `"shell"` → `"bash"`), consistent with
/// [`crate::get_language`].
///
/// # Errors
///
/// Returns [`Error::QueryError`] if the bundled `.scm` source fails to compile, or
/// [`Error::LanguageNotFound`] if the language cannot be loaded.
///
/// # Example
///
/// ```
/// use tree_sitter_language_pack::{get_query, QueryKind};
///
/// // No query is bundled for an unknown language → Ok(None).
/// assert!(get_query("nonexistent_lang", QueryKind::Tags)?.is_none());
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
#[cfg_attr(alef, alef(skip))]
pub fn get_query(language: &str, kind: QueryKind) -> Result<Option<Arc<Query>>, Error> {
    let lang = crate::registry::resolve_alias(language);

    // Read fast path.
    {
        let cache = QUERY_CACHE.read().map_err(|e| Error::LockPoisoned(e.to_string()))?;
        if let Some(entry) = cache.get(&(lang.to_string(), kind)) {
            return Ok(entry.clone());
        }
    }

    // Miss: compile outside the write lock to keep the critical section small.
    let compiled: Option<Arc<Query>> = match source_for(lang, kind) {
        None => None, // negative cache
        Some(src) => {
            let language = crate::get_language(lang)?;
            let query = Query::new(&language, src)
                .map_err(|e| Error::QueryError(format!("failed to compile {kind:?} query for '{lang}': {e}")))?;
            Some(Arc::new(query))
        }
    };

    // Write with double-check: a concurrent compile wins and we drop ours.
    let mut cache = QUERY_CACHE.write().map_err(|e| Error::LockPoisoned(e.to_string()))?;
    let entry = cache.entry((lang.to_string(), kind)).or_insert(compiled);
    Ok(entry.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_KINDS: [QueryKind; 6] = [
        QueryKind::Highlights,
        QueryKind::Injections,
        QueryKind::Locals,
        QueryKind::Tags,
        QueryKind::Indents,
        QueryKind::Folds,
    ];

    #[test]
    fn missing_query_returns_none_and_is_negatively_cached() {
        // Unknown language → no bundled source → Ok(None). The None is cached, so a
        // second call returns None via the read fast path. Content-independent: never
        // calls `get_language` (the `None` source short-circuits before any load).
        for _ in 0..2 {
            for kind in ALL_KINDS {
                assert!(get_query("definitely_not_a_language", kind).unwrap().is_none());
            }
        }
    }
}
