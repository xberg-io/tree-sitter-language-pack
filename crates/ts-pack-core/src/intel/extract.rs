//! One bounded traversal that feeds every enabled extractor.
//!
//! Each extractor used to own a recursive walk of the whole tree, so
//! `process(all)` walked it eight times and paid the traversal cost for
//! extractors that found nothing. Traversal — not the predicates — dominates:
//! seven walks running one predicate each measured 9.35x the cost of one walk
//! running all seven. This module runs the predicates per node inside a single
//! [`super::walk::walk_bounded`] pass, which is also what makes the whole
//! extraction depth-bounded rather than natively recursive.
//!
//! Output ordering is identical to the old per-extractor walks: the traversal
//! is pre-order, and the two extractors with non-uniform shapes — structure
//! (which nests children and descends only into a matched node's body) and
//! imports (which prunes Elixir `quote` bodies) — reproduce that shape with
//! depth-keyed scopes instead of recursion.

use std::collections::BTreeSet;

use tree_sitter::Node;

use super::elixir;
use super::intelligence::{
    apply_comment_lines, comment_at, diagnostic_at, doc_comment_at, docstring_at, export_at, import_at,
    is_comment_node, mark_comment_rows, resolve_structure_name, span_from_node, structure_kind_at, structure_signature,
    symbol_at,
};
use super::types::*;
use super::walk::{Descend, walk_bounded, warn_if_truncated};

/// Which extractors the traversal should run. Node counting, error counting and
/// depth measurement are unconditional: they feed `FileMetrics`, which every
/// `process()` call returns.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Wanted {
    pub structure: bool,
    pub imports: bool,
    pub exports: bool,
    pub comments: bool,
    pub docstrings: bool,
    pub symbols: bool,
    pub diagnostics: bool,
}

impl Wanted {
    /// Every extractor enabled.
    // ~keep Only `intelligence::extract_intelligence` needs this, and `intel` is a crate-private module.
    #[allow(dead_code)]
    pub(crate) fn all() -> Self {
        Self {
            structure: true,
            imports: true,
            exports: true,
            comments: true,
            docstrings: true,
            symbols: true,
            diagnostics: true,
        }
    }
}

/// Walk `root` once, running every extractor enabled by `wanted`, and write the
/// results into `out`.
///
/// `out.metrics` is expected to already carry the line-level metrics from
/// [`super::intelligence::compute_line_metrics`]; the tree-derived fields —
/// including `comment_lines`, which needs the grammar's comment nodes rather
/// than a language-blind line prefix — are filled in here. If the tree is deeper
/// than
/// [`super::walk::MAX_TREE_DEPTH`] the results are truncated and a WARN is
/// emitted; no error is returned.
pub(crate) fn extract_all(root: &Node<'_>, source: &str, language: &str, wanted: Wanted, out: &mut ProcessResult) {
    // ~keep SQL, Just, Dockerfile, C and Cedar declarations are read by their own
    // ~keep adapters (`super::sql`, `super::just`, `super::dockerfile`, `super::c`,
    // ~keep `super::cedar`): their grammars share node names with the generic arms
    // ~keep while meaning something else (a plpgsql DECLARE variable is a
    // ~keep `function_declaration`; a C prototype is a `declaration` the generic
    // ~keep arms never see), so the language-neutral structure and symbol
    // ~keep matchers must not see them.
    let declarative = matches!(language, "sql" | "just" | "dockerfile" | "c" | "cedar");
    let generic = Wanted {
        structure: wanted.structure && !declarative,
        symbols: wanted.symbols && !declarative,
        ..wanted
    };
    let mut collector = Collector::new(source, language, generic);
    let truncated = walk_bounded(root, |node, depth| collector.visit(node, depth));
    warn_if_truncated(truncated, "intel::extract", language);
    collector.finish(out);
    if wanted.structure && declarative {
        out.structure = match language {
            "sql" => super::sql::structure(root, source),
            "just" => super::just::structure(root, source),
            "c" => super::c::structure(root, source),
            "cedar" => super::cedar::structure(root, source),
            _ => super::dockerfile::structure(root, source),
        };
    }
    // ~keep Counts are reported once here, after the walk: a per-node event would
    // ~keep reintroduce the per-node cost this single-pass design exists to remove.
    tracing::debug!(
        target: "ts_pack::intel",
        operation = "intel::extract",
        language,
        nodes = out.metrics.node_count,
        max_depth = out.metrics.max_depth,
        error_nodes = out.metrics.error_count,
        structure = out.structure.len(),
        imports = out.imports.len(),
        exports = out.exports.len(),
        comments = out.comments.len(),
        docstrings = out.docstrings.len(),
        symbols = out.symbols.len(),
        diagnostics = out.diagnostics.len(),
        "extraction complete"
    );
}

