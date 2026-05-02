//! Pattern-based extraction from parsed syntax trees.
//!
//! Provides a high-level API for running tree-sitter queries against source code
//! and extracting structured results including node info, text, and child fields.

use ahash::AHashMap;

use crate::Error;
use crate::node::{NodeInfo, node_info_from_node};

/// Controls what data is captured for each query match.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) enum CaptureOutput {
    /// Capture only the matched text.
    Text,
    /// Capture only the `NodeInfo`.
    Node,
    /// Capture both text and `NodeInfo` (default).
    #[default]
    Full,
}

/// Defines a single extraction pattern and its configuration.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct ExtractionPattern {
    /// The tree-sitter query string (S-expression).
    pub query: String,
    /// What to include in each capture result.
    #[cfg_attr(feature = "serde", serde(default))]
    pub capture_output: CaptureOutput,
    /// Field names to extract from child nodes of each capture.
    /// Maps a label to a tree-sitter field name used with `child_by_field_name`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub child_fields: Vec<String>,
    /// Maximum number of matches to return. `None` means unlimited.
    #[cfg_attr(feature = "serde", serde(default))]
    pub max_results: Option<usize>,
    /// Restrict matches to a byte range `(start, end)`.
    #[cfg_attr(feature = "serde", serde(default))]
    pub byte_range: Option<(usize, usize)>,
}

/// Configuration for an extraction run against a single language.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct ExtractionConfig {
    /// The language name (e.g., `"python"`).
    pub language: String,
    /// Named patterns to run. Keys become the keys in `ExtractionResult::results`.
    pub patterns: AHashMap<String, ExtractionPattern>,
}

/// A single captured node within a match.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct CaptureResult {
    /// The capture name from the query (e.g., `"fn_name"`).
    pub name: String,
    /// The `NodeInfo` snapshot, present when `CaptureOutput` is `Node` or `Full`.
    pub node: Option<NodeInfo>,
    /// The matched source text, present when `CaptureOutput` is `Text` or `Full`.
    pub text: Option<String>,
    /// Values of requested child fields, keyed by field name.
    pub child_fields: AHashMap<String, Option<String>>,
    /// Byte offset where this capture starts in the source.
    pub start_byte: usize,
}

/// A single query match containing one or more captures.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct MatchResult {
    /// The pattern index within the query that produced this match.
    pub pattern_index: usize,
    /// The captures for this match.
    pub captures: Vec<CaptureResult>,
}

/// Results for a single named pattern.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct PatternResult {
    /// The individual matches.
    pub matches: Vec<MatchResult>,
    /// Total number of matches before `max_results` truncation.
    pub total_count: usize,
}

/// Complete extraction results for all patterns.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct ExtractionResult {
    /// The language that was used.
    pub language: String,
    /// Results keyed by pattern name.
    pub results: AHashMap<String, PatternResult>,
}

/// Validation information for a single pattern.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct PatternValidation {
    /// Whether the pattern compiled successfully.
    pub valid: bool,
    /// Names of captures defined in the query.
    pub capture_names: Vec<String>,
    /// Number of patterns in the query.
    pub pattern_count: usize,
    /// Non-fatal warnings (e.g., unused captures).
    pub warnings: Vec<String>,
    /// Fatal errors (e.g., query syntax errors).
    pub errors: Vec<String>,
}

/// Validation results for an entire extraction config.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(crate) struct ValidationResult {
    /// Whether all patterns are valid.
    pub valid: bool,
    /// Per-pattern validation details.
    pub patterns: AHashMap<String, PatternValidation>,
}

/// A compiled query with pre-computed capture names.
struct CompiledPattern {
    name: String,
    query: tree_sitter::Query,
    capture_names: Vec<String>,
    config: ExtractionPattern,
}

/// A pre-compiled extraction that can be reused across multiple source inputs.
///
/// Stores compiled `tree_sitter::Query` objects and their capture names so they
/// don't need to be recompiled for every call. A `QueryCursor` is reused across
/// patterns within a single extraction call, making this type `Send + Sync`.
pub(crate) struct CompiledExtraction {
    language: tree_sitter::Language,
    language_name: String,
    patterns: Vec<CompiledPattern>,
}

