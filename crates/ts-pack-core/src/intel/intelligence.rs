//! Language-neutral, per-node classification for the intelligence extractors.
//!
//! Every function here inspects a single node and returns what that node
//! contributes; none of them walks. Descent is owned by [`super::extract`],
//! which runs all enabled classifiers in one depth-bounded pass.

// ~keep `extract_intelligence` remains public even though `intel::process` drives `extract` directly.
#![allow(dead_code)]

use std::collections::BTreeSet;

use super::extract::Wanted;
use super::types::*;
use super::walk::{Descend, walk_bounded};

/// Extract all intelligence from a parsed source file.
pub fn extract_intelligence(source: &str, language: &str, tree: &tree_sitter::Tree) -> ProcessResult {
    let root = tree.root_node();
    let mut result = ProcessResult {
        language: language.to_string(),
        metrics: compute_line_metrics(source),
        ..Default::default()
    };
    super::extract::extract_all(&root, source, language, Wanted::all(), &mut result);
    result
}

pub(super) fn span_from_node(node: &tree_sitter::Node) -> Span {
    let start = node.start_position();
    let end = node.end_position();
    Span {
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_line: start.row,
        start_column: start.column,
        end_line: end.row,
        end_column: end.column,
    }
}

/// The span from `start` to `end` (exclusive) in `source`, with the trailing
/// whitespace trimmed off, so a declaration whose grammar node swallows the
/// blank lines after it still ends on its last line of content.
///
/// `anchor` is a position already known for `start`, normally the node's own
/// `start_position()`, which the parser has already computed. Passing it
/// keeps this off the source prefix. Without an anchor, every call counted
/// the newlines from byte zero, and counted them twice, so a file of many
/// short declarations cost O(n * len) to extract. Positions are measured from
/// the anchor across the span's own text instead, which is work proportional
/// to the span rather than to everything before it.
pub(super) fn span_between_from(source: &str, start: usize, end: usize, anchor: (usize, usize)) -> Span {
    let end = start + source.get(start..end).map_or(0, |text| text.trim_end().len());
    let (start_line, start_column) = anchor;
    let (end_line, end_column) = advance_position(source, start, anchor, end);
    Span {
        start_byte: start,
        end_byte: end,
        start_line,
        start_column,
        end_line,
        end_column,
    }
}

/// [`span_between_from`] over a node's own byte range, anchored on the
/// position the parser already recorded for it.
pub(super) fn span_trimmed(node: &tree_sitter::Node, source: &str) -> Span {
    let start = node.start_position();
    span_between_from(source, node.start_byte(), node.end_byte(), (start.row, start.column))
}

/// The position of `to`, measured forward from `from` at `position`.
///
/// Costs the length of the span rather than the length of the prefix before
/// it, so a walk over many declarations stays linear in the source overall.
fn advance_position(source: &str, from: usize, position: (usize, usize), to: usize) -> (usize, usize) {
    let (mut row, mut column) = position;
    let Some(text) = source.get(from..to) else {
        return position_at(source, to);
    };
    let bytes = text.as_bytes();
    match memchr::memrchr(b'\n', bytes) {
        Some(last) => {
            row += memchr::memchr_iter(b'\n', bytes).count();
            column = bytes.len() - (last + 1);
        }
        None => column += bytes.len(),
    }
    (row, column)
}

/// Zero-indexed row and byte column of `byte` in `source`.
fn position_at(source: &str, byte: usize) -> (usize, usize) {
    let before = &source.as_bytes()[..byte.min(source.len())];
    let row = memchr::memchr_iter(b'\n', before).count();
    let column = before.len()
        - before
            .iter()
            .rposition(|b| *b == b'\n')
            .map_or(0, |newline| newline + 1);
    (row, column)
}

pub(super) fn node_text<'a>(node: &tree_sitter::Node, source: &'a str) -> &'a str {
    &source[node.start_byte()..node.end_byte()]
}

fn go_type_spec_symbol_kind(node: &tree_sitter::Node) -> SymbolKind {
    let ty_kind = node
        .child_by_field_name("type")
        .map(|n| n.kind().to_string())
        .unwrap_or_default();
    match ty_kind.as_str() {
        "struct_type" => SymbolKind::Type,
        "interface_type" => SymbolKind::Interface,
        _ => SymbolKind::Type,
    }
}

/// Line-level metrics, which are derived from the source text alone.
///
/// The tree-derived fields (`node_count`, `error_count`, `max_depth`) are left
/// at zero here, and `comment_lines` at zero, because a comment cannot be
/// recognised from a line prefix without knowing the language: `#` opens a C
/// preprocessor directive, a Markdown heading and a YAML block-scalar line just
/// as often as it opens a comment. [`super::extract::extract_all`] — the only
/// traversal of the tree — fills all four in from the AST via
/// [`apply_comment_lines`].
pub(crate) fn compute_line_metrics(source: &str) -> FileMetrics {
    let mut total_lines = 0usize;
    let mut blank_lines = 0;
    for line in source.lines() {
        total_lines += 1;
        if line.trim().is_empty() {
            blank_lines += 1;
        }
    }

    FileMetrics {
        total_lines,
        code_lines: total_lines.saturating_sub(blank_lines),
        comment_lines: 0,
        blank_lines,
        total_bytes: source.len(),
        node_count: 0,
        error_count: 0,
        max_depth: 0,
    }
}

/// Node kinds that grammars in this pack use for comments.
const COMMENT_NODE_KINDS: &[&str] = &[
    "comment",
    "line_comment",
    "block_comment",
    "doc_comment",
    "documentation_comment",
];

/// Whether `node` is a comment node in its grammar.
pub(super) fn is_comment_node(node: &tree_sitter::Node) -> bool {
    COMMENT_NODE_KINDS.contains(&node.kind())
}

/// Record the source rows that a comment `node` occupies.
///
/// The comment's own first row counts only when nothing but whitespace precedes
/// it, so a trailing `x = 1; // note` stays a code line. Interior rows of a
/// block comment are wholly inside the comment and always count. The closing
/// row counts only when nothing but whitespace follows the comment's end
/// column, so `*/ fn main() {}` leaves that row as code.
pub(super) fn mark_comment_rows(node: &tree_sitter::Node, source: &str, rows: &mut BTreeSet<usize>) {
    let start = node.start_position();
    if line_prefix_is_blank(source, node.start_byte(), start.column) {
        rows.insert(start.row);
    }
    let (content_end_byte, content_end_row) = comment_content_end(node, source);
    for row in (start.row + 1)..content_end_row {
        rows.insert(row);
    }
    if content_end_row > start.row && line_suffix_is_blank(source, content_end_byte) {
        rows.insert(content_end_row);
    }
}

/// The byte offset and row of the end of a comment's own real content,
/// excluding a trailing newline the comment's token swallowed.
///
/// ~keep tree-sitter-rust's `line_comment` token matches through and
/// ~keep including the newline that ends it, so `node.end_byte()` and
/// ~keep `node.end_position()` point at the *next* line, not the comment's
/// ~keep own closing row. Treating that as the comment's closing row made
/// ~keep `mark_comment_rows` inspect the unrelated following line and, when
/// ~keep that line was blank, wrongly record it as a comment row.
fn comment_content_end(node: &tree_sitter::Node, source: &str) -> (usize, usize) {
    let end_byte = node.end_byte();
    let end_row = node.end_position().row;
    if end_byte > node.start_byte() && source.as_bytes().get(end_byte - 1) == Some(&b'\n') {
        (end_byte - 1, end_row.saturating_sub(1))
    } else {
        (end_byte, end_row)
    }
}