struct Collector<'a> {
    source: &'a str,
    language: &'a str,
    wanted: Wanted,
    node_count: usize,
    error_count: usize,
    max_depth: usize,
    /// Rows the AST says a comment occupies. Collected unconditionally: it feeds
    /// `FileMetrics`, which every `process()` call returns.
    comment_rows: BTreeSet<usize>,
    comments: Vec<CommentInfo>,
    docstrings: Vec<DocstringInfo>,
    exports: Vec<ExportInfo>,
    symbols: Vec<SymbolInfo>,
    diagnostics: Vec<Diagnostic>,
    imports: ImportScope,
    structure: StructureScope,
}

impl<'a> Collector<'a> {
    fn new(source: &'a str, language: &'a str, wanted: Wanted) -> Self {
        Self {
            source,
            language,
            wanted,
            node_count: 0,
            error_count: 0,
            max_depth: 0,
            comment_rows: BTreeSet::new(),
            comments: Vec::new(),
            docstrings: Vec::new(),
            exports: Vec::new(),
            symbols: Vec::new(),
            diagnostics: Vec::new(),
            imports: ImportScope::default(),
            structure: StructureScope::default(),
        }
    }

    fn visit(&mut self, node: &Node<'_>, depth: usize) -> Descend {
        self.node_count += 1;
        self.max_depth = self.max_depth.max(depth);
        if node.is_error() || node.is_missing() {
            self.error_count += 1;
        }
        if is_comment_node(node) {
            mark_comment_rows(node, self.source, &mut self.comment_rows);
        }

        if self.wanted.comments
            && let Some(comment) = comment_at(node, self.source)
        {
            self.comments.push(comment);
        }
        if self.wanted.docstrings
            && let Some(docstring) = docstring_at(node, self.source, self.language)
        {
            self.docstrings.push(docstring);
        }
        if self.wanted.exports
            && let Some(export) = export_at(node, self.source, self.language)
        {
            self.exports.push(export);
        }
        if self.wanted.symbols
            && let Some(symbol) = symbol_at(node, self.source, self.language)
        {
            self.symbols.push(symbol);
        }
        if self.wanted.diagnostics
            && let Some(diagnostic) = diagnostic_at(node, self.source)
        {
            self.diagnostics.push(diagnostic);
        }
        if self.wanted.imports {
            self.imports.visit(node, depth, self.source, self.language);
        }
        if self.wanted.structure {
            self.structure.visit(node, depth, self.source, self.language);
        }
        Descend::Children
    }

    fn finish(self, out: &mut ProcessResult) {
        out.metrics.node_count = self.node_count;
        out.metrics.error_count = self.error_count;
        out.metrics.max_depth = self.max_depth;
        apply_comment_lines(&mut out.metrics, self.source, &self.comment_rows);
        out.comments = self.comments;
        out.docstrings = self.docstrings;
        out.exports = self.exports;
        out.symbols = self.symbols;
        out.diagnostics = self.diagnostics;
        out.imports = self.imports.found;
        out.structure = self.structure.finish();
    }
}

/// Import collection. Language-neutral except for Elixir, whose directives are
/// `call` nodes and whose `quote` bodies are generated AST that must not be
/// reported.
#[derive(Default)]
struct ImportScope {
    found: Vec<ImportInfo>,
    quote_depth: Option<usize>,
}

impl ImportScope {
    fn visit(&mut self, node: &Node<'_>, depth: usize, source: &str, language: &str) {
        if self.quote_depth.is_some_and(|quoted| depth <= quoted) {
            self.quote_depth = None;
        }
        if self.quote_depth.is_some() {
            return;
        }
        if language == "elixir" && node.kind() == "call" {
            if elixir::is_quote_call(node, source) {
                self.quote_depth = Some(depth);
            } else if let Some(import) = elixir::import_directive(node, source) {
                self.found.push(import);
            }
            return;
        }
        if let Some(import) = import_at(node, source, language) {
            self.found.push(import);
        }
    }
}

