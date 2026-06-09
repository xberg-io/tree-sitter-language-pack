//! Bundled tree-sitter highlight, injection, locals, and tags queries.
//!
//! Queries are embedded from `parsers/{lang}/queries/*.scm` at build time.
//! Not all languages have queries — returns `None` for languages without bundled queries.

include!(concat!(env!("OUT_DIR"), "/queries_generated.rs"));

/// Get the highlights query for a language, if bundled.
///
/// Returns the contents of `highlights.scm` as a static string, or `None`
/// if no highlights query is bundled for this language.
///
/// # Example
///
/// ```
/// use tree_sitter_language_pack::get_highlights_query;
///
/// // Returns Some(...) for languages with bundled queries
/// let query = get_highlights_query("python");
/// // Returns None for languages without bundled highlights queries
/// let missing = get_highlights_query("nonexistent_lang");
/// assert!(missing.is_none());
/// ```
pub fn get_highlights_query(language: &str) -> Option<&'static str> {
    get_highlights_query_impl(language)
}

/// Get the injections query for a language, if bundled.
///
/// Returns the contents of `injections.scm` as a static string, or `None`
/// if no injections query is bundled for this language.
///
/// # Example
///
/// ```
/// use tree_sitter_language_pack::get_injections_query;
///
/// let query = get_injections_query("markdown");
/// // Returns None for languages without bundled injections queries
/// let missing = get_injections_query("nonexistent_lang");
/// assert!(missing.is_none());
/// ```
pub fn get_injections_query(language: &str) -> Option<&'static str> {
    get_injections_query_impl(language)
}

/// Get the locals query for a language, if bundled.
///
/// Returns the contents of `locals.scm` as a static string, or `None`
/// if no locals query is bundled for this language.
///
/// # Example
///
/// ```
/// use tree_sitter_language_pack::get_locals_query;
///
/// let query = get_locals_query("python");
/// // Returns None for languages without bundled locals queries
/// let missing = get_locals_query("nonexistent_lang");
/// assert!(missing.is_none());
/// ```
pub fn get_locals_query(language: &str) -> Option<&'static str> {
    get_locals_query_impl(language)
}

/// Get the tags query for a language, if bundled.
///
/// Returns the contents of `tags.scm` as a static string, or `None`
/// if no tags query is bundled for this language.
///
/// # Example
///
/// ```
/// use tree_sitter_language_pack::get_tags_query;
///
/// let query = get_tags_query("rust");
/// // Returns None for languages without bundled tags queries
/// let missing = get_tags_query("nonexistent_lang");
/// assert!(missing.is_none());
/// ```
pub fn get_tags_query(language: &str) -> Option<&'static str> {
    get_tags_query_impl(language)
}
