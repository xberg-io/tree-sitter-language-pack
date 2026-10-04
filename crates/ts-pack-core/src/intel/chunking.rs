use memchr::memchr_iter;
use tree_sitter::{Language, Tree};

use super::intelligence::{comment_at, docstring_at};
use super::types::*;
use super::walk::{Descend, walk_bounded, warn_if_truncated};

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
    let raw_chunks = crate::text_splitter::split_code(source, tree, max_chunk_size);
    let total_chunks = raw_chunks.len();
    let root = tree.root_node();

    let newline_positions: Vec<usize> = memchr_iter(b'\n', source.as_bytes()).collect();
    let mut truncated = 0usize;

    let chunks: Vec<CodeChunk> = raw_chunks
        .into_iter()
        .enumerate()
        .map(|(idx, (start_byte, end_byte))| {
            let content = &source[start_byte..end_byte];
            let start_line = newline_positions.partition_point(|&pos| pos < start_byte);
            let end_line = chunk_end_line(&newline_positions, start_byte, end_byte);

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
            truncated += collect_chunk_metadata(&root, source, language, start_byte, end_byte, &mut collector);

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
        .collect();

    warn_if_truncated(truncated, "intel::chunking", language);
    tracing::debug!(
        target: "ts_pack::intel",
        operation = "intel::chunking",
        language,
        max_chunk_size,
        chunks = total_chunks,
        "chunking complete"
    );
    chunks
}

/// The zero-indexed row of a chunk's last included byte.
///
/// ~keep `end_byte` is exclusive, so computing its row directly counts the
/// ~keep newline the chunk itself ends on when the source has a trailing
/// ~keep newline, landing one row past the chunk's actual last content row —
/// ~keep but not when the source has no trailing newline, since then there is
/// ~keep no such newline to count. Rows a chunk with identical content would
/// ~keep report would then differ by one depending on a byte outside the
/// ~keep chunk entirely. Computing the row of `end_byte - 1` instead — the
/// ~keep last byte actually in the chunk — is immune to what comes after it.
fn chunk_end_line(newline_positions: &[usize], start_byte: usize, end_byte: usize) -> usize {
    if end_byte <= start_byte {
        return newline_positions.partition_point(|&pos| pos < start_byte);
    }
    newline_positions.partition_point(|&pos| pos < end_byte - 1)
}

fn node_text<'a>(node: &tree_sitter::Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

pub(super) struct MetadataCollector<'a> {
    pub(super) node_types: &'a mut Vec<String>,
    pub(super) symbols: &'a mut Vec<String>,
    pub(super) comments: &'a mut Vec<CommentInfo>,
    pub(super) docstrings: &'a mut Vec<DocstringInfo>,
    pub(super) has_errors: &'a mut bool,
    pub(super) context_path: &'a mut Vec<String>,
}

/// Whether `node` lies entirely outside `[chunk_start, chunk_end)` and its
/// subtree should be pruned from this chunk's walk.
///
/// A zero-width node (tree-sitter's synthetic `MISSING` nodes) needs an
/// inclusive test at both boundaries: the ordinary half-open test excludes a
/// zero-width node sitting exactly on `chunk_start` (`end_byte <=
/// chunk_start` is trivially true when `start_byte == end_byte ==
/// chunk_start`) as well as one sitting exactly on `chunk_end`, dropping it
/// from *every* chunk rather than attributing it to a neighbour. Letting it
/// belong to both chunks adjacent to the boundary is the safe direction: a
/// caller that skips a `MISSING` node still sees `has_error_nodes` from an
/// ordinary error node elsewhere, but a caller that needs it can never
/// recover a node this excluded outright.
fn is_outside_chunk(node: &tree_sitter::Node, chunk_start: usize, chunk_end: usize) -> bool {
    if node.start_byte() == node.end_byte() {
        return node.start_byte() < chunk_start || node.start_byte() > chunk_end;
    }
    node.end_byte() <= chunk_start || node.start_byte() >= chunk_end
}

/// Collect metadata for one chunk from the nodes that overlap it.
///
/// Returns the number of nodes dropped because the tree was deeper than the
/// traversal limit. Nodes that do not overlap the chunk prune their subtree,
/// exactly as the previous recursive early return did.
fn collect_chunk_metadata(
    root: &tree_sitter::Node,
    source: &str,
    language: &str,
    chunk_start: usize,
    chunk_end: usize,
    collector: &mut MetadataCollector<'_>,
) -> usize {
    walk_bounded(root, |node, _depth| {
        if is_outside_chunk(node, chunk_start, chunk_end) {
            return Descend::Skip;
        }
        record_chunk_node(node, source, language, chunk_start, chunk_end, collector);
        Descend::Children
    })
}