/// A structure item whose subtree is still being walked.
struct OpenItem {
    /// Depth of the item's own node; the item closes when the walk returns here.
    depth: usize,
    /// Node id of the item's body, the only child subtree that may contribute
    /// nested items.
    body_id: Option<usize>,
    /// Depth at which the body node was entered, or `None` while outside it.
    body_depth: Option<usize>,
    item: StructureItem,
    children: Vec<StructureItem>,
}

/// Structure collection.
///
/// The structure walk is not a uniform visit-every-node walk: a matched node
/// emits an item and then descends *only* into its body, nesting whatever it
/// finds there. This reproduces that with a stack of open items — an item is
/// pushed when matched and popped into its parent when the traversal leaves its
/// subtree, which yields the same items in the same order as the recursion did.
#[derive(Default)]
struct StructureScope {
    open: Vec<OpenItem>,
    done: Vec<StructureItem>,
    quote_depth: Option<usize>,
}

impl StructureScope {
    fn visit(&mut self, node: &Node<'_>, depth: usize, source: &str, language: &str) {
        self.leave(depth);
        if self.quote_depth.is_some() {
            return;
        }
        if !self.is_in_scope(node, depth) {
            return;
        }
        self.try_open(node, depth, source, language);
    }

    /// Close every item the traversal has walked past, and drop a body or quote
    /// scope that no longer contains the current position.
    fn leave(&mut self, depth: usize) {
        if self.quote_depth.is_some_and(|quoted| depth <= quoted) {
            self.quote_depth = None;
        }
        while self.open.last().is_some_and(|open| depth <= open.depth) {
            let Some(mut open) = self.open.pop() else { break };
            open.item.children = open.children;
            match self.open.last_mut() {
                Some(parent) => parent.children.push(open.item),
                None => self.done.push(open.item),
            }
        }
        if let Some(top) = self.open.last_mut()
            && top.body_depth.is_some_and(|body_depth| depth <= body_depth)
        {
            top.body_depth = None;
        }
    }

    /// Whether the structure walk may inspect `node` at all: true at top level,
    /// and inside the innermost open item only within its body subtree. Marks
    /// the body scope as entered when `node` is that body.
    fn is_in_scope(&mut self, node: &Node<'_>, depth: usize) -> bool {
        let Some(top) = self.open.last_mut() else {
            return true;
        };
        if top.body_id == Some(node.id()) {
            top.body_depth = Some(depth);
        }
        top.body_depth.is_some()
    }

    fn try_open(&mut self, node: &Node<'_>, depth: usize, source: &str, language: &str) {
        if language == "elixir" {
            // ~keep A `quote` block body is generated AST, not real structure.
            if elixir::is_quote_call(node, source) {
                self.quote_depth = Some(depth);
                return;
            }
            if let Some(definition) = elixir::definition(node, source) {
                let body = definition.body;
                self.push(
                    node,
                    depth,
                    source,
                    definition.kind,
                    definition.name,
                    definition.visibility,
                    body,
                );
                return;
            }
        }
        if let Some(kind) = structure_kind_at(node, language) {
            let name = resolve_structure_name(node, source);
            let body = node.child_by_field_name("body");
            self.push(node, depth, source, kind, name, None, body);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn push(
        &mut self,
        node: &Node<'_>,
        depth: usize,
        source: &str,
        kind: StructureKind,
        name: Option<String>,
        visibility: Option<String>,
        body: Option<Node<'_>>,
    ) {
        let signature = structure_signature(node, source, body.as_ref());
        self.open.push(OpenItem {
            depth,
            body_id: body.as_ref().map(Node::id),
            body_depth: None,
            item: StructureItem {
                kind,
                name,
                visibility,
                span: span_from_node(node),
                children: Vec::new(),
                decorators: Vec::new(),
                doc_comment: doc_comment_at(node, source),
                signature,
                body_span: body.as_ref().map(span_from_node),
            },
            children: Vec::new(),
        });
    }

    fn finish(mut self) -> Vec<StructureItem> {
        // ~keep Depth 0 is at or above every open item, so this closes all of them.
        self.leave(0);
        self.done
    }
}
