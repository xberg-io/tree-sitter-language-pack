//! Content intelligence and code chunking using tree-sitter.
//!
//! This module provides rich AST metadata extraction and intelligent code chunking.
//! It analyzes source code to extract structure, imports, exports, comments,
//! docstrings, symbols, and diagnostics.

pub mod chunking;
pub mod intelligence;
pub mod types;

pub use types::*;

use crate::process_config::ProcessConfig;

/// Process source code: parse once, extract intelligence based on config, and return it.
pub fn process(
    source: &str,
    config: &ProcessConfig,
    registry: &crate::LanguageRegistry,
) -> Result<ProcessResult, crate::Error> {
    let (lang, tree) = parse_source(source, &config.language, registry)?;
    let root = tree.root_node();

    let mut result = ProcessResult {
        language: config.language.as_ref().to_string(),
        metrics: intelligence::compute_metrics(source, &root),
        ..Default::default()
    };

    if config.structure {
        result.structure = intelligence::extract_structure(&root, source, &config.language);
    }
    if config.imports {
        result.imports = intelligence::extract_imports(&root, source, &config.language);
    }
    if config.exports {
        result.exports = intelligence::extract_exports(&root, source, &config.language);
    }
    if config.comments {
        result.comments = intelligence::extract_comments(&root, source, &config.language);
    }
    if config.docstrings {
        result.docstrings = intelligence::extract_docstrings(&root, source, &config.language);
    }
    if config.symbols {
        result.symbols = intelligence::extract_symbols(&root, source, &config.language);
    }
    if config.diagnostics {
        result.diagnostics = intelligence::extract_diagnostics(&root, source);
    }
    if let Some(max_size) = config.chunk_max_size {
        result.chunks = chunking::chunk_source(source, &config.language, max_size, &lang, &tree);
    }

    Ok(result)
}

/// Parse source code and return the tree-sitter language and tree.
///
/// Uses the thread-local parser cache to avoid creating a new parser on
/// every call.
fn parse_source(
    source: &str,
    language: &str,
    registry: &crate::LanguageRegistry,
) -> Result<(tree_sitter::Language, tree_sitter::Tree), crate::Error> {
    let lang = registry.get_language(language)?;
    let tree = crate::parse::parse_with_language(language, &lang, source.as_bytes())?;
    Ok((lang, tree))
}

#[cfg(test)]
mod tests {
    use crate::LanguageRegistry;
    use crate::process_config::ProcessConfig;

    fn first_lang(registry: &LanguageRegistry) -> Option<String> {
        let langs = registry.available_languages();
        langs.into_iter().next()
    }

    #[test]
    fn test_process_returns_intelligence() {
        let registry = LanguageRegistry::new();
        let Some(lang) = first_lang(&registry) else { return };
        let source = "x";
        let config = ProcessConfig::new(&lang).all();
        let result = super::process(source, &config, &registry);
        assert!(result.is_ok(), "process should succeed for available language");
        let intel = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(intel.metrics.total_lines >= 1);
        assert!(intel.metrics.node_count > 0);
    }

    #[test]
    fn test_process_with_chunking() {
        let registry = LanguageRegistry::new();
        let Some(lang) = first_lang(&registry) else { return };
        let source = "x";
        let config = ProcessConfig::new(&lang).all().with_chunking(1000);
        let result = super::process(source, &config, &registry);
        assert!(result.is_ok());
        let intel = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(!intel.chunks.is_empty(), "should have at least one chunk");
        assert_eq!(intel.chunks[0].metadata.language, lang);
    }

    #[test]
    fn test_process_invalid_language() {
        let registry = LanguageRegistry::new();
        let config = ProcessConfig::new("nonexistent_lang_xyz");
        let result = super::process("x", &config, &registry);
        assert!(result.is_err(), "should fail for nonexistent language");
    }

    #[test]
    fn test_process_empty_source() {
        let registry = LanguageRegistry::new();
        let Some(lang) = first_lang(&registry) else { return };
        let config = ProcessConfig::new(&lang);
        let result = super::process("", &config, &registry);
        assert!(result.is_ok(), "empty source should parse without error");
        let intel = result.unwrap();
        assert_eq!(intel.metrics.total_bytes, 0);
    }

    #[test]
    fn test_process_with_small_chunk_size() {
        let registry = LanguageRegistry::new();
        if !registry.has_language("python") {
            return;
        }
        let source = "def foo():\n    pass\ndef bar():\n    pass\n";
        let config = ProcessConfig::new("python").all().with_chunking(20);
        let result = super::process(source, &config, &registry);
        assert!(result.is_ok());
        let intel = result.unwrap();
        assert!(
            intel.chunks.len() >= 2,
            "small max_chunk_size should split into multiple chunks"
        );
        assert_eq!(intel.language, "python");
    }
}
