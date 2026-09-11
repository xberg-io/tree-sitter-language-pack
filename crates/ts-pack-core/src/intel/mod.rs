//! Content intelligence and code chunking using tree-sitter.
//!
//! This module provides rich AST metadata extraction and intelligent code chunking.
//! It analyzes source code to extract structure, imports, exports, comments,
//! docstrings, symbols, and diagnostics.

pub(crate) mod c;
pub(crate) mod cedar;
pub mod chunking;
pub(crate) mod data_extraction;
pub(crate) mod dockerfile;
pub(crate) mod elixir;
pub(crate) mod extract;
pub mod intelligence;
pub(crate) mod just;
#[cfg(test)]
mod legacy;
pub(crate) mod sql;
#[cfg(test)]
mod test_support;
pub mod types;
pub(crate) mod walk;

pub use types::*;

use crate::process_config::ProcessConfig;

/// Process source code: parse once, extract intelligence based on config, and return it.
///
/// # Observability
///
/// Opens the `intel::process` span (target `ts_pack::intel`, DEBUG). The span
/// fields describe the resolved request; every stage reports its own counts as a
/// DEBUG event keyed by `operation`. Per-node work inside the walk is
/// deliberately uninstrumented: an event per node would cost more than the
/// single-pass traversal it would be describing.
#[tracing::instrument(
    name = "intel::process",
    target = "ts_pack::intel",
    level = "debug",
    skip_all,
    fields(
        language = %config.language,
        source_bytes = source.len(),
        want_structure = config.structure,
        want_imports = config.imports,
        want_exports = config.exports,
        want_comments = config.comments,
        want_docstrings = config.docstrings,
        want_symbols = config.symbols,
        want_diagnostics = config.diagnostics,
        want_chunking = config.chunk_max_size.is_some(),
        want_data_extraction = config.data_extraction
    )
)]
pub fn process(
    source: &str,
    config: &ProcessConfig,
    registry: &crate::LanguageRegistry,
) -> Result<ProcessResult, crate::Error> {
    let (lang, tree) = parse_source(source, config, registry).inspect_err(|error| {
        tracing::debug!(
            target: "ts_pack::intel",
            operation = "intel::parse",
            language = %config.language,
            error = %error,
            "parse failed; no intelligence extracted"
        );
    })?;
    let root = tree.root_node();
    tracing::debug!(
        target: "ts_pack::intel",
        operation = "intel::parse",
        language = %config.language,
        has_errors = root.has_error(),
        "parsed source"
    );

    let mut result = ProcessResult {
        language: config.language.as_ref().to_string(),
        metrics: intelligence::compute_line_metrics(source),
        ..Default::default()
    };

    // ~keep All enabled extractors share one depth-bounded walk; see `intel::extract`.
    let wanted = extract::Wanted {
        structure: config.structure,
        imports: config.imports,
        exports: config.exports,
        comments: config.comments,
        docstrings: config.docstrings,
        symbols: config.symbols,
        diagnostics: config.diagnostics,
    };
    extract::extract_all(&root, source, &config.language, wanted, &mut result);

    if let Some(max_size) = config.chunk_max_size {
        result.chunks = chunking::chunk_source(source, &config.language, max_size, &lang, &tree);
    }
    if config.data_extraction {
        result.data = data_extraction::extract_data(&root, source, &config.language);
    }

    Ok(result)
}

/// Parse source code and return the tree-sitter language and tree.
///
/// Uses the thread-local parser cache to avoid creating a new parser on
/// every call.
fn parse_source(
    source: &str,
    config: &ProcessConfig,
    registry: &crate::LanguageRegistry,
) -> Result<(tree_sitter::Language, tree_sitter::Tree), crate::Error> {
    let lang = registry.get_language(&config.language)?;
    let tree =
        crate::parse::parse_with_language_limited(&config.language, &lang, source.as_bytes(), config.parse_timeout_ms)?;
    Ok((lang, tree))
}

#[cfg(test)]
mod tests {
    // ~keep These tests report a skip on stderr rather than passing silently when no grammar loads.
    #![allow(clippy::print_stderr)]

    use crate::LanguageRegistry;
    use crate::process_config::ProcessConfig;

    use super::walk::MAX_TREE_DEPTH;