/// Whether only whitespace precedes the byte at `start_byte` on its own line.
///
/// ~keep `tree_sitter::Point::column` is a byte offset within the row, so
/// ~keep subtracting it from the start byte lands on the row's first byte.
fn line_prefix_is_blank(source: &str, start_byte: usize, column: usize) -> bool {
    let line_start = start_byte.saturating_sub(column);
    source
        .get(line_start..start_byte)
        .is_some_and(|prefix| prefix.trim().is_empty())
}

/// Whether only whitespace follows the byte at `end_byte` up to the end of its line.
fn line_suffix_is_blank(source: &str, end_byte: usize) -> bool {
    let line_end = source[end_byte..]
        .find('\n')
        .map_or(source.len(), |offset| end_byte + offset);
    source
        .get(end_byte..line_end)
        .is_some_and(|suffix| suffix.trim().is_empty())
}

/// Fill `metrics.comment_lines` from the rows the AST reported as comment rows,
/// and re-derive `code_lines` from them.
///
/// A blank row inside a block comment stays a blank line, as it was before the
/// count moved to the AST.
pub(super) fn apply_comment_lines(metrics: &mut FileMetrics, source: &str, comment_rows: &BTreeSet<usize>) {
    let comment_lines = source
        .lines()
        .enumerate()
        .filter(|(row, line)| comment_rows.contains(row) && !line.trim().is_empty())
        .count();
    metrics.comment_lines = comment_lines;
    metrics.code_lines = metrics.total_lines.saturating_sub(metrics.blank_lines + comment_lines);
}

/// Classify `node` as a comment, if it is one.
pub(super) fn comment_at(node: &tree_sitter::Node, source: &str) -> Option<CommentInfo> {
    if !is_comment_node(node) {
        return None;
    }
    // ~keep tree-sitter-rust nests a `doc_comment` inside the `line_comment`/`block_comment`
    // ~keep that delimits it; reporting both would emit one comment twice.
    if node.parent().is_some_and(|parent| is_comment_node(&parent)) {
        return None;
    }
    let kind = node.kind();
    let text = node_text(node, source).to_string();
    let comment_kind = if kind == "doc_comment" || kind == "documentation_comment" {
        CommentKind::Doc
    } else if kind == "block_comment" {
        CommentKind::Block
    } else if text.starts_with("///")
        || text.starts_with("//!")
        || text.starts_with("/**")
        || text.starts_with("/*!")
        || text.starts_with("##")
    {
        CommentKind::Doc
    } else {
        CommentKind::Line
    };
    Some(CommentInfo {
        text,
        kind: comment_kind,
        span: span_from_node(node),
        associated_node: node.next_named_sibling().map(|n| n.kind().to_string()),
    })
}

/// Classify `node` as a docstring, if it is one.
///
/// Only Python has a dedicated docstring form; every other language's doc
/// comments are already captured by [`comment_at`].
pub(super) fn docstring_at(node: &tree_sitter::Node, source: &str, language: &str) -> Option<DocstringInfo> {
    if language != "python" {
        return None;
    }
    let (string_node, body) = python_docstring_parts(node)?;
    let text = node_text(&string_node, source).to_string();
    let format = python_docstring_format(&text);
    Some(DocstringInfo {
        text,
        format,
        span: span_from_node(&string_node),
        associated_item: body.parent().and_then(|item| {
            item.child_by_field_name("name")
                .map(|n| node_text(&n, source).to_string())
        }),
        parsed_sections: Vec::new(),
    })
}

/// The string node of a Python docstring and the `block`/`module` holding it.
///
/// ~keep `expression_statement` is declared a *supertype* in this pack's
/// ~keep tree-sitter-python grammar, so it is hidden and a docstring appears as a
/// ~keep bare `string` directly under the block. The wrapped shape is still
/// ~keep accepted, for grammars that expose the statement node; the two forms are
/// ~keep disjoint (a wrapped string's parent is the statement, not the block), so
/// ~keep no docstring is reported twice.
///
/// ~keep A Python docstring is positional, not merely "a bare string in a
/// ~keep body": only the body's first statement counts. Without this check
/// ~keep any stray string statement later in the body — `"stray"` after real
/// ~keep code — was reported as a second, fabricated docstring.
///
/// ~keep "First statement" is not `named_child(0)`: comments are `extra`
/// ~keep nodes (see `parsers/python/src/node-types.json`), spliced into the
/// ~keep named-child list at their lexical position rather than kept out of
/// ~keep it, so a leading shebang-adjacent or license comment makes
/// ~keep `named_child(0)` the comment, not the docstring — dropping every
/// ~keep real docstring that follows a leading comment. `is_comment_node`
/// ~keep (used for the same purpose in `mark_comment_rows`) is reused here
/// ~keep rather than re-deriving "is this an extra" from `is_extra()`, so a
/// ~keep future grammar-specific extra never has to be taught to two places.
fn python_docstring_parts<'tree>(
    node: &tree_sitter::Node<'tree>,
) -> Option<(tree_sitter::Node<'tree>, tree_sitter::Node<'tree>)> {
    let parent = node.parent()?;
    if !is_python_body(&parent) {
        return None;
    }
    let mut cursor = parent.walk();
    let first_statement = parent
        .named_children(&mut cursor)
        .find(|child| !is_comment_node(child))?;
    if first_statement.id() != node.id() {
        return None;
    }
    if is_python_string(node) {
        return Some((*node, parent));
    }
    if node.kind() != "expression_statement" {
        return None;
    }
    let child = node.child(0)?;
    if !is_python_string(&child) {
        return None;
    }
    Some((child, parent))
}

fn is_python_string(node: &tree_sitter::Node) -> bool {
    matches!(node.kind(), "string" | "concatenated_string")
}

fn is_python_body(node: &tree_sitter::Node) -> bool {
    matches!(node.kind(), "block" | "module")
}

/// The [`DocstringFormat`] for a Python docstring's raw text.
///
/// ~keep Only the triple-quoted form is `PythonTripleQuote`; `def f(): 'x'`
/// ~keep is a single-quoted string statement, not the triple-quote convention,
/// ~keep so it is reported as `Other` instead of being mislabelled.
fn python_docstring_format(text: &str) -> DocstringFormat {
    let quoted = text.trim_start_matches(|c: char| c.is_ascii_alphabetic());
    if quoted.starts_with("\"\"\"") || quoted.starts_with("'''") {
        DocstringFormat::PythonTripleQuote
    } else {
        DocstringFormat::Other("python-single-quote".to_string())
    }
}

