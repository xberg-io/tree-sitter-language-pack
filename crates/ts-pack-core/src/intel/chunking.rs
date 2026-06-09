use memchr::memchr_iter;
use tree_sitter::{Language, Tree};

use super::types::*;

/// Chunk source code and produce rich metadata per chunk.
///
/// Uses the vendored text-splitter algorithm for AST-aware splitting,
/// then overlays rich metadata on each resulting chunk.
pub fn chunk_source(
    source: &str,
    language: &str,
    max_chunk_size: usize,
    _lang: &Language,
    tree: &Tree,
) -> Vec<CodeChunk> {
    // Split source into chunks using AST-aware boundaries
    let raw_chunks = crate::text_splitter::split_code(source, tree, max_chunk_size);
    let total_chunks = raw_chunks.len();
    let root = tree.root_node();

    // Precompute newline positions once for O(log n) line number lookup per chunk
    let newline_positions: Vec<usize> = memchr_iter(b'\n', source.as_bytes()).collect();

    raw_chunks
        .into_iter()
        .enumerate()
        .map(|(idx, (start_byte, end_byte))| {
            let content = &source[start_byte..end_byte];
            let start_line = newline_positions.partition_point(|&pos| pos < start_byte);
            let end_line = newline_positions.partition_point(|&pos| pos < end_byte);

            // Collect metadata for this chunk's byte range
            let mut node_types = Vec::new();
            let mut symbols_defined = Vec::new();
            let mut comments = Vec::new();
            let mut docstrings = Vec::new();
            let mut has_error_nodes = false;
            let mut context_path = Vec::new();

            let mut collector = MetadataCollector {
                node_types: &mut node_types,
                symbols: &mut symbols_defined,
                comments: &mut comments,
                docstrings: &mut docstrings,
                has_errors: &mut has_error_nodes,
                context_path: &mut context_path,
            };
            collect_chunk_metadata(&root, source, start_byte, end_byte, &mut collector, 0);

            CodeChunk {
                content: content.to_string(),
                start_byte,
                end_byte,
                start_line,
                end_line,
                metadata: ChunkContext {
                    language: language.to_string(),
                    chunk_index: idx,
                    total_chunks,
                    node_types,
                    context_path,
                    symbols_defined,
                    comments,
                    docstrings,
                    has_error_nodes,
                },
            }
        })
        .collect()
}

fn span_from_node(node: &tree_sitter::Node) -> Span {
    let s = node.start_position();
    let e = node.end_position();
    Span {
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_line: s.row,
        start_column: s.column,
        end_line: e.row,
        end_column: e.column,
    }
}

fn node_text<'a>(node: &tree_sitter::Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

#[allow(dead_code)]
struct MetadataCollector<'a> {
    node_types: &'a mut Vec<String>,
    symbols: &'a mut Vec<String>,
    comments: &'a mut Vec<CommentInfo>,
    docstrings: &'a mut Vec<DocstringInfo>,
    has_errors: &'a mut bool,
    context_path: &'a mut Vec<String>,
}