// tree_sitter::Query is Send + Sync, tree_sitter::Language is Send + Sync.
// SAFETY: All fields are Send + Sync. QueryCursor is created per-call, not stored.
unsafe impl Send for CompiledExtraction {}
// SAFETY: Same reasoning — no interior mutability, cursors created per call.
unsafe impl Sync for CompiledExtraction {}

impl std::fmt::Debug for CompiledExtraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledExtraction")
            .field("language_name", &self.language_name)
            .field("pattern_count", &self.patterns.len())
            .finish()
    }
}

/// Run extraction patterns against source code, parsing and querying in one step.
///
/// This is the simplest entry point. For repeated extractions with the same
/// config, prefer [`CompiledExtraction::compile`] to avoid recompiling queries.
///
/// # Errors
///
/// Returns an error if the language is not found, parsing fails, or a query
/// pattern is invalid.
pub(crate) fn extract(source: &str, config: &ExtractionConfig) -> Result<ExtractionResult, Error> {
    let compiled = CompiledExtraction::compile(config)?;
    compiled.extract(source)
}

/// Validate an extraction config without running it.
///
/// Checks that the language exists and all query patterns compile. Returns
/// detailed diagnostics per pattern.
///
/// # Errors
///
/// Returns an error if the language cannot be loaded.
pub(crate) fn validate_extraction(config: &ExtractionConfig) -> Result<ValidationResult, Error> {
    let lang = crate::get_language(&config.language)?;
    let mut all_valid = true;
    let mut patterns = AHashMap::new();

    for (name, pat) in &config.patterns {
        match tree_sitter::Query::new(&lang, &pat.query) {
            Ok(query) => {
                let capture_names: Vec<String> = query.capture_names().iter().map(|s| s.to_string()).collect();
                let pattern_count = query.pattern_count();

                let mut warnings = Vec::new();
                // Check if requested child_fields are plausible
                for field in &pat.child_fields {
                    if field.is_empty() {
                        warnings.push(format!("empty child field name in pattern '{name}'"));
                    }
                }

                patterns.insert(
                    name.clone(),
                    PatternValidation {
                        valid: true,
                        capture_names,
                        pattern_count,
                        warnings,
                        errors: Vec::new(),
                    },
                );
            }
            Err(e) => {
                all_valid = false;
                patterns.insert(
                    name.clone(),
                    PatternValidation {
                        valid: false,
                        capture_names: Vec::new(),
                        pattern_count: 0,
                        warnings: Vec::new(),
                        errors: vec![e.to_string()],
                    },
                );
            }
        }
    }

    Ok(ValidationResult {
        valid: all_valid,
        patterns,
    })
}

impl CompiledExtraction {
    /// Compile an extraction config for repeated use.
    ///
    /// # Errors
    ///
    /// Returns an error if the language is not found or any query pattern is invalid.
    pub fn compile(config: &ExtractionConfig) -> Result<Self, Error> {
        let language = crate::get_language(&config.language)?;
        Self::compile_with_language(language, &config.language, &config.patterns)
    }

    /// Compile extraction patterns using a pre-loaded `tree_sitter::Language`.
    ///
    /// This avoids a redundant language registry lookup when the caller already
    /// has the language (e.g., from an earlier parse step).
    ///
    /// # Errors
    ///
    /// Returns an error if any query pattern is invalid.
    pub fn compile_with_language(
        language: tree_sitter::Language,
        language_name: &str,
        extraction_patterns: &AHashMap<String, ExtractionPattern>,
    ) -> Result<Self, Error> {
        let mut patterns = Vec::with_capacity(extraction_patterns.len());

        for (name, pat) in extraction_patterns {
            let query = tree_sitter::Query::new(&language, &pat.query)
                .map_err(|e| Error::QueryError(format!("pattern '{name}': {e}")))?;
            let capture_names = query.capture_names().iter().map(|s| s.to_string()).collect();
            patterns.push(CompiledPattern {
                name: name.clone(),
                query,
                capture_names,
                config: pat.clone(),
            });
        }

        Ok(Self {
            language,
            language_name: language_name.to_string(),
            patterns,
        })
    }

    /// Extract from source code, parsing it first.
    ///
    /// Uses the thread-local parser cache to avoid creating a new parser on
    /// every call.
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails.
    pub fn extract(&self, source: &str) -> Result<ExtractionResult, Error> {
        let tree = crate::parse::parse_with_language(&self.language_name, &self.language, source.as_bytes())?;
        self.extract_from_tree(&tree, source.as_bytes())
    }