/// Classify `node` as an import for the language-neutral matcher.
///
/// Elixir directives are `call` nodes and are classified by
/// [`super::elixir::import_directive`] instead.
pub(super) fn import_at(node: &tree_sitter::Node, source: &str, language: &str) -> Option<ImportInfo> {
    let kind = node.kind();
    let is_import = match language {
        "python" => kind == "import_statement" || kind == "import_from_statement",
        "javascript" | "typescript" | "tsx" => kind == "import_statement",
        "rust" => kind == "use_declaration",
        // ~keep Go nests `import_spec` inside `import_declaration`; matching both counted
        // ~keep every import twice and the enclosing block a third time.
        "go" => kind == "import_spec",
        "java" | "kotlin" => kind == "import_declaration",
        _ => false,
    };
    if !is_import {
        return None;
    }
    let text = node_text(node, source);
    let (items, alias) = match language {
        "python" => python_import_items_and_alias(node, source),
        "javascript" | "typescript" | "tsx" => js_import_items_and_alias(node, source),
        _ => (Vec::new(), None),
    };
    Some(ImportInfo {
        source: text.to_string(),
        items,
        alias,
        is_wildcard: has_wildcard_token(node),
        span: span_from_node(node),
    })
}

/// The `(name, alias)` pair for one entry of a Python `import_statement` or
/// `import_from_statement`'s `_import_list` — a bare `dotted_name`, or an
/// `aliased_import` with `name` and `alias` fields.
fn python_import_entries(node: &tree_sitter::Node, source: &str) -> Vec<(String, Option<String>)> {
    let mut cursor = node.walk();
    node.children_by_field_name("name", &mut cursor)
        .map(|entry| {
            if entry.kind() == "aliased_import" {
                let name = entry
                    .child_by_field_name("name")
                    .map(|n| node_text(&n, source).to_string())
                    .unwrap_or_default();
                let alias = entry
                    .child_by_field_name("alias")
                    .map(|n| node_text(&n, source).to_string());
                (name, alias)
            } else {
                (node_text(&entry, source).to_string(), None)
            }
        })
        .collect()
}

/// `items`/`alias` for a Python import statement.
///
/// `items` lists every imported name (`import a, b` or `from m import a, b`).
/// `alias` is populated only for the single-name form (`import numpy as np`,
/// `from m import a as b`) — a statement that aliases several names at once
/// has no single alias to report, so `alias` stays `None` and only `items`
/// is populated for it.
fn python_import_items_and_alias(node: &tree_sitter::Node, source: &str) -> (Vec<String>, Option<String>) {
    let entries = python_import_entries(node, source);
    let alias = if let [(_, alias)] = entries.as_slice() {
        alias.clone()
    } else {
        None
    };
    let items = entries.into_iter().map(|(name, _)| name).collect();
    (items, alias)
}

/// `items`/`alias` for a JavaScript/TypeScript `import_statement`.
///
/// A namespace import (`import * as ns from 'm'`) has no named items; it
/// aliases the whole module, so it populates `alias` only. A named-imports
/// clause (`import { a, b as c } from 'm'`) populates `items` with each
/// specifier's original name; `alias` is populated only when the clause
/// names exactly one specifier, for the same reason as the Python case
/// above. A default import (`import fs from 'm'`) is not represented in
/// either field.
fn js_import_items_and_alias(node: &tree_sitter::Node, source: &str) -> (Vec<String>, Option<String>) {
    let mut cursor = node.walk();
    let Some(clause) = node
        .named_children(&mut cursor)
        .find(|child| child.kind() == "import_clause")
    else {
        return (Vec::new(), None);
    };
    let mut items = Vec::new();
    let mut alias = None;
    let mut clause_cursor = clause.walk();
    for child in clause.named_children(&mut clause_cursor) {
        match child.kind() {
            "namespace_import" => {
                if let Some(identifier) = child.named_child(0) {
                    alias = Some(node_text(&identifier, source).to_string());
                }
            }
            "named_imports" => {
                let mut spec_cursor = child.walk();
                let specifiers: Vec<_> = child
                    .named_children(&mut spec_cursor)
                    .filter(|specifier| specifier.kind() == "import_specifier")
                    .collect();
                for specifier in &specifiers {
                    if let Some(name) = specifier.child_by_field_name("name") {
                        items.push(node_text(&name, source).to_string());
                    }
                }
                if let [specifier] = specifiers.as_slice() {
                    alias = specifier
                        .child_by_field_name("alias")
                        .map(|n| node_text(&n, source).to_string());
                }
            }
            _ => {}
        }
    }
    (items, alias)
}

/// Classify `node` as an export, if the language has export statements.
pub(super) fn export_at(node: &tree_sitter::Node, source: &str, language: &str) -> Option<ExportInfo> {
    let is_export = match language {
        "javascript" | "typescript" | "tsx" => node.kind() == "export_statement",
        _ => false,
    };
    if !is_export {
        return None;
    }
    let export_kind = if has_default_keyword(node) {
        ExportKind::Default
    } else if node.child_by_field_name("source").is_some() {
        ExportKind::ReExport
    } else {
        ExportKind::Named
    };
    Some(ExportInfo {
        name: export_name(node, source),
        kind: export_kind,
        span: span_from_node(node),
    })
}

/// Whether an `export_statement` carries the `default` keyword.
///
/// ~keep tree-sitter-javascript spells `default` as an anonymous token, not a
/// ~keep field, so `child_by_field_name("default")` never matched and
/// ~keep `ExportKind::Default` was unreachable. Only direct children are scanned:
/// ~keep `export { default as x }` keeps the token inside the export specifier.
fn has_default_keyword(node: &tree_sitter::Node) -> bool {
    let mut cursor = node.walk();
    node.children(&mut cursor).any(|child| child.kind() == "default")
}

/// Resolve the name an `export_statement` exports.
///
/// The statement itself has no `name` field: the name lives on the exported
/// declaration, on the specifiers of an export clause, or — for `export * from
/// '…'` — nowhere at all, where the statement text is the only identifier
/// available.
fn export_name(node: &tree_sitter::Node, source: &str) -> String {
    if let Some(declaration) = node.child_by_field_name("declaration")
        && let Some(name) = declared_name(&declaration, source)
    {
        return name;
    }
    if let Some(value) = node.child_by_field_name("value") {
        return first_line(node_text(&value, source));
    }
    let specifiers = export_specifier_names(node, source);
    if !specifiers.is_empty() {
        return specifiers.join(", ");
    }
    first_line(node_text(node, source))
}

fn first_line(text: &str) -> String {
    text.lines().next().unwrap_or("").to_string()
}

/// The declared name of `declaration`, looking through the `variable_declarator`
/// that `const`/`let`/`var` declarations interpose.
fn declared_name(declaration: &tree_sitter::Node, source: &str) -> Option<String> {
    if let Some(name) = declaration.child_by_field_name("name") {
        return Some(node_text(&name, source).to_string());
    }
    let mut cursor = declaration.walk();
    let declarator = declaration
        .named_children(&mut cursor)
        .find(|child| child.kind() == "variable_declarator")?;
    let name = declarator.child_by_field_name("name")?;
    Some(node_text(&name, source).to_string())
}

/// The exported names of an `export { a, b }` clause, in source order.
fn export_specifier_names(node: &tree_sitter::Node, source: &str) -> Vec<String> {
    let mut cursor = node.walk();
    let Some(clause) = node
        .named_children(&mut cursor)
        .find(|child| child.kind() == "export_clause")
    else {
        return Vec::new();
    };
    let mut clause_cursor = clause.walk();
    clause
        .named_children(&mut clause_cursor)
        .filter(|child| child.kind() == "export_specifier")
        .filter_map(|child| child.child_by_field_name("name"))
        .map(|name| node_text(&name, source).to_string())
        .collect()
}