fn collect_chunk_metadata(
    node: &tree_sitter::Node,
    source: &str,
    chunk_start: usize,
    chunk_end: usize,
    collector: &mut MetadataCollector<'_>,
    depth: usize,
) {
    // Skip nodes entirely outside the chunk range
    if node.end_byte() <= chunk_start || node.start_byte() >= chunk_end {
        return;
    }

    let kind = node.kind();

    // Track top-level node types in the chunk
    if depth <= 1
        && node.start_byte() >= chunk_start
        && node.end_byte() <= chunk_end
        && !collector.node_types.iter().any(|t| t == kind)
    {
        collector.node_types.push(kind.to_string());
    }

    // Track errors
    if node.is_error() || node.is_missing() {
        *collector.has_errors = true;
    }

    // Track symbols defined in this chunk
    let is_definition = matches!(
        kind,
        "function_definition"
            | "function_declaration"
            | "function_item"
            | "class_definition"
            | "class_declaration"
            | "struct_item"
            | "struct_definition"
            | "enum_item"
            | "enum_declaration"
            | "method_definition"
            | "method_declaration"
            | "trait_item"
            | "impl_item"
    );
    if is_definition {
        let name_node = node
            .child_by_field_name("name")
            .or_else(|| node.child_by_field_name("declarator"))
            .or_else(|| node.child_by_field_name("binding"));
        if let Some(name_node) = name_node {
            let name = node_text(&name_node, source).to_string();
            collector.symbols.push(name.clone());
            // Build context path for enclosing scopes
            if node.start_byte() < chunk_start {
                collector.context_path.push(name);
            }
        }
    }

    // Extract comments in this chunk
    if (kind == "comment" || kind == "line_comment" || kind == "block_comment")
        && node.start_byte() >= chunk_start
        && node.end_byte() <= chunk_end
    {
        let text = node_text(node, source).to_string();
        let comment_kind = if kind == "block_comment" {
            CommentKind::Block
        } else if kind == "doc_comment"
            || kind == "documentation_comment"
            || text.starts_with("///")
            || text.starts_with("//!")
            || text.starts_with("/**")
            || text.starts_with("/*!")
        {
            CommentKind::Doc
        } else {
            CommentKind::Line
        };
        collector.comments.push(CommentInfo {
            text,
            kind: comment_kind,
            span: span_from_node(node),
            associated_node: node.next_named_sibling().map(|n| n.kind().to_string()),
        });
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_chunk_metadata(&child, source, chunk_start, chunk_end, collector, depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_with(source: &str, lang_name: &str) -> Option<(tree_sitter::Language, tree_sitter::Tree)> {
        let registry = crate::LanguageRegistry::new();
        let lang = registry.get_language(lang_name).ok()?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&lang).ok()?;
        let tree = parser.parse(source, None)?;
        Some((lang, tree))
    }

    #[test]
    fn test_chunk_small_source() {
        let source = "def foo():\n    pass\n";
        let Some((lang, tree)) = parse_with(source, "python") else {
            return;
        };
        let chunks = chunk_source(source, "python", 10000, &lang, &tree);

        assert_eq!(chunks.len(), 1, "small source should fit in one chunk");
        assert_eq!(chunks[0].content, source);
        assert_eq!(chunks[0].start_byte, 0);
        assert_eq!(chunks[0].end_byte, source.len());
        assert_eq!(chunks[0].metadata.language, "python");
        assert_eq!(chunks[0].metadata.chunk_index, 0);
        assert_eq!(chunks[0].metadata.total_chunks, 1);
    }

    #[test]
    fn test_chunk_large_source_produces_multiple() {
        let source = "def foo():\n    pass\ndef bar():\n    pass\ndef baz():\n    pass\n";
        let Some((lang, tree)) = parse_with(source, "python") else {
            return;
        };
        let chunks = chunk_source(source, "python", 20, &lang, &tree);

        assert!(chunks.len() >= 2, "small max_chunk_size should produce multiple chunks");
        // Verify contiguity
        for window in chunks.windows(2) {
            assert_eq!(window[0].end_byte, window[1].start_byte, "chunks must be contiguous");
        }
        assert_eq!(chunks.first().unwrap().start_byte, 0);
        assert_eq!(chunks.last().unwrap().end_byte, source.len());
        // Verify metadata
        for (i, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.metadata.chunk_index, i);
            assert_eq!(chunk.metadata.total_chunks, chunks.len());
        }
    }

    #[test]
    fn test_chunk_metadata_symbols() {
        let source = "def alpha():\n    pass\ndef beta():\n    pass\n";
        let Some((lang, tree)) = parse_with(source, "python") else {
            return;
        };
        let chunks = chunk_source(source, "python", 10000, &lang, &tree);

        // Single chunk with both functions
        assert_eq!(chunks.len(), 1);
        let syms = &chunks[0].metadata.symbols_defined;
        assert!(syms.contains(&"alpha".to_string()), "should contain alpha");
        assert!(syms.contains(&"beta".to_string()), "should contain beta");
    }

    #[test]
    fn test_chunk_metadata_comments() {
        let source = "# A comment\ndef foo():\n    pass\n";
        let Some((lang, tree)) = parse_with(source, "python") else {
            return;
        };
        let chunks = chunk_source(source, "python", 10000, &lang, &tree);

        assert_eq!(chunks.len(), 1);
        assert!(
            !chunks[0].metadata.comments.is_empty(),
            "should extract comment metadata"
        );
    }

    #[test]
    fn test_chunk_has_error_nodes() {
        // Use Python with clearly invalid syntax
        let source = "def :\n    pass\n";
        let Some((lang, tree)) = parse_with(source, "python") else {
            return;
        };
        let chunks = chunk_source(source, "python", 10000, &lang, &tree);

        assert_eq!(chunks.len(), 1);
        assert!(
            chunks[0].metadata.has_error_nodes,
            "invalid source should set has_error_nodes"
        );
    }
}