/// Node kinds whose `name` is a symbol definition.
const DEFINITION_NODE_KINDS: &[&str] = &[
    "function_definition",
    "function_declaration",
    "function_item",
    "class_definition",
    "class_declaration",
    "struct_item",
    "struct_definition",
    "enum_item",
    "enum_declaration",
    "method_definition",
    "method_declaration",
    "trait_item",
    "impl_item",
];

/// Whether `node` lies wholly inside the chunk.
///
/// ~keep The root spans the whole file, so treating it as contained would make
/// ~keep every top-level item a *nested* node and leave `node_types` empty for a
/// ~keep single-chunk file. It is deliberately never contained.
fn is_contained(node: &tree_sitter::Node, chunk_start: usize, chunk_end: usize) -> bool {
    node.parent().is_some() && node.start_byte() >= chunk_start && node.end_byte() <= chunk_end
}

/// Whether `node` is one of the chunk's outermost nodes — contained in it, but
/// with a parent that is not. This is what `ChunkContext::node_types` documents:
/// the kinds that appear at the top level of the chunk.
fn is_chunk_top_level(node: &tree_sitter::Node, chunk_start: usize, chunk_end: usize) -> bool {
    node.is_named()
        && is_contained(node, chunk_start, chunk_end)
        && node
            .parent()
            .is_none_or(|parent| !is_contained(&parent, chunk_start, chunk_end))
}

/// Record one node's contribution to a chunk's metadata.
pub(super) fn record_chunk_node(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    chunk_start: usize,
    chunk_end: usize,
    collector: &mut MetadataCollector<'_>,
) {
    let kind = node.kind();

    if is_chunk_top_level(node, chunk_start, chunk_end) && !collector.node_types.iter().any(|t| t == kind) {
        collector.node_types.push(kind.to_string());
    }

    if node.is_error() || node.is_missing() {
        *collector.has_errors = true;
    }

    if DEFINITION_NODE_KINDS.contains(&kind) {
        record_definition_name(node, source, chunk_start, chunk_end, collector);
    }

    if is_contained(node, chunk_start, chunk_end) {
        if let Some(comment) = comment_at(node, source) {
            collector.comments.push(comment);
        }
        if let Some(docstring) = docstring_at(node, source, language) {
            collector.docstrings.push(docstring);
        }
    }
}