/// Classify `node` as a top-level structure item for the language-neutral
/// matcher. Elixir definitions are handled by [`super::elixir::definition`].
pub(super) fn structure_kind_at(node: &tree_sitter::Node, language: &str) -> Option<StructureKind> {
    match node.kind() {
        "function_definition"
        | "function_declaration"
        | "protocol_function_declaration"
        | "function_item"
        | "arrow_function" => Some(StructureKind::Function),
        "method_definition" | "method_declaration" => Some(StructureKind::Method),
        "method" | "singleton_method" if language == "ruby" => Some(StructureKind::Method),
        "class_declaration" if language == "swift" => swift_nominal_kind(node),
        "class_definition" | "class_declaration" | "class" => Some(StructureKind::Class),
        "struct_item" | "struct_definition" | "struct_declaration" => Some(StructureKind::Struct),
        "interface_declaration" | "interface_definition" | "protocol_declaration" => Some(StructureKind::Interface),
        "enum_item" | "enum_definition" | "enum_declaration" => Some(StructureKind::Enum),
        "module_definition" | "mod_item" | "package_header" | "package_declaration" => Some(StructureKind::Module),
        "module" if language == "ruby" => Some(StructureKind::Module),
        "internal_module" if matches!(language, "typescript" | "tsx") => Some(StructureKind::Namespace),
        "trait_item" => Some(StructureKind::Trait),
        "impl_item" => Some(StructureKind::Impl),
        _ => None,
    }
}

/// The doc comment immediately preceding `node`, if any.
///
/// Walks backward over adjacent [`CommentKind::Doc`] siblings — no blank
/// line between one comment and the next, nor between the last comment and
/// `node` itself — and joins them in source order. This captures both the
/// single-node block forms (Javadoc, JSDoc `/** ... */`) and multi-line `///`
/// blocks, which tree-sitter-rust represents as one sibling node per line.
///
/// Populated for Rust (`///`/`//!`), Java (`/** */`), and JavaScript/
/// TypeScript (`/** */`) — the languages whose [`comment_at`] already
/// classifies a comment as `Doc`. Plain `#`-style comments (Python, Ruby,
/// Elixir) are never classified `Doc`, so this is always `None` there; a
/// Python docstring is captured separately, as [`DocstringInfo`].
pub(super) fn doc_comment_at(node: &tree_sitter::Node, source: &str) -> Option<String> {
    let mut lines = Vec::new();
    let mut boundary_row = node.start_position().row;
    let mut current = node.prev_named_sibling();
    while let Some(sibling) = current {
        let Some(comment) = comment_at(&sibling, source) else {
            break;
        };
        // ~keep Gap detection uses the comment's real last content row, from
        // ~keep `comment_content_end`, not `comment.span.end_line`: a comment
        // ~keep token that swallows its own trailing newline (tree-sitter-rust's
        // ~keep does) reports `end_line` one row past its real content, which
        // ~keep would make an adjacent doc comment look like it had a blank-line
        // ~keep gap before it and a genuine gap look adjacent.
        let (_, content_end_row) = comment_content_end(&sibling, source);
        if comment.kind != CommentKind::Doc || boundary_row.saturating_sub(content_end_row) > 1 {
            break;
        }
        boundary_row = sibling.start_position().row;
        // ~keep Trimming drops that same swallowed newline from the joined text,
        // ~keep so a multi-line `///` block does not pick up a doubled blank line
        // ~keep at each join point.
        lines.push(comment.text.trim_end().to_string());
        current = sibling.prev_named_sibling();
    }
    if lines.is_empty() {
        return None;
    }
    lines.reverse();
    Some(lines.join("\n"))
}

/// The signature text of a structure item: everything from the start of
/// `node` up to the start of its `body`, trimmed of trailing whitespace.
///
/// For an item with no body (e.g. a unit struct `struct Foo;`), the whole
/// item's text is the signature. Populated for every language
/// [`structure_kind_at`] and [`super::elixir::definition`] recognise, since
/// both already resolve `body` before constructing a [`StructureItem`].
pub(super) fn structure_signature(
    node: &tree_sitter::Node,
    source: &str,
    body: Option<&tree_sitter::Node>,
) -> Option<String> {
    let end = body.map_or_else(|| node.end_byte(), tree_sitter::Node::start_byte);
    let text = source.get(node.start_byte()..end)?.trim_end();
    if text.is_empty() { None } else { Some(text.to_string()) }
}

/// Node kinds whose text is string content rather than syntax, so a literal
/// `*` inside them (e.g. a glob in an import path string) must not be
/// mistaken for a wildcard-import token.
const STRING_LIKE_NODE_KINDS: &[&str] = &[
    "string",
    "string_literal",
    "interpreted_string_literal",
    "raw_string_literal",
    "template_string",
    "concatenated_string",
];

/// Whether `node`'s subtree contains a bare wildcard glyph — a standalone `*`
/// token, or a dedicated wildcard node (`wildcard_import` in Python,
/// `use_wildcard` in Rust) — outside of any string-literal content.
///
/// Import specifiers are shallow, grammar-bounded subtrees (not
/// user-controlled nesting), so the bounded walk here never truncates.
pub(super) fn has_wildcard_token(node: &tree_sitter::Node) -> bool {
    let mut found = false;
    walk_bounded(node, |candidate, _depth| {
        if STRING_LIKE_NODE_KINDS.contains(&candidate.kind()) {
            return Descend::Skip;
        }
        if matches!(candidate.kind(), "*" | "wildcard_import" | "use_wildcard") {
            found = true;
            return Descend::Skip;
        }
        Descend::Children
    });
    found
}

/// Classify `node` as a symbol, if it is a named declaration.
pub(super) fn symbol_at(node: &tree_sitter::Node, source: &str, language: &str) -> Option<SymbolInfo> {
    let mut declaration = *node;
    let symbol_kind = match node.kind() {
        "function_definition" | "function_declaration" | "protocol_function_declaration" | "function_item" => {
            SymbolKind::Function
        }
        "class_declaration" if language == "swift" => match swift_nominal_kind(node)? {
            StructureKind::Struct => SymbolKind::Type,
            StructureKind::Enum => SymbolKind::Enum,
            StructureKind::Class => SymbolKind::Class,
            _ => return None,
        },
        "class_definition" | "class_declaration" => SymbolKind::Class,
        "type_alias_declaration" | "typealias_declaration" | "type_item" => SymbolKind::Type,
        "type_spec" => go_type_spec_symbol_kind(node),
        "interface_declaration" | "protocol_declaration" => SymbolKind::Interface,
        "enum_item" | "enum_declaration" => SymbolKind::Enum,
        "const_item" | "const_declaration" => SymbolKind::Constant,
        "variable_declarator" if matches!(language, "javascript" | "typescript" | "tsx") => {
            let parent = node.parent()?;
            if parent.kind() != "lexical_declaration"
                || parent
                    .child_by_field_name("kind")
                    .is_none_or(|kind| kind.kind() != "const")
                || !in_declaration_scope(node)
                || node
                    .child_by_field_name("value")
                    .and_then(unparenthesized)
                    .is_some_and(|value| {
                        matches!(
                            value.kind(),
                            "arrow_function" | "function_expression" | "generator_function"
                        )
                    })
            {
                return None;
            }
            SymbolKind::Constant
        }
        "pattern" if language == "swift" => {
            declaration = node.parent()?;
            if declaration.kind() != "property_declaration" || !in_declaration_scope(node) {
                return None;
            }
            let mut cursor = declaration.walk();
            if !declaration
                .named_children(&mut cursor)
                .any(|child| child.kind() == "value_binding_pattern" && node_text(&child, source) == "let")
            {
                return None;
            }
            SymbolKind::Constant
        }
        "let_declaration" | "variable_declaration" | "lexical_declaration" => SymbolKind::Variable,
        _ => return None,
    };
    let name_node = if node.kind() == "pattern" {
        node.child_by_field_name("bound_identifier")?
    } else {
        node.child_by_field_name("name")?
    };
    if node.kind() == "variable_declarator" && name_node.kind() != "identifier" {
        return None;
    }
    Some(SymbolInfo {
        name: node_text(&name_node, source).to_string(),
        kind: symbol_kind,
        span: span_from_node(&declaration),
        type_annotation: declaration
            .child_by_field_name("type")
            .map(|n| node_text(&n, source).to_string()),
        doc: doc_comment_at(&declaration, source),
    })
}

