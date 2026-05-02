use std::cell::RefCell;

use ahash::AHashMap;

use crate::Error;

thread_local! {
    static PARSER_CACHE: RefCell<AHashMap<String, tree_sitter::Parser>> = RefCell::new(AHashMap::new());
}

/// Parse source code with the named language, returning the syntax tree.
///
/// Uses the global registry to look up the language by name.
/// Caches parsers per-thread so repeated calls for the same language avoid
/// re-creating the parser.
///
/// # Examples
///
/// ```no_run
/// let tree = tree_sitter_language_pack::parse_string("python", b"def hello(): pass").unwrap();
/// assert_eq!(tree.root_node().kind(), "module");
/// ```
pub fn parse_string(language: &str, source: &[u8]) -> Result<tree_sitter::Tree, Error> {
    PARSER_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(parser) = cache.get_mut(language) {
            return parser.parse(source, None).ok_or(Error::ParseFailed);
        }
        let language_obj = crate::get_language(language)?;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&language_obj)
            .map_err(|e| Error::ParserSetup(format!("{e}")))?;
        let tree = parser.parse(source, None).ok_or(Error::ParseFailed)?;
        cache.insert(language.to_string(), parser);
        Ok(tree)
    })
}

/// Parse source code with a pre-loaded `Language`, using the thread-local parser cache.
///
/// Avoids a redundant registry lookup when the caller already has the `Language`
/// (e.g., from `CompiledExtraction` or `LanguageRegistry::get_language`).
pub(crate) fn parse_with_language(
    language_name: &str,
    language: &tree_sitter::Language,
    source: &[u8],
) -> Result<tree_sitter::Tree, Error> {
    PARSER_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if let Some(parser) = cache.get_mut(language_name) {
            return parser.parse(source, None).ok_or(Error::ParseFailed);
        }
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(language)
            .map_err(|e| Error::ParserSetup(format!("{e}")))?;
        let tree = parser.parse(source, None).ok_or(Error::ParseFailed)?;
        cache.insert(language_name.to_string(), parser);
        Ok(tree)
    })
}

#[cfg(test)]
pub(crate) fn cached_parser_count_for_tests() -> usize {
    PARSER_CACHE.with(|cache| cache.borrow().len())
}

/// Check whether any node in the tree matches the given type name.
///
/// Performs a depth-first traversal using `TreeCursor`.
#[cfg(test)]
pub(crate) fn tree_contains_node_type(tree: &tree_sitter::Tree, node_type: &str) -> bool {
    let mut cursor = tree.walk();
    traverse_with_cursor(&mut cursor, |node| node.kind() == node_type)
}

/// Check whether the tree contains any ERROR or MISSING nodes.
///
/// Useful for determining if the parse was clean or had syntax errors.
#[cfg(test)]
pub(crate) fn tree_has_error_nodes(tree: &tree_sitter::Tree) -> bool {
    let mut cursor = tree.walk();
    traverse_with_cursor(&mut cursor, |node| node.is_error() || node.is_missing())
}

/// Return the S-expression representation of the entire tree.
///
/// This is the standard tree-sitter debug format, useful for logging,
/// snapshot testing, and debugging grammars.
#[cfg(test)]
pub(crate) fn tree_to_sexp(tree: &tree_sitter::Tree) -> String {
    tree.root_node().to_sexp()
}

/// Count the number of ERROR and MISSING nodes in the tree.
///
/// Returns 0 for a clean parse.
#[cfg(test)]
pub(crate) fn tree_error_count(tree: &tree_sitter::Tree) -> usize {
    let mut count = 0;
    let mut cursor = tree.walk();
    traverse_with_cursor(&mut cursor, |node| {
        if node.is_error() || node.is_missing() {
            count += 1;
        }
        false // never short-circuit, visit all nodes
    });
    count
}

/// Depth-first traversal with a cursor, calling `predicate` on each node.
///
/// Returns `true` as soon as the predicate returns `true` (short-circuit).
/// Returns `false` if no node matches.
#[cfg(test)]
pub(crate) fn traverse_with_cursor(
    cursor: &mut tree_sitter::TreeCursor,
    mut predicate: impl FnMut(tree_sitter::Node) -> bool,
) -> bool {
    loop {
        if predicate(cursor.node()) {
            return true;
        }
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn skip_if_no_languages() -> bool {
        crate::available_languages().is_empty()
    }

    #[test]
    fn test_parse_string_success() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let tree = parse_string(first, b"x");
        assert!(tree.is_ok(), "parse_string should succeed for '{first}'");
    }

    #[test]
    fn test_parse_string_reuses_thread_local_parser_cache() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let before = cached_parser_count_for_tests();
        let _ = parse_string(first, b"x").unwrap();
        let after_first = cached_parser_count_for_tests();
        let _ = parse_string(first, b"y").unwrap();
        let after_second = cached_parser_count_for_tests();
        assert!(after_first >= before);
        assert_eq!(after_second, after_first);
    }

    #[test]
    fn test_parse_string_invalid_language() {
        let result = parse_string("nonexistent_xyz", b"x");
        assert!(result.is_err());
    }

    #[test]
    fn test_tree_to_sexp() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let tree = parse_string(&langs[0], b"x").unwrap();
        let sexp = tree_to_sexp(&tree);
        assert!(!sexp.is_empty());
    }

    #[test]
    fn test_tree_contains_node_type() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let tree = parse_string(&langs[0], b"x").unwrap();
        let root_kind = tree.root_node().kind().to_string();
        assert!(tree_contains_node_type(&tree, &root_kind));
        assert!(!tree_contains_node_type(&tree, "nonexistent_node_type_xyz"));
    }

    #[test]
    fn test_tree_has_error_nodes_clean() {
        if skip_if_no_languages() {
            return;
        }
        // Most parsers handle single-token inputs without error
        let langs = crate::available_languages();
        let tree = parse_string(&langs[0], b"x").unwrap();
        // Just verify it runs without panic; result depends on grammar
        let _ = tree_has_error_nodes(&tree);
    }

    #[test]
    fn test_tree_error_count() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let tree = parse_string(&langs[0], b"x").unwrap();
        let count = tree_error_count(&tree);
        // Just verify it returns a reasonable number
        assert!(count < 1000);
    }

    #[test]
    fn test_parse_with_language_reuses_cache() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let lang = crate::get_language(first).unwrap();
        let _ = parse_with_language(first, &lang, b"x").unwrap();
        let after_first = cached_parser_count_for_tests();
        let _ = parse_with_language(first, &lang, b"y").unwrap();
        let after_second = cached_parser_count_for_tests();
        assert_eq!(after_first, after_second, "second call should reuse cached parser");
    }

    #[test]
    fn test_parse_with_language_and_parse_string_share_cache() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let lang = crate::get_language(first).unwrap();
        let _ = parse_string(first, b"x").unwrap();
        let after_parse_string = cached_parser_count_for_tests();
        let _ = parse_with_language(first, &lang, b"y").unwrap();
        let after_parse_with_lang = cached_parser_count_for_tests();
        assert_eq!(
            after_parse_string, after_parse_with_lang,
            "parse_string and parse_with_language should share the same cache"
        );
    }

    #[test]
    fn test_different_languages_get_separate_cache_entries() {
        let langs = crate::available_languages();
        if langs.len() < 2 {
            return;
        }
        let before = cached_parser_count_for_tests();
        let _ = parse_string(&langs[0], b"x").unwrap();
        let _ = parse_string(&langs[1], b"x").unwrap();
        let after = cached_parser_count_for_tests();
        assert!(
            after >= before + 2,
            "different languages should create separate cache entries"
        );
    }
}