    /// Extract from an already-parsed tree.
    ///
    /// # Errors
    ///
    /// Returns an error if query execution fails.
    pub fn extract_from_tree(&self, tree: &tree_sitter::Tree, source: &[u8]) -> Result<ExtractionResult, Error> {
        use tree_sitter::StreamingIterator;

        let mut results = AHashMap::with_capacity(self.patterns.len());
        let mut cursor = tree_sitter::QueryCursor::new();

        for cp in &self.patterns {
            // Reset byte range between patterns to avoid leaking state.
            if let Some((start, end)) = cp.config.byte_range {
                cursor.set_byte_range(start..end);
            } else {
                cursor.set_byte_range(0..usize::MAX);
            }

            let mut matches_iter = cursor.matches(&cp.query, tree.root_node(), source);
            let mut match_results = Vec::new();
            let mut total_count: usize = 0;

            while let Some(m) = matches_iter.next() {
                total_count += 1;

                // If we already hit max_results, keep counting but don't collect.
                if let Some(max) = cp.config.max_results
                    && match_results.len() >= max
                {
                    continue;
                }

                let mut captures = Vec::with_capacity(m.captures.len());
                for cap in m.captures {
                    let cap_name = cp
                        .capture_names
                        .get(cap.index as usize)
                        .ok_or_else(|| Error::QueryError(format!("invalid capture index {}", cap.index)))?;
                    let ts_node = cap.node;
                    let info = node_info_from_node(ts_node);
                    let capture_start_byte = info.start_byte;

                    let text = match cp.config.capture_output {
                        CaptureOutput::Text | CaptureOutput::Full => {
                            crate::node::extract_text(source, &info).ok().map(String::from)
                        }
                        CaptureOutput::Node => None,
                    };

                    let node = match cp.config.capture_output {
                        CaptureOutput::Node | CaptureOutput::Full => Some(info),
                        CaptureOutput::Text => None,
                    };

                    // Extract requested child fields from the actual tree_sitter::Node.
                    let child_field_values = if cp.config.child_fields.is_empty() {
                        AHashMap::new()
                    } else {
                        let mut fields = AHashMap::with_capacity(cp.config.child_fields.len());
                        for field_name in &cp.config.child_fields {
                            let value = ts_node.child_by_field_name(field_name.as_str()).and_then(|child| {
                                let child_info = node_info_from_node(child);
                                crate::node::extract_text(source, &child_info).ok().map(String::from)
                            });
                            fields.insert(field_name.clone(), value);
                        }
                        fields
                    };

                    captures.push(CaptureResult {
                        name: cap_name.clone(),
                        node,
                        text,
                        child_fields: child_field_values,
                        start_byte: capture_start_byte,
                    });
                }

                match_results.push(MatchResult {
                    pattern_index: m.pattern_index,
                    captures,
                });
            }

            // Sort matches by the start byte of their first capture.
            match_results.sort_by_key(|m| m.captures.first().map_or(0, |c| c.start_byte));

            results.insert(
                cp.name.clone(),
                PatternResult {
                    matches: match_results,
                    total_count,
                },
            );
        }

        Ok(ExtractionResult {
            language: self.language_name.clone(),
            results,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns true if Python is not available (tests should early-return).
    fn skip_if_no_python() -> bool {
        !crate::has_language("python")
    }

    fn python_config(patterns: AHashMap<String, ExtractionPattern>) -> ExtractionConfig {
        ExtractionConfig {
            language: "python".to_string(),
            patterns,
        }
    }

    fn single_pattern(name: &str, query: &str) -> AHashMap<String, ExtractionPattern> {
        let mut m = AHashMap::new();
        m.insert(
            name.to_string(),
            ExtractionPattern {
                query: query.to_string(),
                capture_output: CaptureOutput::default(),
                child_fields: Vec::new(),
                max_results: None,
                byte_range: None,
            },
        );
        m
    }

    #[test]
    fn test_basic_extraction() {
        if skip_if_no_python() {
            return;
        }
        let config = python_config(single_pattern(
            "functions",
            "(function_definition name: (identifier) @fn_name) @fn_def",
        ));
        let result = extract("def hello():\n    pass\n\ndef world():\n    pass\n", &config).unwrap();
        assert_eq!(result.language, "python");

        let fns = &result.results["functions"];
        assert_eq!(fns.total_count, 2);
        assert_eq!(fns.matches.len(), 2);

        // Each match should have two captures: fn_def and fn_name
        for m in &fns.matches {
            assert_eq!(m.captures.len(), 2);
        }
    }

    #[test]
    fn test_capture_output_text_only() {
        if skip_if_no_python() {
            return;
        }
        let mut patterns = AHashMap::new();
        patterns.insert(
            "names".to_string(),
            ExtractionPattern {
                query: "(function_definition name: (identifier) @fn_name)".to_string(),
                capture_output: CaptureOutput::Text,
                child_fields: Vec::new(),
                max_results: None,
                byte_range: None,
            },
        );
        let config = python_config(patterns);
        let result = extract("def foo():\n    pass\n", &config).unwrap();
        let names = &result.results["names"];
        assert_eq!(names.matches.len(), 1);

        let cap = &names.matches[0].captures[0];
        assert_eq!(cap.name, "fn_name");
        assert!(cap.text.is_some());
        assert_eq!(cap.text.as_deref(), Some("foo"));
        assert!(cap.node.is_none(), "Text mode should not include NodeInfo");
    }

    #[test]
    fn test_capture_output_node_only() {
        if skip_if_no_python() {
            return;
        }
        let mut patterns = AHashMap::new();
        patterns.insert(
            "names".to_string(),
            ExtractionPattern {
                query: "(function_definition name: (identifier) @fn_name)".to_string(),
                capture_output: CaptureOutput::Node,
                child_fields: Vec::new(),
                max_results: None,
                byte_range: None,
            },
        );
        let config = python_config(patterns);
        let result = extract("def foo():\n    pass\n", &config).unwrap();
        let cap = &result.results["names"].matches[0].captures[0];
        assert!(cap.node.is_some(), "Node mode should include NodeInfo");
        assert!(cap.text.is_none(), "Node mode should not include text");
    }

    #[test]
    fn test_capture_output_full() {
        if skip_if_no_python() {
            return;
        }
        let mut patterns = AHashMap::new();
        patterns.insert(
            "names".to_string(),
            ExtractionPattern {
                query: "(function_definition name: (identifier) @fn_name)".to_string(),
                capture_output: CaptureOutput::Full,
                child_fields: Vec::new(),
                max_results: None,
                byte_range: None,
            },
        );
        let config = python_config(patterns);
        let result = extract("def foo():\n    pass\n", &config).unwrap();
        let cap = &result.results["names"].matches[0].captures[0];
        assert!(cap.node.is_some(), "Full mode should include NodeInfo");
        assert!(cap.text.is_some(), "Full mode should include text");
        assert_eq!(cap.text.as_deref(), Some("foo"));
    }

    #[test]
    fn test_child_fields_extraction() {
        if skip_if_no_python() {
            return;
        }
        let mut patterns = AHashMap::new();
        patterns.insert(
            "functions".to_string(),
            ExtractionPattern {
                query: "(function_definition) @fn_def".to_string(),
                capture_output: CaptureOutput::Full,
                child_fields: vec!["name".to_string(), "parameters".to_string()],
                max_results: None,
                byte_range: None,
            },
        );
        let config = python_config(patterns);
        let result = extract("def greet(name):\n    pass\n", &config).unwrap();
        let fns = &result.results["functions"];
        assert_eq!(fns.matches.len(), 1);

        let cap = &fns.matches[0].captures[0];
        assert!(cap.child_fields.contains_key("name"));
        assert_eq!(cap.child_fields["name"].as_deref(), Some("greet"));
        assert!(cap.child_fields.contains_key("parameters"));
        // parameters should contain "(name)"
        assert!(cap.child_fields["parameters"].is_some());
    }

    #[test]
    fn test_validation_valid_query() {
        if skip_if_no_python() {
            return;
        }
        let config = python_config(single_pattern(
            "fns",
            "(function_definition name: (identifier) @fn_name)",
        ));
        let validation = validate_extraction(&config).unwrap();
        assert!(validation.valid);
        let pv = &validation.patterns["fns"];
        assert!(pv.valid);
        assert!(pv.capture_names.contains(&"fn_name".to_string()));
        assert!(pv.errors.is_empty());
    }

    #[test]
    fn test_validation_invalid_query() {
        if skip_if_no_python() {
            return;
        }
        let config = python_config(single_pattern("bad", "((((not valid syntax"));
        let validation = validate_extraction(&config).unwrap();
        assert!(!validation.valid);
        let pv = &validation.patterns["bad"];
        assert!(!pv.valid);
        assert!(!pv.errors.is_empty());
    }

    #[test]
    fn test_validation_unknown_language() {
        let config = ExtractionConfig {
            language: "nonexistent_xyz_lang".to_string(),
            patterns: AHashMap::new(),
        };
        let result = validate_extraction(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_max_results_truncation() {
        if skip_if_no_python() {
            return;
        }
        let mut patterns = AHashMap::new();
        patterns.insert(
            "fns".to_string(),
            ExtractionPattern {
                query: "(function_definition name: (identifier) @fn_name)".to_string(),
                capture_output: CaptureOutput::Text,
                child_fields: Vec::new(),
                max_results: Some(1),
                byte_range: None,
            },
        );
        let config = python_config(patterns);
        let result = extract("def a():\n    pass\ndef b():\n    pass\ndef c():\n    pass\n", &config).unwrap();
        let fns = &result.results["fns"];
        assert_eq!(fns.matches.len(), 1, "should be truncated to max_results=1");
        assert_eq!(fns.total_count, 3, "total_count should reflect all matches");
    }

    #[test]
    fn test_compiled_extraction_reuse() {
        if skip_if_no_python() {
            return;
        }
        let config = python_config(single_pattern(
            "fns",
            "(function_definition name: (identifier) @fn_name)",
        ));
        let compiled = CompiledExtraction::compile(&config).unwrap();

        let r1 = compiled.extract("def a():\n    pass\n").unwrap();
        let r2 = compiled.extract("def x():\n    pass\ndef y():\n    pass\n").unwrap();

        assert_eq!(r1.results["fns"].total_count, 1);
        assert_eq!(r2.results["fns"].total_count, 2);
    }

    #[test]
    fn test_empty_results() {
        if skip_if_no_python() {
            return;
        }
        let config = python_config(single_pattern(
            "classes",
            "(class_definition name: (identifier) @cls_name)",
        ));
        // Source has no classes.
        let result = extract("x = 1\n", &config).unwrap();
        let classes = &result.results["classes"];
        assert!(classes.matches.is_empty());
        assert_eq!(classes.total_count, 0);
    }

    #[test]
    fn test_send_sync() {
        fn assert_send<T: Send>() {}
        fn assert_sync<T: Sync>() {}
        assert_send::<CompiledExtraction>();
        assert_sync::<CompiledExtraction>();
        assert_send::<ExtractionResult>();
        assert_sync::<ExtractionResult>();
        assert_send::<ExtractionConfig>();
        assert_sync::<ExtractionConfig>();
        assert_send::<CaptureOutput>();
        assert_sync::<CaptureOutput>();
    }

    #[test]
    fn test_byte_range_restriction() {
        if skip_if_no_python() {
            return;
        }
        let source = "def a():\n    pass\ndef b():\n    pass\ndef c():\n    pass\n";
        // Restrict to byte range covering only the second function.
        let second_fn_start = source.find("def b").unwrap();
        let second_fn_end = source[second_fn_start..]
            .find("def c")
            .map_or(source.len(), |i| second_fn_start + i);
        let mut patterns = AHashMap::new();
        patterns.insert(
            "fns".to_string(),
            ExtractionPattern {
                query: "(function_definition name: (identifier) @fn_name)".to_string(),
                capture_output: CaptureOutput::Text,
                child_fields: Vec::new(),
                max_results: None,
                byte_range: Some((second_fn_start, second_fn_end)),
            },
        );
        let config = python_config(patterns);
        let result = extract(source, &config).unwrap();
        let fns = &result.results["fns"];
        assert_eq!(fns.matches.len(), 1, "byte_range should restrict to one function");
        assert_eq!(
            fns.matches[0].captures[0].text.as_deref(),
            Some("b"),
            "should capture function 'b' within the byte range"
        );
    }

    #[test]
    fn test_result_ordering() {
        if skip_if_no_python() {
            return;
        }
        // Results should be sorted by source position regardless of capture mode.
        for mode in [CaptureOutput::Text, CaptureOutput::Node, CaptureOutput::Full] {
            let mut patterns = AHashMap::new();
            patterns.insert(
                "fns".to_string(),
                ExtractionPattern {
                    query: "(function_definition name: (identifier) @fn_name)".to_string(),
                    capture_output: mode.clone(),
                    child_fields: Vec::new(),
                    max_results: None,
                    byte_range: None,
                },
            );
            let config = python_config(patterns);
            let result = extract(
                "def alpha():\n    pass\ndef beta():\n    pass\ndef gamma():\n    pass\n",
                &config,
            )
            .unwrap();
            let fns = &result.results["fns"];
            assert_eq!(fns.matches.len(), 3);

            // Verify start_byte values are monotonically increasing.
            let start_bytes: Vec<usize> = fns.matches.iter().map(|m| m.captures[0].start_byte).collect();
            for pair in start_bytes.windows(2) {
                assert!(
                    pair[0] < pair[1],
                    "results should be sorted by position, got {start_bytes:?} in mode {mode:?}"
                );
            }
        }
    }

    #[test]
    fn test_extract_from_tree() {
        if skip_if_no_python() {
            return;
        }
        let config = python_config(single_pattern(
            "fns",
            "(function_definition name: (identifier) @fn_name)",
        ));
        let compiled = CompiledExtraction::compile(&config).unwrap();

        let source = "def hello():\n    pass\n";
        let tree = crate::parse::parse_string("python", source.as_bytes()).unwrap();
        let result = compiled.extract_from_tree(&tree, source.as_bytes()).unwrap();

        assert_eq!(result.results["fns"].total_count, 1);
        let cap = &result.results["fns"].matches[0].captures[0];
        assert_eq!(cap.text.as_deref(), Some("hello"));
    }

    #[test]
    fn test_byte_range_does_not_leak_between_patterns() {
        if skip_if_no_python() {
            return;
        }
        let source = "def a():\n    pass\ndef b():\n    pass\ndef c():\n    pass\n";
        let second_fn_start = source.find("def b").unwrap();
        let second_fn_end = source[second_fn_start..]
            .find("def c")
            .map_or(source.len(), |i| second_fn_start + i);

        // Two patterns: first has a byte_range, second does not.
        // The second pattern should see ALL functions, not just the byte_range.
        let mut patterns = AHashMap::new();
        patterns.insert(
            "restricted".to_string(),
            ExtractionPattern {
                query: "(function_definition name: (identifier) @fn_name)".to_string(),
                capture_output: CaptureOutput::Text,
                child_fields: Vec::new(),
                max_results: None,
                byte_range: Some((second_fn_start, second_fn_end)),
            },
        );
        patterns.insert(
            "unrestricted".to_string(),
            ExtractionPattern {
                query: "(function_definition name: (identifier) @fn_name)".to_string(),
                capture_output: CaptureOutput::Text,
                child_fields: Vec::new(),
                max_results: None,
                byte_range: None,
            },
        );

        let config = python_config(patterns);
        let compiled = CompiledExtraction::compile(&config).unwrap();
        let result = compiled.extract(source).unwrap();

        let restricted = &result.results["restricted"];
        assert_eq!(restricted.matches.len(), 1, "restricted pattern should find 1 function");

        let unrestricted = &result.results["unrestricted"];
        assert_eq!(
            unrestricted.matches.len(),
            3,
            "unrestricted pattern should find all 3 functions, not be limited by previous byte_range"
        );
    }

    #[test]
    fn test_compiled_extraction_capture_names_precomputed() {
        if skip_if_no_python() {
            return;
        }
        // Verify capture names are correct when using multiple captures.
        let mut patterns = AHashMap::new();
        patterns.insert(
            "fns".to_string(),
            ExtractionPattern {
                query: "(function_definition name: (identifier) @fn_name) @fn_def".to_string(),
                capture_output: CaptureOutput::Full,
                child_fields: Vec::new(),
                max_results: None,
                byte_range: None,
            },
        );
        let config = python_config(patterns);
        let result = extract("def hello():\n    pass\n", &config).unwrap();
        let fns = &result.results["fns"];
        assert_eq!(fns.matches.len(), 1);
        let names: Vec<&str> = fns.matches[0].captures.iter().map(|c| c.name.as_str()).collect();
        assert!(names.contains(&"fn_name"), "should have fn_name capture");
        assert!(names.contains(&"fn_def"), "should have fn_def capture");
    }
}