/// Parentheses preserve an initializer's identity; calls and other expressions do not.
fn unparenthesized(mut node: tree_sitter::Node<'_>) -> Option<tree_sitter::Node<'_>> {
    while node.kind() == "parenthesized_expression" {
        node = node.named_child(0)?;
    }
    Some(node)
}

/// Swift uses one node kind for classes, structs, actors, extensions and enums.
fn swift_nominal_kind(node: &tree_sitter::Node) -> Option<StructureKind> {
    match node.child_by_field_name("declaration_kind")?.kind() {
        "struct" => Some(StructureKind::Struct),
        "extension" => Some(StructureKind::Impl),
        "enum" => Some(StructureKind::Enum),
        "class" | "actor" => Some(StructureKind::Class),
        _ => None,
    }
}

/// New constant declarations belong to file/type scope, not executable bodies.
///
/// Generators are their own node kinds in the JavaScript family, with or
/// without `async`, so they are listed beside the plain function forms.
fn in_declaration_scope(node: &tree_sitter::Node) -> bool {
    let mut parent = node.parent();
    while let Some(ancestor) = parent {
        if matches!(
            ancestor.kind(),
            "function_declaration"
                | "function_expression"
                | "generator_function_declaration"
                | "generator_function"
                | "arrow_function"
                | "method_definition"
                | "function_body"
                | "lambda_literal"
                | "computed_property"
        ) {
            return false;
        }
        parent = ancestor.parent();
    }
    true
}

/// Classify `node` as a syntax diagnostic, if it is an error or missing node.
pub(super) fn diagnostic_at(node: &tree_sitter::Node, source: &str) -> Option<Diagnostic> {
    if node.is_error() {
        return Some(Diagnostic {
            message: format!("Syntax error: unexpected '{}'", node_text(node, source)),
            severity: DiagnosticSeverity::Error,
            span: span_from_node(node),
        });
    }
    if node.is_missing() {
        return Some(Diagnostic {
            message: format!("Missing expected node: {}", node.kind()),
            severity: DiagnosticSeverity::Error,
            span: span_from_node(node),
        });
    }
    None
}

/// Resolve the name of a structure node using a fallback chain.
///
/// Tries `"name"` field first (covers Python, Rust, Java classes), then finds
/// the first named child with kind `"type_identifier"` (Kotlin classes), then
/// `"identifier"` (Kotlin packages), then `"scoped_identifier"` (Java packages).
/// Returns `None` if no non-empty text is found via any strategy.
pub(super) fn resolve_structure_name(node: &tree_sitter::Node, source: &str) -> Option<String> {
    if node.kind() == "arrow_function" {
        let mut value = *node;
        let mut parent = value.parent()?;
        while parent.kind() == "parenthesized_expression" {
            value = parent;
            parent = value.parent()?;
        }
        if parent.kind() != "variable_declarator" || parent.child_by_field_name("value")?.id() != value.id() {
            return None;
        }
        let name = parent.child_by_field_name("name")?;
        return (name.kind() == "identifier").then(|| node_text(&name, source).to_string());
    }
    if let Some(n) = node.child_by_field_name("name") {
        let text = node_text(&n, source);
        if !text.is_empty() {
            return Some(text.to_string());
        }
    }
    for target_kind in &["type_identifier", "identifier", "scoped_identifier"] {
        let mut cursor = node.walk();
        for child in node.named_children(&mut cursor) {
            if child.kind() == *target_kind {
                let text = node_text(&child, source);
                if !text.is_empty() {
                    return Some(text.to_string());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // ~keep A missing grammar reports `SKIPPED` on stderr; see `intel::test_support`.
    fn parse_or_skip(source: &str, lang_name: &str) -> Option<tree_sitter::Tree> {
        crate::intel::test_support::parse_or_skip(source, lang_name)
    }

    #[test]
    fn test_extract_python_function() {
        let source = "def foo():\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.language, "python");
        assert!(!intel.structure.is_empty(), "should find at least one structure item");
        let func = &intel.structure[0];
        assert_eq!(func.kind, StructureKind::Function);
        assert_eq!(func.name.as_deref(), Some("foo"));
    }

    #[test]
    fn test_extract_python_class() {
        let source = "class MyClass:\n    def method(self):\n        pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
        assert!(class.is_some(), "should find a class");
        let class = class.unwrap();
        assert_eq!(class.name.as_deref(), Some("MyClass"));
        assert!(!class.children.is_empty(), "class should have child methods");
        assert_eq!(class.children[0].kind, StructureKind::Function);
        assert_eq!(class.children[0].name.as_deref(), Some("method"));
    }

    #[test]
    fn test_extract_ruby_module_class_and_methods() {
        let source = "module Outer\n  class Widget\n    def call\n      true\n    end\n\n    def self.build\n      new\n    end\n  end\nend\n";
        let Some(tree) = parse_or_skip(source, "ruby") else {
            return;
        };
        let intel = extract_intelligence(source, "ruby", &tree);

        let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
        assert!(module.is_some(), "should find a Ruby module entry");
        let module = module.unwrap();
        assert_eq!(module.name.as_deref(), Some("Outer"));

        let class = module.children.iter().find(|s| s.kind == StructureKind::Class);
        assert!(class.is_some(), "should find a Ruby class inside the module");
        let class = class.unwrap();
        assert_eq!(class.name.as_deref(), Some("Widget"));

        let method_names = class
            .children
            .iter()
            .filter(|s| s.kind == StructureKind::Method)
            .filter_map(|s| s.name.as_deref())
            .collect::<Vec<_>>();
        assert!(method_names.contains(&"call"), "should find an instance method");
        assert!(method_names.contains(&"build"), "should find a singleton method");
    }

    #[test]
    fn test_extract_rust_function() {
        let source = "fn main() {\n    let x = 5;\n}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert!(!intel.structure.is_empty(), "should find at least one structure item");
        let func = &intel.structure[0];
        assert_eq!(func.kind, StructureKind::Function);
        assert_eq!(func.name.as_deref(), Some("main"));
    }

    #[test]
    fn test_extract_python_imports() {
        let source = "import os\nfrom sys import path\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.imports.len(), 2, "should find 2 imports");
        assert!(intel.imports[0].source.contains("import os"));
        assert!(intel.imports[1].source.contains("from sys import path"));
    }

    #[test]
    fn test_extract_rust_imports() {
        let source = "use std::collections::HashMap;\nuse std::io;\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(intel.imports.len(), 2, "should find 2 use declarations");
    }

    #[test]
    fn test_extract_comments() {
        let source = "// This is a comment\nfn main() {}\n// Another comment\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert!(intel.comments.len() >= 2, "should find at least 2 comments");
        assert!(intel.comments[0].text.contains("This is a comment"));
    }

    #[test]
    fn test_extract_doc_comments() {
        let source = "/// Documentation comment\nfn documented() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        let doc_comments: Vec<_> = intel.comments.iter().filter(|c| c.kind == CommentKind::Doc).collect();
        assert!(!doc_comments.is_empty(), "should find doc comments");
    }

    #[test]
    fn test_metrics_counts() {
        let source = "fn foo() {}\n\n// comment\nfn bar() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert!(intel.metrics.total_lines >= 4, "should have at least 4 lines");
        assert!(intel.metrics.blank_lines >= 1, "should have at least 1 blank line");
        assert!(intel.metrics.comment_lines >= 1, "should have at least 1 comment line");
        assert!(intel.metrics.code_lines >= 2, "should have at least 2 code lines");
        assert!(intel.metrics.node_count > 0, "should have nodes");
        assert_eq!(intel.metrics.error_count, 0, "valid code should have 0 errors");
        assert!(intel.metrics.max_depth > 0, "tree should have depth > 0");
        assert_eq!(intel.metrics.total_bytes, source.len());
    }

    #[test]
    fn should_count_each_go_import_once() {
        let source = "package main\n\nimport (\n\t\"fmt\"\n\t\"os\"\n)\n";
        let Some(tree) = parse_or_skip(source, "go") else {
            return;
        };
        let intel = extract_intelligence(source, "go", &tree);

        let sources: Vec<&str> = intel.imports.iter().map(|i| i.source.as_str()).collect();
        assert_eq!(
            sources,
            vec!["\"fmt\"", "\"os\""],
            "an import block must yield one entry per spec, not one per spec plus the block"
        );
    }

    #[test]
    fn should_count_a_single_go_import_once() {
        let source = "package main\n\nimport \"fmt\"\n";
        let Some(tree) = parse_or_skip(source, "go") else {
            return;
        };
        let intel = extract_intelligence(source, "go", &tree);

        let sources: Vec<&str> = intel.imports.iter().map(|i| i.source.as_str()).collect();
        assert_eq!(sources, vec!["\"fmt\""]);
    }

    #[test]
    fn should_populate_items_and_alias_for_a_single_name_python_import() {
        let source = "import numpy as np\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert_eq!(intel.imports[0].items, vec!["numpy".to_string()]);
        assert_eq!(intel.imports[0].alias.as_deref(), Some("np"));
    }

    #[test]
    fn should_populate_items_without_an_alias_for_a_python_from_import() {
        let source = "from sys import path\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert_eq!(intel.imports[0].items, vec!["path".to_string()]);
        assert_eq!(intel.imports[0].alias, None);
    }

    #[test]
    fn should_leave_alias_none_for_a_python_import_naming_several_modules() {
        let source = "import os, sys as s\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert_eq!(
            intel.imports[0].items,
            vec!["os".to_string(), "sys".to_string()],
            "items lists every entry's base name, aliased or not"
        );
        assert_eq!(
            intel.imports[0].alias, None,
            "a statement aliasing one of several names has no single alias to report"
        );
    }

    #[test]
    fn should_populate_alias_for_a_javascript_namespace_import() {
        let source = "import * as ns from 'mod';\n";
        let Some(tree) = parse_or_skip(source, "javascript") else {
            return;
        };
        let intel = extract_intelligence(source, "javascript", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert_eq!(intel.imports[0].items, Vec::<String>::new());
        assert_eq!(intel.imports[0].alias.as_deref(), Some("ns"));
    }

    #[test]
    fn should_populate_items_for_javascript_named_imports() {
        let source = "import { a, b as c } from 'mod';\n";
        let Some(tree) = parse_or_skip(source, "javascript") else {
            return;
        };
        let intel = extract_intelligence(source, "javascript", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert_eq!(intel.imports[0].items, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(
            intel.imports[0].alias, None,
            "more than one specifier has no single alias to report"
        );
    }

    #[test]
    fn should_leave_items_and_alias_empty_for_a_javascript_default_import() {
        let source = "import fs from 'node:fs';\n";
        let Some(tree) = parse_or_skip(source, "javascript") else {
            return;
        };
        let intel = extract_intelligence(source, "javascript", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert_eq!(intel.imports[0].items, Vec::<String>::new());
        assert_eq!(intel.imports[0].alias, None);
    }

    #[test]
    fn should_flag_a_python_wildcard_import() {
        let source = "from pkg import *\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert!(intel.imports[0].is_wildcard);
    }

    #[test]
    fn should_flag_a_rust_glob_use_as_wildcard() {
        let source = "use std::collections::*;\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert!(intel.imports[0].is_wildcard);
    }

    #[test]
    fn should_not_flag_a_glob_character_inside_an_import_path_string_as_wildcard() {
        let source = "import a from './glob*.js';\n";
        let Some(tree) = parse_or_skip(source, "javascript") else {
            return;
        };
        let intel = extract_intelligence(source, "javascript", &tree);

        assert_eq!(intel.imports.len(), 1);
        assert!(
            !intel.imports[0].is_wildcard,
            "a `*` inside the source path string is not a wildcard-import token"
        );
    }

    #[test]
    fn should_populate_a_rust_function_signature_up_to_its_body() {
        let source = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(intel.structure.len(), 1);
        assert_eq!(
            intel.structure[0].signature.as_deref(),
            Some("fn add(a: i32, b: i32) -> i32")
        );
    }

    #[test]
    fn should_populate_a_doc_comment_for_a_documented_rust_function() {
        let source = "/// Adds two numbers.\nfn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(intel.structure.len(), 1);
        assert_eq!(intel.structure[0].doc_comment.as_deref(), Some("/// Adds two numbers."));
    }

    #[test]
    fn should_join_a_multiline_rust_doc_comment_in_source_order() {
        let source = "/// Line one.\n/// Line two.\nfn add() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(intel.structure.len(), 1);
        assert_eq!(
            intel.structure[0].doc_comment.as_deref(),
            Some("/// Line one.\n/// Line two.")
        );
    }

    #[test]
    fn should_leave_doc_comment_none_when_a_blank_line_separates_the_comment() {
        let source = "/// Unrelated doc comment.\n\nfn add() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(intel.structure.len(), 1);
        assert_eq!(
            intel.structure[0].doc_comment, None,
            "a blank line breaks the association between a doc comment and the item after it"
        );
    }

    #[test]
    fn should_leave_decorators_reserved_and_empty_for_a_python_decorated_function() {
        let source = "@staticmethod\ndef f():\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert!(!intel.structure.is_empty(), "should still find the function item");
        assert!(
            intel.structure.iter().all(|item| item.decorators.is_empty()),
            "decorators is reserved and must stay empty until implemented"
        );
    }

    #[test]
    fn should_leave_parsed_sections_reserved_and_empty_for_a_python_docstring() {
        let source = "def f(x):\n    \"\"\"Do a thing.\n\n    Args:\n        x: a value.\n    \"\"\"\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.docstrings.len(), 1);
        assert!(
            intel.docstrings[0].parsed_sections.is_empty(),
            "parsed_sections is reserved and must stay empty until implemented"
        );
    }

    #[test]
    fn should_resolve_javascript_export_kinds_and_names() {
        let source = "const helper = 2;\nexport const alpha = 1;\nexport default function beta() {}\nexport function gamma() {}\nexport { helper };\nexport * from './other.js';\n";
        let Some(tree) = parse_or_skip(source, "javascript") else {
            return;
        };
        let intel = extract_intelligence(source, "javascript", &tree);

        assert_eq!(intel.exports.len(), 5, "every export statement must be reported once");
        let names: Vec<&str> = intel.exports.iter().take(4).map(|e| e.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["alpha", "beta", "gamma", "helper"],
            "the name must be the exported binding, not the whole statement"
        );
        let kinds: Vec<&ExportKind> = intel.exports.iter().map(|e| &e.kind).collect();
        assert_eq!(
            kinds,
            vec![
                &ExportKind::Named,
                &ExportKind::Default,
                &ExportKind::Named,
                &ExportKind::Named,
                &ExportKind::ReExport,
            ]
        );
    }

    #[test]
    fn should_not_count_string_content_as_comment_lines() {
        let source = "text = \"\"\"\n# not a comment\n* not a comment\n// not a comment\n\"\"\"\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.metrics.total_lines, 5);
        assert_eq!(
            intel.metrics.comment_lines, 0,
            "a line prefix inside a string literal is not a comment"
        );
        assert_eq!(intel.metrics.code_lines, 5);
    }

    #[test]
    fn should_not_count_a_yaml_block_scalar_line_as_a_comment() {
        let source = "script: |\n  # not a comment\n  echo hi\n";
        let Some(tree) = parse_or_skip(source, "yaml") else {
            return;
        };
        let intel = extract_intelligence(source, "yaml", &tree);

        assert_eq!(intel.metrics.total_lines, 3);
        assert_eq!(
            intel.metrics.comment_lines, 0,
            "a `#` inside a block scalar is data, not a comment"
        );
        assert_eq!(intel.metrics.code_lines, 3);
    }

    #[test]
    fn should_count_a_trailing_comment_line_as_code() {
        let source = "# leading\nx = 1  # trailing\n\n# another\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.metrics.total_lines, 4);
        assert_eq!(intel.metrics.blank_lines, 1);
        assert_eq!(intel.metrics.comment_lines, 2, "only the two whole-line comments count");
        assert_eq!(intel.metrics.code_lines, 1);
    }

    #[test]
    fn should_count_every_line_of_a_multi_line_block_comment() {
        let source = "/* first\n   second\n   third */\nfn main() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(intel.metrics.total_lines, 4);
        assert_eq!(intel.metrics.comment_lines, 3, "a block comment covers all of its rows");
        assert_eq!(intel.metrics.code_lines, 1);
    }

    #[test]
    fn should_report_a_rust_doc_comment_once() {
        let source = "/// Documented\nfn documented() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(
            intel.comments.len(),
            1,
            "the grammar nests `doc_comment` inside `line_comment`; only the outer node is a comment"
        );
        assert_eq!(intel.comments[0].kind, CommentKind::Doc);
        // ~keep `text` is the raw node text. Whether a line comment's token includes its
        // ~keep trailing newline is grammar-dependent (tree-sitter-rust's does), so the
        // ~keep assertion trims rather than baking one grammar's token boundary in.
        assert_eq!(intel.comments[0].text.trim_end(), "/// Documented");
    }

    #[test]
    fn test_extract_symbols() {
        let source = "fn alpha() {}\nfn beta() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        let func_symbols: Vec<_> = intel
            .symbols
            .iter()
            .filter(|s| s.kind == SymbolKind::Function)
            .collect();
        assert!(func_symbols.len() >= 2, "should find at least 2 function symbols");
        let names: Vec<_> = func_symbols.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"alpha"));
        assert!(names.contains(&"beta"));
    }

    #[test]
    fn test_extract_go_type_declarations_as_symbols() {
        let source = "type User struct{}\ntype Service interface{}\ntype ID string\n";
        let Some(tree) = parse_or_skip(source, "go") else {
            return;
        };
        let intel = extract_intelligence(source, "go", &tree);

        assert!(
            intel
                .symbols
                .iter()
                .any(|s| { s.kind == SymbolKind::Type && s.name == "User" })
        );
        assert!(
            intel
                .symbols
                .iter()
                .any(|s| { s.kind == SymbolKind::Interface && s.name == "Service" })
        );
        assert!(
            intel
                .symbols
                .iter()
                .any(|s| { s.kind == SymbolKind::Type && s.name == "ID" })
        );
    }

    #[test]
    fn should_populate_symbol_doc_from_an_immediately_preceding_doc_comment() {
        let source = "/// Documented function.\nfn documented() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        let symbol = intel
            .symbols
            .iter()
            .find(|s| s.name == "documented")
            .expect("the documented function should be extracted as a symbol");
        assert_eq!(
            symbol.doc.as_deref(),
            Some("/// Documented function."),
            "SymbolInfo.doc must reuse doc_comment_at rather than stay hard-coded None"
        );
    }

    #[test]
    fn should_leave_symbol_doc_none_for_a_symbol_with_no_preceding_doc_comment() {
        let source = "fn undocumented() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        let symbol = intel
            .symbols
            .iter()
            .find(|s| s.name == "undocumented")
            .expect("the undocumented function should be extracted as a symbol");
        assert_eq!(symbol.doc, None);
    }

    #[test]
    fn test_error_nodes_detected() {
        let source = "def :\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert!(
            intel.metrics.error_count > 0,
            "invalid syntax should produce error nodes"
        );
        assert!(!intel.diagnostics.is_empty(), "should have diagnostics for errors");
        assert!(
            intel
                .diagnostics
                .iter()
                .any(|d| d.severity == DiagnosticSeverity::Error)
        );
    }

    #[test]
    fn test_valid_code_no_diagnostics() {
        let source = "def foo():\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.metrics.error_count, 0);
        assert!(intel.diagnostics.is_empty(), "valid code should have no diagnostics");
    }

    #[test]
    fn should_extract_a_python_function_docstring_with_its_text() {
        let source = "def greet():\n    \"\"\"Say hello.\"\"\"\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.docstrings.len(), 1, "should find exactly one python docstring");
        let docstring = &intel.docstrings[0];
        assert_eq!(docstring.text, "\"\"\"Say hello.\"\"\"");
        assert_eq!(docstring.format, DocstringFormat::PythonTripleQuote);
        assert_eq!(docstring.associated_item.as_deref(), Some("greet"));
        assert_eq!(docstring.span.start_line, 1);
    }

    #[test]
    fn should_extract_module_and_class_docstrings_without_duplicating_them() {
        let source = "\"\"\"Module doc.\"\"\"\n\n\nclass Widget:\n    \"\"\"Widget doc.\"\"\"\n\n    def render(self):\n        \"\"\"Render doc.\"\"\"\n        return 1\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        let texts: Vec<&str> = intel.docstrings.iter().map(|d| d.text.as_str()).collect();
        assert_eq!(
            texts,
            vec![
                "\"\"\"Module doc.\"\"\"",
                "\"\"\"Widget doc.\"\"\"",
                "\"\"\"Render doc.\"\"\""
            ],
            "one docstring per module, class and function, in source order"
        );
        let owners: Vec<Option<&str>> = intel.docstrings.iter().map(|d| d.associated_item.as_deref()).collect();
        assert_eq!(owners, vec![None, Some("Widget"), Some("render")]);
    }

    #[test]
    fn should_not_report_a_nested_string_expression_as_a_docstring() {
        let source = "def greet():\n    if True:\n        pass\n    return \"not a docstring\"\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert!(
            intel.docstrings.is_empty(),
            "a returned string is not a docstring; got {:?}",
            intel.docstrings
        );
    }

    #[test]
    fn should_report_a_module_docstring_after_a_shebang_line() {
        let source = "#!/usr/bin/env python3\n\"\"\"Module doc.\"\"\"\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(
            intel.docstrings.len(),
            1,
            "a leading shebang must not hide the module docstring; got {:?}",
            intel.docstrings
        );
        assert_eq!(intel.docstrings[0].text, "\"\"\"Module doc.\"\"\"");
    }

    #[test]
    fn should_report_a_module_docstring_after_leading_license_comments() {
        let source = "# Copyright 2026 Example Corp.\n# SPDX-License-Identifier: Apache-2.0\n\"\"\"Module doc.\"\"\"\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(
            intel.docstrings.len(),
            1,
            "leading license comments must not hide the module docstring; got {:?}",
            intel.docstrings
        );
        assert_eq!(intel.docstrings[0].text, "\"\"\"Module doc.\"\"\"");
    }

    #[test]
    // ~keep This case does NOT exercise the leading-comment bug, and is kept as a shape
    // ~keep guard rather than a regression test. Measured tree for this exact source:
    // ~keep (function_definition ... (comment) body: (block (string ...) (pass_statement)))
    // ~keep — the comment attaches to `function_definition`, NOT to the `block`, so the
    // ~keep block's first named child was already the docstring and the old
    // ~keep `named_child(0)` check passed here too. Verified by dumping the s-expression.
    // ~keep Only the MODULE-level tests above discriminate, because there a comment is a
    // ~keep direct child of `module`. This test pins that attachment shape: if a grammar
    // ~keep update ever moves the comment inside the block, it starts failing and tells us.
    fn should_report_a_function_docstring_after_a_leading_comment_in_its_body() {
        let source = "def greet():\n    # set up\n    \"\"\"Doc.\"\"\"\n    pass\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(
            intel.docstrings.len(),
            1,
            "a leading comment inside the body must not hide the function docstring; got {:?}",
            intel.docstrings
        );
        assert_eq!(intel.docstrings[0].text, "\"\"\"Doc.\"\"\"");
        assert_eq!(intel.docstrings[0].associated_item.as_deref(), Some("greet"));
    }

    #[test]
    fn should_still_report_exactly_one_docstring_when_a_stray_string_follows_it() {
        let source = "def greet():\n    \"\"\"Real doc.\"\"\"\n    name = \"cafe\"\n    \"stray string\"\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(
            intel.docstrings.len(),
            1,
            "the skip-extras fix must not resurrect the fabricated stray docstring; got {:?}",
            intel.docstrings
        );
        assert_eq!(intel.docstrings[0].text, "\"\"\"Real doc.\"\"\"");
        assert_eq!(intel.docstrings[0].associated_item.as_deref(), Some("greet"));
    }

    #[test]
    fn should_not_mark_a_blank_line_after_a_newline_swallowing_comment_as_a_comment_row() {
        let source = "// comment\n\nfn main() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let root = tree.root_node();
        let comment = root.named_child(0).expect("source must start with a comment node");
        assert_eq!(
            comment.kind(),
            "line_comment",
            "test setup must target the comment node"
        );

        let mut rows = BTreeSet::new();
        mark_comment_rows(&comment, source, &mut rows);

        assert_eq!(
            rows,
            BTreeSet::from([0]),
            "the blank row after the comment must not be reported as a comment row"
        );
    }

    #[test]
    fn test_intelligence_language_field() {
        let source = "x = 1";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);
        assert_eq!(intel.language, "python");
    }

    #[test]
    fn collect_structure_kotlin_package_and_class() {
        let source = "package foo.bar\n\nclass Widget {}\n";
        let Some(tree) = parse_or_skip(source, "kotlin") else {
            return;
        };
        let intel = extract_intelligence(source, "kotlin", &tree);

        let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
        assert!(module.is_some(), "should find a Module entry for the package header");
        assert_eq!(module.unwrap().name.as_deref(), Some("foo.bar"));

        let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
        assert!(class.is_some(), "should find a Class entry");
        assert_eq!(class.unwrap().name.as_deref(), Some("Widget"));
    }

    #[test]
    fn collect_structure_java_package_and_class() {
        let source = "package com.example;\n\npublic class Widget {}\n";
        let Some(tree) = parse_or_skip(source, "java") else {
            return;
        };
        let intel = extract_intelligence(source, "java", &tree);

        let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
        assert!(
            module.is_some(),
            "should find a Module entry for the package declaration"
        );
        assert_eq!(module.unwrap().name.as_deref(), Some("com.example"));

        let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
        assert!(class.is_some(), "should find a Class entry");
        assert_eq!(class.unwrap().name.as_deref(), Some("Widget"));
    }

    #[test]
    fn should_report_exactly_one_docstring_when_a_stray_string_follows_it() {
        let source = "def greet():\n    \"\"\"Real doc.\"\"\"\n    name = \"cafe\"\n    \"stray string\"\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(
            intel.docstrings.len(),
            1,
            "only the first statement is a docstring; got {:?}",
            intel.docstrings
        );
        assert_eq!(intel.docstrings[0].text, "\"\"\"Real doc.\"\"\"");
        assert_eq!(intel.docstrings[0].associated_item.as_deref(), Some("greet"));
    }

    #[test]
    fn should_use_a_non_triple_quote_format_when_docstring_is_single_quoted() {
        let source = "def f():\n    'x'\n";
        let Some(tree) = parse_or_skip(source, "python") else {
            return;
        };
        let intel = extract_intelligence(source, "python", &tree);

        assert_eq!(intel.docstrings.len(), 1);
        assert_eq!(
            intel.docstrings[0].format,
            DocstringFormat::Other("python-single-quote".to_string()),
            "a single-quoted string statement is not the triple-quote convention"
        );
    }

    #[test]
    fn should_count_code_lines_when_a_block_comment_closes_mid_line() {
        let source = "/* a\n*/ fn main() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(
            intel.metrics.comment_lines, 1,
            "only the opening row is wholly a comment; the closing row carries code"
        );
        assert_eq!(
            intel.metrics.code_lines, 1,
            "the line containing `fn main` must not be entirely swallowed by the comment"
        );
    }

    #[test]
    fn should_count_closing_row_as_comment_when_blank_after_close() {
        let source = "/* a\n*/   \nfn main() {}\n";
        let Some(tree) = parse_or_skip(source, "rust") else {
            return;
        };
        let intel = extract_intelligence(source, "rust", &tree);

        assert_eq!(
            intel.metrics.comment_lines, 2,
            "the closing row has only whitespace after `*/` so it is still a comment row"
        );
        assert_eq!(intel.metrics.code_lines, 1);
    }
}