    /// Source samples chosen to exercise every extractor and every non-uniform
    /// walk shape: nested structure bodies, Ruby module/class/method nesting,
    /// Elixir `quote` pruning and compact `do:` bodies, JS exports, Python
    /// docstrings, and error recovery.
    const EQUIVALENCE_CORPUS: &[(&str, &str)] = &[
        (
            "python",
            "import os\nfrom sys import path\n\n\"\"\"Module doc.\"\"\"\n\nclass Widget:\n    \"\"\"Widget doc.\"\"\"\n\n    def render(self):\n        # inline\n        def inner():\n            pass\n        return inner\n\ndef top():\n    pass\n",
        ),
        ("python", "def :\n    pass\nclass ???:\n"),
        (
            "rust",
            "//! Crate doc\nuse std::collections::HashMap;\nuse std::io;\n\n/// Documented\npub struct Config { pub name: String }\n\ntrait Render { fn render(&self); }\n\nimpl Render for Config {\n    /// method doc\n    fn render(&self) {\n        let x = 5;\n        const Y: u8 = 1;\n    }\n}\n\nmod inner {\n    enum Kind { A, B }\n    fn helper() {}\n}\n",
        ),
        (
            "ruby",
            "module Outer\n  # a comment\n  class Widget\n    def call\n      true\n    end\n\n    def self.build\n      new\n    end\n  end\nend\n",
        ),
        (
            "elixir",
            "defmodule Calc do\n  import Enum\n  alias Foo.Bar\n\n  defstruct [do: (def fake, do: 1)]\n\n  defmacro gen do\n    quote do\n      import Should.Not.Appear\n      def generated, do: :ok\n    end\n  end\n\n  def add(a, b), do: a + b\n\n  defp helper(x) do\n    x * 2\n  end\n\n  defmodule Inner do\n    def f, do: 1\n  end\nend\n",
        ),
        ("elixir", "defmodule M, [do: (def f, do: 1)]\n"),
        (
            "javascript",
            "import fs from 'node:fs';\nexport default function main() {}\nexport { a, b };\nexport * from './other.js';\nclass Thing {\n  method() {\n    const arrow = () => 1;\n    return arrow;\n  }\n}\n",
        ),
        // ~keep A matched node descends into its `body` only: the arrow functions in the
        // ~keep parameter list and the heritage clause must not become nested items.
        (
            "javascript",
            "function outer(cb = () => 1, fallback = () => 2) {\n  const inner = () => 3;\n  return inner;\n}\nclass Sub extends mixin(() => 4) {\n  method(handler = () => 5) {}\n}\n",
        ),
        (
            "typescript",
            "import type { A } from './a';\nexport interface Shape { x: number }\ntype Alias = string;\nexport class Impl implements Shape {\n  x = 1;\n  method(): void {}\n}\n",
        ),
        (
            "go",
            "package main\n\nimport (\n\t\"fmt\"\n\t\"os\"\n)\n\ntype User struct{}\ntype Service interface{}\ntype ID string\n\nfunc main() {\n\tfmt.Println(os.Args)\n}\n",
        ),
        (
            "java",
            "package com.example;\n\nimport java.util.List;\n\n/** doc */\npublic class Widget {\n    enum Kind { A }\n    interface Inner {}\n    public void run() {}\n}\n",
        ),
        (
            "kotlin",
            "package foo.bar\n\nimport kotlin.math.abs\n\nclass Widget {\n    fun run() {}\n}\n",
        ),
    ];

    /// The grammar every non-language-specific test in this module uses.
    ///
    /// ~keep Naming it matters: picking `available_languages().first()` makes the
    /// ~keep test measure whichever grammar happens to sort first, and skip
    /// ~keep invisibly in a default build that links none.
    const DEFAULT_TEST_LANGUAGE: &str = "python";

    /// Whether `language` is linked into this build, reporting a skip if not.
    fn has_grammar(registry: &LanguageRegistry, language: &str, test_name: &str) -> bool {
        match registry.get_language(language) {
            Ok(_) => true,
            Err(error) => {
                eprintln!("SKIPPED {test_name}: grammar '{language}' is not available: {error}");
                false
            }
        }
    }

