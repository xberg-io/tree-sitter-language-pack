//! Reference implementation of the pre-refactor extractors, kept for tests only.
//!
//! These are the original per-extractor recursive walks, one per extractor,
//! transcribed unchanged apart from calling the Elixir classifiers that
//! [`super::elixir`] now exposes. They exist so
//! `equivalence::should_match_the_reference_walks_on_normal_input` can prove
//! that the single bounded traversal in [`super::extract`] produces
//! byte-identical output to the walks it replaced.
//!
//! They are deliberately recursive: that is the defect [`super::walk`] fixes,
//! and the oracle only ever runs on small, shallow test inputs.

use std::collections::BTreeSet;

use super::intelligence::{
    apply_comment_lines, comment_at, diagnostic_at, doc_comment_at, docstring_at, export_at, import_at,
    is_comment_node, mark_comment_rows, resolve_structure_name, span_from_node, structure_kind_at, structure_signature,
    symbol_at,
};
use super::types::*;

/// The original `extract_intelligence`: one full walk per extractor.
pub(super) fn extract_intelligence(source: &str, language: &str, tree: &tree_sitter::Tree) -> ProcessResult {
    let root = tree.root_node();

    let mut node_count = 0;
    let mut error_count = 0;
    let mut max_depth = 0;
    count_nodes(&root, 0, &mut node_count, &mut error_count, &mut max_depth);
    let mut comment_rows = BTreeSet::new();
    collect_comment_rows(&root, source, &mut comment_rows);
    let mut metrics = super::intelligence::compute_line_metrics(source);
    metrics.node_count = node_count;
    metrics.error_count = error_count;
    metrics.max_depth = max_depth;
    apply_comment_lines(&mut metrics, source, &comment_rows);

    let mut structure = Vec::new();
    collect_structure(&root, source, language, &mut structure);
    let mut imports = Vec::new();
    collect_imports(&root, source, language, &mut imports);
    let mut exports = Vec::new();
    collect_exports(&root, source, language, &mut exports);
    let mut comments = Vec::new();
    collect_comments(&root, source, &mut comments);
    let mut docstrings = Vec::new();
    collect_docstrings(&root, source, language, &mut docstrings);
    let mut symbols = Vec::new();
    collect_symbols(&root, source, language, &mut symbols);
    let mut diagnostics = Vec::new();
    collect_diagnostics(&root, source, &mut diagnostics);

    ProcessResult {
        language: language.to_string(),
        metrics,
        structure,
        imports,
        exports,
        comments,
        docstrings,
        symbols,
        diagnostics,
        chunks: Vec::new(),
        data: None,
    }
}

fn count_nodes(node: &tree_sitter::Node, depth: usize, count: &mut usize, errors: &mut usize, max_depth: &mut usize) {
    *count += 1;
    if depth > *max_depth {
        *max_depth = depth;
    }
    if node.is_error() || node.is_missing() {
        *errors += 1;
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        count_nodes(&child, depth + 1, count, errors, max_depth);
    }
}

fn collect_comment_rows(node: &tree_sitter::Node, source: &str, rows: &mut BTreeSet<usize>) {
    if is_comment_node(node) {
        mark_comment_rows(node, source, rows);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_comment_rows(&child, source, rows);
    }
}

fn collect_comments(node: &tree_sitter::Node, source: &str, comments: &mut Vec<CommentInfo>) {
    if let Some(comment) = comment_at(node, source) {
        comments.push(comment);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_comments(&child, source, comments);
    }
}

fn collect_docstrings(node: &tree_sitter::Node, source: &str, language: &str, docstrings: &mut Vec<DocstringInfo>) {
    if let Some(docstring) = docstring_at(node, source, language) {
        docstrings.push(docstring);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_docstrings(&child, source, language, docstrings);
    }
}