/// Attribute a definition's name to the chunk that holds it.
///
/// A definition only *overlaps* a chunk when the splitter cut its body across
/// chunk boundaries; every one of those fragments used to claim the name in
/// `symbols_defined`. The name is a symbol of the chunk only when the whole
/// definition fits inside it, and enclosing context otherwise.
fn record_definition_name(
    node: &tree_sitter::Node,
    source: &str,
    chunk_start: usize,
    chunk_end: usize,
    collector: &mut MetadataCollector<'_>,
) {
    let name_node = node
        .child_by_field_name("name")
        .or_else(|| node.child_by_field_name("declarator"))
        .or_else(|| node.child_by_field_name("binding"));
    let Some(name_node) = name_node else { return };
    let name = node_text(&name_node, source).to_string();

    if node.start_byte() < chunk_start {
        collector.context_path.push(name);
        return;
    }
    if node.end_byte() <= chunk_end {
        collector.symbols.push(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intel::test_support::parse_with_language_or_skip;

    // ~keep A missing grammar reports `SKIPPED` on stderr; see `intel::test_support`.
    fn parse_with(source: &str, lang_name: &str) -> Option<(tree_sitter::Language, tree_sitter::Tree)> {
        parse_with_language_or_skip(source, lang_name)
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
        for window in chunks.windows(2) {
            assert_eq!(window[0].end_byte, window[1].start_byte, "chunks must be contiguous");
        }
        assert_eq!(chunks.first().unwrap().start_byte, 0);
        assert_eq!(chunks.last().unwrap().end_byte, source.len());
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

    /// A class whose body the splitter has to cut across several chunks.
    const SPLIT_CLASS_SOURCE: &str = "class Big:\n    def alpha(self):\n        return 1\n\n    def beta(self):\n        return 2\n\n    def gamma(self):\n        return 3\n";

    #[test]
    fn should_record_top_level_node_types_for_a_single_chunk_file() {
        let source = "# lead\ndef foo():\n    pass\n";
        let Some((lang, tree)) = parse_with(source, "python") else {
            return;
        };
        let chunks = chunk_source(source, "python", 10000, &lang, &tree);

        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks[0].metadata.node_types,
            vec!["comment".to_string(), "function_definition".to_string()],
            "the chunk's top level is the file's top-level items, not the root node"
        );
    }

    #[test]
    fn should_record_node_types_for_chunks_of_a_split_definition() {
        let Some((lang, tree)) = parse_with(SPLIT_CLASS_SOURCE, "python") else {
            return;
        };
        let chunks = chunk_source(SPLIT_CLASS_SOURCE, "python", 40, &lang, &tree);

        assert!(
            chunks.len() >= 3,
            "the sample must actually split; got {}",
            chunks.len()
        );
        let with_types = chunks.iter().filter(|c| !c.metadata.node_types.is_empty()).count();
        assert!(
            with_types >= 2,
            "chunks inside a split definition must still report node types; {with_types} of {} did",
            chunks.len()
        );
    }

    #[test]
    fn should_not_claim_a_split_definition_as_defined_by_every_chunk() {
        let Some((lang, tree)) = parse_with(SPLIT_CLASS_SOURCE, "python") else {
            return;
        };
        let chunks = chunk_source(SPLIT_CLASS_SOURCE, "python", 40, &lang, &tree);

        assert!(
            chunks.len() >= 3,
            "the sample must actually split; got {}",
            chunks.len()
        );
        let claiming: Vec<usize> = chunks
            .iter()
            .enumerate()
            .filter(|(_, c)| c.metadata.symbols_defined.iter().any(|s| s == "Big"))
            .map(|(index, _)| index)
            .collect();
        assert!(
            claiming.is_empty(),
            "no chunk contains the whole class, so none defines it; chunks {claiming:?} claimed it"
        );
        assert!(
            chunks
                .iter()
                .any(|c| c.metadata.context_path.iter().any(|s| s == "Big")),
            "chunks after the first must record the class as enclosing context"
        );
    }

    #[test]
    fn test_chunk_has_error_nodes() {
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

    #[test]
    fn should_populate_docstrings_for_a_python_docstring_inside_a_chunk() {
        let source = "def foo():\n    \"\"\"Say hello.\"\"\"\n    pass\n";
        let Some((lang, tree)) = parse_with(source, "python") else {
            return;
        };
        let chunks = chunk_source(source, "python", 10000, &lang, &tree);

        assert_eq!(chunks.len(), 1);
        assert_eq!(
            chunks[0].metadata.docstrings.len(),
            1,
            "should extract docstring metadata"
        );
        assert_eq!(chunks[0].metadata.docstrings[0].text, "\"\"\"Say hello.\"\"\"");
    }

    #[test]
    fn should_report_the_same_end_line_regardless_of_a_trailing_newline() {
        let with_newline = "line0\nline1\nline2\n";
        let without_newline = "line0\nline1\nline2";

        let Some((lang_with, tree_with)) = parse_with(with_newline, "python") else {
            return;
        };
        let Some((lang_without, tree_without)) = parse_with(without_newline, "python") else {
            return;
        };

        let chunks_with = chunk_source(with_newline, "python", 10000, &lang_with, &tree_with);
        let chunks_without = chunk_source(without_newline, "python", 10000, &lang_without, &tree_without);

        assert_eq!(chunks_with.len(), 1);
        assert_eq!(chunks_without.len(), 1);
        assert_eq!(
            chunks_with[0].end_line, 2,
            "end_line is the row of the last byte actually in the chunk"
        );
        assert_eq!(
            chunks_with[0].end_line, chunks_without[0].end_line,
            "a trailing newline in the source must not change end_line for identical content"
        );
    }

    #[test]
    fn should_attribute_a_zero_width_missing_node_to_a_chunk_on_either_side_of_its_boundary() {
        let source = "x = (1";
        let Some((_, tree)) = parse_with(source, "python") else {
            return;
        };
        let root = tree.root_node();

        fn find_missing<'tree>(node: tree_sitter::Node<'tree>) -> Option<tree_sitter::Node<'tree>> {
            if node.is_missing() {
                return Some(node);
            }
            let mut cursor = node.walk();
            node.children(&mut cursor).find_map(find_missing)
        }
        let Some(missing) = find_missing(root) else {
            // ~keep A grammar version might recover differently; this test only
            // ~keep proves something when it actually produces a MISSING node.
            return;
        };
        assert_eq!(missing.start_byte(), missing.end_byte(), "MISSING nodes are zero-width");
        let boundary = missing.start_byte();

        assert!(
            !is_outside_chunk(&missing, 0, boundary),
            "a chunk ending exactly at the MISSING node's position must still include it"
        );
        assert!(
            !is_outside_chunk(&missing, boundary, source.len()),
            "a chunk starting exactly at the MISSING node's position must still include it"
        );
    }
}