    fn parse(source: &str, language: &str, registry: &LanguageRegistry) -> tree_sitter::Tree {
        let lang = registry
            .get_language(language)
            .unwrap_or_else(|e| panic!("grammar '{language}' must be loadable: {e}"));
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&lang).expect("language must be settable");
        parser.parse(source, None).expect("source must parse")
    }

    /// The single bounded traversal must produce exactly what the eight
    /// independent recursive walks produced, for every extractor.
    #[test]
    fn should_match_the_reference_walks_on_normal_input() {
        let registry = LanguageRegistry::new();
        if registry.available_languages().is_empty() {
            eprintln!("SKIPPED should_match_the_reference_walks_on_normal_input: no grammars available");
            return;
        }

        let mut checked = 0usize;
        for (language, source) in EQUIVALENCE_CORPUS {
            // ~keep Skip per entry, not all-or-nothing. Gating only on "no grammars at all" made a
            // ~keep partial build (TSLP_LANGUAGES=python,rust,c) panic on the first corpus language
            // ~keep it did not link, so the suite was runnable at zero grammars or all twelve and
            // ~keep nothing in between.
            if !has_grammar(&registry, language, "should_match_the_reference_walks_on_normal_input") {
                continue;
            }
            let tree = parse(source, language, &registry);
            let actual = super::intelligence::extract_intelligence(source, language, &tree);
            let expected = super::legacy::extract_intelligence(source, language, &tree);

            assert_eq!(actual.metrics, expected.metrics, "metrics differ for {language}");
            assert_eq!(actual.structure, expected.structure, "structure differs for {language}");
            assert_eq!(actual.imports, expected.imports, "imports differ for {language}");
            assert_eq!(actual.exports, expected.exports, "exports differ for {language}");
            assert_eq!(actual.comments, expected.comments, "comments differ for {language}");
            assert_eq!(
                actual.docstrings, expected.docstrings,
                "docstrings differ for {language}"
            );
            assert_eq!(actual.symbols, expected.symbols, "symbols differ for {language}");
            assert_eq!(
                actual.diagnostics, expected.diagnostics,
                "diagnostics differ for {language}"
            );
            checked += 1;
        }

        assert!(
            checked > 0,
            "no corpus entry was compared — the equivalence oracle proved nothing"
        );
    }

    /// Chunk metadata comes from the same bounded walk and must also be unchanged.
    #[test]
    fn should_match_the_reference_chunk_metadata_walk() {
        let registry = LanguageRegistry::new();
        if registry.get_language("python").is_err() {
            eprintln!("SKIPPED should_match_the_reference_chunk_metadata_walk: no python grammar");
            return;
        }
        let source =
            "# lead\ndef alpha():\n    \"\"\"doc\"\"\"\n    pass\n\nclass Beta:\n    def gamma(self):\n        pass\n";
        let config = ProcessConfig::new("python").all().with_chunking(30);
        let result = super::process(source, &config, &registry).expect("process must succeed");

        assert!(result.chunks.len() >= 2, "the sample must actually split into chunks");
        let tree = parse(source, "python", &registry);
        let root = tree.root_node();

        for chunk in &result.chunks {
            let mut node_types = Vec::new();
            let mut symbols = Vec::new();
            let mut comments = Vec::new();
            let mut docstrings = Vec::new();
            let mut has_errors = false;
            let mut context_path = Vec::new();
            let mut collector = super::chunking::MetadataCollector {
                node_types: &mut node_types,
                symbols: &mut symbols,
                comments: &mut comments,
                docstrings: &mut docstrings,
                has_errors: &mut has_errors,
                context_path: &mut context_path,
            };
            super::legacy::collect_chunk_metadata(
                &root,
                source,
                "python",
                chunk.start_byte,
                chunk.end_byte,
                &mut collector,
                0,
            );

            assert_eq!(chunk.metadata.node_types, node_types, "node_types differ");
            assert_eq!(chunk.metadata.symbols_defined, symbols, "symbols differ");
            assert_eq!(chunk.metadata.comments, comments, "comments differ");
            assert_eq!(chunk.metadata.docstrings, docstrings, "docstrings differ");
            assert_eq!(chunk.metadata.has_error_nodes, has_errors, "error flag differs");
            assert_eq!(chunk.metadata.context_path, context_path, "context_path differs");
        }
    }

    /// A 50,000-level nesting used to abort the process with a stack overflow
    /// that `catch_unwind` could not contain. It must now return a truncated
    /// result (WARN-level, degraded but continuing).
    ///
    /// The brackets must be balanced: an unclosed run of `[` parses to a single
    /// flat ERROR node in the Python grammar (measured `max_depth` 1), so it
    /// never reproduced the overflow in the first place.
    #[test]
    fn should_truncate_instead_of_aborting_on_deeply_nested_input() {
        let registry = LanguageRegistry::new();
        if registry.get_language("python").is_err() {
            eprintln!("SKIPPED should_truncate_instead_of_aborting_on_deeply_nested_input: no python grammar");
            return;
        }
        const NESTING: usize = 50_000;
        let source = format!("{}1{}", "[".repeat(NESTING), "]".repeat(NESTING));
        // ~keep Chunking is on deliberately: `text_splitter::collect_node_ranges` is now depth-
        // ~keep bounded too, so this input must survive the chunking path as well as the walk.
        let config = ProcessConfig::new("python").all().with_chunking(4096);

        let result = super::process(&source, &config, &registry).expect("deep input must not error");

        assert_eq!(
            result.metrics.max_depth, MAX_TREE_DEPTH,
            "the walk must stop at the depth limit rather than following the tree"
        );
        // ~keep The full tree has at least one node per nesting level; a bounded walk sees far fewer.
        assert!(
            result.metrics.node_count < NESTING,
            "a truncated walk must not have visited the whole tree; visited {} of >{NESTING} nodes",
            result.metrics.node_count
        );
        assert_eq!(
            result.metrics.total_bytes,
            source.len(),
            "metrics still describe the file"
        );
    }

    /// Data extraction keeps its own per-format recursion, so it needs its own
    /// deep-input regression.
    #[test]
    fn should_truncate_deeply_nested_data_instead_of_aborting() {
        let registry = LanguageRegistry::new();
        if registry.get_language("json").is_err() {
            eprintln!("SKIPPED should_truncate_deeply_nested_data_instead_of_aborting: no json grammar");
            return;
        }
        let depth = 20_000;
        let source = format!("{}1{}", "[".repeat(depth), "]".repeat(depth));
        let config = ProcessConfig::new("json").all().with_data_extraction(true);

        let result = super::process(&source, &config, &registry).expect("deep JSON must not error");

        let mut node = result.data.as_ref().expect("json must yield a data tree");
        let mut levels = 1usize;
        while let Some(child) = node.children.first() {
            node = child;
            levels += 1;
        }
        assert!(
            levels <= MAX_TREE_DEPTH + 1,
            "data tree must be truncated at the depth limit; got {levels} levels"
        );
        assert!(
            levels > MAX_TREE_DEPTH / 2,
            "the input must actually have reached the limit, not stopped early; got {levels} levels"
        );
    }

    #[test]
    fn test_process_returns_intelligence() {
        let registry = LanguageRegistry::new();
        let lang = DEFAULT_TEST_LANGUAGE;
        if !has_grammar(&registry, lang, "test_process_returns_intelligence") {
            return;
        }
        let source = "x";
        let config = ProcessConfig::new(lang).all();
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
        let lang = DEFAULT_TEST_LANGUAGE;
        if !has_grammar(&registry, lang, "test_process_with_chunking") {
            return;
        }
        let source = "x";
        let config = ProcessConfig::new(lang).all().with_chunking(1000);
        let result = super::process(source, &config, &registry);
        assert!(result.is_ok());
        let intel = result.unwrap();
        assert_eq!(intel.language, lang);
        assert!(!intel.chunks.is_empty(), "should have at least one chunk");
        assert_eq!(intel.chunks[0].metadata.language, lang);
    }

    /// A leading UTF-8 BOM is never stripped before parsing (`parse_source` calls
    /// `source.as_bytes()` on the exact `&str` the caller passed in), so every span
    /// this crate returns must already index correctly into that same original
    /// buffer, BOM included. This is the property that would silently break if a
    /// future change stripped the BOM at the parse boundary without adding its
    /// byte length back into every reported offset.
    #[test]
    fn should_index_symbol_spans_into_the_original_bom_prefixed_source() {
        let registry = LanguageRegistry::new();
        if !has_grammar(
            &registry,
            "rust",
            "should_index_symbol_spans_into_the_original_bom_prefixed_source",
        ) {
            return;
        }
        let source = "\u{FEFF}fn alpha() {}\n";
        let config = ProcessConfig::new("rust").all();
        let result = super::process(source, &config, &registry).expect("BOM-prefixed source must still parse");

        assert_eq!(
            result.metrics.error_count, 0,
            "a leading BOM must not be reported as a parse error"
        );

        let symbol = result
            .symbols
            .iter()
            .find(|s| s.name == "alpha")
            .expect("the alpha function must be extracted as a symbol");
        assert_eq!(
            &source[symbol.span.start_byte..symbol.span.end_byte],
            "fn alpha() {}",
            "span byte offsets must slice the exact source the caller passed in, BOM included"
        );
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
        let lang = DEFAULT_TEST_LANGUAGE;
        if !has_grammar(&registry, lang, "test_process_empty_source") {
            return;
        }
        let config = ProcessConfig::new(lang);
        let result = super::process("", &config, &registry);
        assert!(result.is_ok(), "empty source should parse without error");
        let intel = result.unwrap();
        assert_eq!(intel.metrics.total_bytes, 0);
    }

    #[test]
    fn test_process_with_small_chunk_size() {
        let registry = LanguageRegistry::new();
        if !has_grammar(&registry, "python", "test_process_with_small_chunk_size") {
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