fn collect_imports(node: &tree_sitter::Node, source: &str, language: &str, imports: &mut Vec<ImportInfo>) {
    if language == "elixir" && node.kind() == "call" {
        if super::elixir::is_quote_call(node, source) {
            return;
        }
        if let Some(import) = super::elixir::import_directive(node, source) {
            imports.push(import);
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_imports(&child, source, language, imports);
        }
        return;
    }
    if let Some(import) = import_at(node, source, language) {
        imports.push(import);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_imports(&child, source, language, imports);
    }
}

fn collect_exports(node: &tree_sitter::Node, source: &str, language: &str, exports: &mut Vec<ExportInfo>) {
    if let Some(export) = export_at(node, source, language) {
        exports.push(export);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_exports(&child, source, language, exports);
    }
}

fn collect_structure(node: &tree_sitter::Node, source: &str, language: &str, items: &mut Vec<StructureItem>) {
    if language == "elixir" && collect_structure_call(node, source, language, items) {
        return;
    }

    if let Some(kind) = structure_kind_at(node, language) {
        let name = resolve_structure_name(node, source);
        let body = node.child_by_field_name("body");
        let body_span = body.as_ref().map(span_from_node);
        let mut children = Vec::new();
        if let Some(body) = body {
            collect_structure(&body, source, language, &mut children);
        }
        items.push(StructureItem {
            kind,
            name,
            visibility: None,
            span: span_from_node(node),
            children,
            decorators: Vec::new(),
            doc_comment: doc_comment_at(node, source),
            signature: structure_signature(node, source, body.as_ref()),
            body_span,
        });
    } else {
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            collect_structure(&child, source, language, items);
        }
    }
}

fn collect_structure_call(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    items: &mut Vec<StructureItem>,
) -> bool {
    if super::elixir::is_quote_call(node, source) {
        return true;
    }
    let Some(definition) = super::elixir::definition(node, source) else {
        return false;
    };
    let body_span = definition.body.as_ref().map(span_from_node);
    let mut children = Vec::new();
    if let Some(body) = definition.body {
        collect_structure(&body, source, language, &mut children);
    }
    let signature = structure_signature(node, source, definition.body.as_ref());
    items.push(StructureItem {
        kind: definition.kind,
        name: definition.name,
        visibility: definition.visibility,
        span: span_from_node(node),
        children,
        decorators: Vec::new(),
        doc_comment: doc_comment_at(node, source),
        signature,
        body_span,
    });
    true
}

fn collect_symbols(node: &tree_sitter::Node, source: &str, language: &str, symbols: &mut Vec<SymbolInfo>) {
    if let Some(symbol) = symbol_at(node, source, language) {
        symbols.push(symbol);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_symbols(&child, source, language, symbols);
    }
}

fn collect_diagnostics(node: &tree_sitter::Node, source: &str, diagnostics: &mut Vec<Diagnostic>) {
    if let Some(diagnostic) = diagnostic_at(node, source) {
        diagnostics.push(diagnostic);
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_diagnostics(&child, source, diagnostics);
    }
}

/// The original per-chunk metadata walk, recursive and unbounded.
///
/// ~keep This reproduces the pre-fix boundary test too (a half-open range
/// ~keep with no zero-width exception), not just the pre-fix recursion: a
/// ~keep zero-width `MISSING` node sitting exactly on `chunk_start`/`chunk_end`
/// ~keep is a second, independent defect from the one this module exists to
/// ~keep reproduce, so it is covered by its own dedicated regression test
/// ~keep (`chunking::tests::should_attribute_a_zero_width_missing_node_to_a_chunk_on_either_side_of_its_boundary`)
/// ~keep rather than through this equivalence oracle.
pub(super) fn collect_chunk_metadata(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    chunk_start: usize,
    chunk_end: usize,
    collector: &mut super::chunking::MetadataCollector<'_>,
) {
    if node.end_byte() <= chunk_start || node.start_byte() >= chunk_end {
        return;
    }
    super::chunking::record_chunk_node(node, source, language, chunk_start, chunk_end, collector);
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_chunk_metadata(&child, source, language, chunk_start, chunk_end, collector);
    }
}
