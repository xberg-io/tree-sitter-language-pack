use std::cell::RefCell;
use std::sync::Mutex;

use ahash::AHashMap;

use crate::Error;

thread_local! {
    static PARSER_CACHE: RefCell<AHashMap<String, tree_sitter::Parser>> = RefCell::new(AHashMap::new());
}

static PARSE_LOCK: Mutex<()> = Mutex::new(());

/// Parse source code with a pre-loaded `Language`, using the thread-local parser cache.
///
/// Avoids a redundant registry lookup when the caller already has the `Language`
/// (e.g., from `LanguageRegistry::get_language`).
pub(crate) fn parse_with_language(
    language_name: &str,
    language: &tree_sitter::Language,
    source: &[u8],
) -> Result<tree_sitter::Tree, Error> {
    // Some third-party grammar scanner code keeps process-global mutable state.
    // The public API remains safe for concurrent callers by serializing entry
    // into parser execution while retaining thread-local parser instances.
    let _guard = PARSE_LOCK.lock().map_err(|e| Error::LockPoisoned(e.to_string()))?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn skip_if_no_languages() -> bool {
        crate::available_languages().is_empty()
    }

    fn parse_for_test(language_name: &str, source: &[u8]) -> Result<tree_sitter::Tree, Error> {
        let language = crate::get_language(language_name)?;
        parse_with_language(language_name, &language, source)
    }

    #[test]
    fn test_parse_with_language_success() {
        if skip_if_no_languages() {
            return;
        }
        let langs = crate::available_languages();
        let first = &langs[0];
        let tree = parse_for_test(first, b"x");
        assert!(tree.is_ok(), "parse_with_language should succeed for '{first}'");
    }

    #[test]
    fn test_get_language_invalid_language() {
        let result = crate::get_language("nonexistent_xyz");
        assert!(result.is_err());
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
    fn test_different_languages_get_separate_cache_entries() {
        let langs = crate::available_languages();
        if langs.len() < 2 {
            return;
        }
        let before = cached_parser_count_for_tests();
        let _ = parse_for_test(&langs[0], b"x").unwrap();
        let _ = parse_for_test(&langs[1], b"x").unwrap();
        let after = cached_parser_count_for_tests();
        assert!(
            after >= before + 2,
            "different languages should create separate cache entries"
        );
    }
}
