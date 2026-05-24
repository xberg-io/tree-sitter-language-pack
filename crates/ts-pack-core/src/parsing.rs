//! Universal parser/tree/node surface over `tree_sitter`.
//!
//! These wrappers own the relationship between a [`Tree`] and the [`Node`]s
//! and [`TreeCursor`]s derived from it. Upstream `tree_sitter::Node<'a>` and
//! `tree_sitter::TreeCursor<'a>` borrow from `tree_sitter::Tree`; that
//! borrow can't be propagated across FFI boundaries, so each [`Node`] and
//! [`TreeCursor`] here owns an `Arc<tree_sitter::Tree>` and stores the raw
//! upstream value with an extended `'static` lifetime. The `Arc<>` keeps
//! the underlying tree alive for as long as any derived node or cursor
//! exists.
//!
//! Cloning a [`Tree`] (and therefore any [`Node`] or [`TreeCursor`]) is
//! cheap — one atomic refcount bump.

use std::sync::Arc;

use crate::error::Error;

/// A source position — row + column, zero-indexed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Point {
    /// Zero-indexed row number.
    pub row: usize,
    /// Zero-indexed column number, in UTF-16 code units.
    pub column: usize,
}

impl Point {
    /// Construct a [`Point`] from row and column.
    #[cfg_attr(alef, alef(skip))]
    #[must_use]
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

impl From<tree_sitter::Point> for Point {
    fn from(p: tree_sitter::Point) -> Self {
        Self {
            row: p.row,
            column: p.column,
        }
    }
}

/// A byte range — start (inclusive) to end (exclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ByteRange {
    /// Inclusive start byte offset.
    pub start: usize,
    /// Exclusive end byte offset.
    pub end: usize,
}

/// A tree-sitter parser configured for one language at a time.
///
/// # Example
///
/// ```no_run
/// use tree_sitter_language_pack::Parser;
///
/// let mut parser = Parser::new();
/// parser.set_language("python")?;
/// let tree = parser.parse("def hello(): pass").expect("parse failed");
/// assert_eq!(tree.root_node().kind(), "module");
/// # Ok::<(), tree_sitter_language_pack::Error>(())
/// ```
pub struct Parser {
    inner: tree_sitter::Parser,
}

impl Parser {
    /// Construct a new parser with no language set.
    ///
    /// Call [`Parser::set_language`] before parsing.
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: tree_sitter::Parser::new(),
        }
    }

    /// Configure the parser to use the language identified by name (e.g. `"python"`).
    ///
    /// Resolves the language through the global registry — auto-downloading
    /// if necessary, when the `download` feature is enabled.
    ///
    /// # Errors
    ///
    /// Returns [`Error::LanguageNotFound`] if the language is not recognized,
    /// or [`Error::ParserSetup`] if the language ABI is incompatible.
    pub fn set_language(&mut self, name: &str) -> Result<(), Error> {
        let language = crate::get_language(name)?;
        self.inner
            .set_language(&language)
            .map_err(|e| Error::ParserSetup(format!("{e}")))
    }

    /// Parse a UTF-8 source string. Returns `None` if parsing was cancelled
    /// or no language is set.
    #[must_use]
    pub fn parse(&mut self, source: &str) -> Option<Tree> {
        self.inner.parse(source, None).map(|t| Tree(Arc::new(t)))
    }

    /// Parse a raw byte slice. Returns `None` if parsing was cancelled or
    /// no language is set.
    #[must_use]
    pub fn parse_bytes(&mut self, source: &[u8]) -> Option<Tree> {
        self.inner.parse(source, None).map(|t| Tree(Arc::new(t)))
    }

    /// Reset internal state. The next call to [`parse`](Self::parse) will
    /// not be incremental.
    pub fn reset(&mut self) {
        self.inner.reset();
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

/// A parsed syntax tree. Cheap to clone (refcount bump).
#[derive(Clone)]
pub struct Tree(Arc<tree_sitter::Tree>);

impl Tree {
    /// Return the root [`Node`] of this tree.
    #[must_use]
    pub fn root_node(&self) -> Node {
        // SAFETY: Node holds an `Arc<tree_sitter::Tree>` that keeps the
        // upstream tree alive for the entire lifetime of `raw`. Extending
        // the borrow to 'static is valid because the Arc owns the
        // backing storage; no aliasing rules are violated because every
        // Node clone increments the Arc refcount.
        let raw: tree_sitter::Node<'static> = unsafe { std::mem::transmute(self.0.root_node()) };
        Node {
            tree: Arc::clone(&self.0),
            raw,
        }
    }

    /// Return a [`TreeCursor`] positioned at the root.
    #[must_use]
    pub fn walk(&self) -> TreeCursor {
        // SAFETY: same justification as `root_node` — TreeCursor owns the
        // Arc<Tree> that keeps the backing tree alive.
        let raw: tree_sitter::TreeCursor<'static> = unsafe { std::mem::transmute(self.0.walk()) };
        TreeCursor {
            tree: Arc::clone(&self.0),
            raw,
        }
    }
}

/// A single syntax node within a [`Tree`].
///
/// Nodes hold a strong reference to their parent tree so they remain valid
/// regardless of how the tree is moved or stored at the FFI boundary.
pub struct Node {
    tree: Arc<tree_sitter::Tree>,
    raw: tree_sitter::Node<'static>,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Self {
            tree: Arc::clone(&self.tree),
            raw: self.raw,
        }
    }
}

impl Node {
    /// Return the node's kind name (e.g. `"function_definition"`).
    #[must_use]
    pub fn kind(&self) -> String {
        self.raw.kind().to_string()
    }

    /// Return the node's numeric kind ID.
    ///
    /// Tree-sitter assigns a stable `u16` ID to every node kind in a grammar
    /// (e.g. `"function_definition" → 42`). Comparing `kind_id()` is cheaper
    /// than comparing the string [`kind()`](Self::kind) in tight AST loops.
    #[must_use]
    pub fn kind_id(&self) -> u16 {
        self.raw.kind_id()
    }

    /// Return the inclusive start byte offset of this node.
    #[must_use]
    pub fn start_byte(&self) -> usize {
        self.raw.start_byte()
    }

    /// Return the exclusive end byte offset of this node.
    #[must_use]
    pub fn end_byte(&self) -> usize {
        self.raw.end_byte()
    }

    /// Return the node's byte range as a [`ByteRange`].
    ///
    /// Callers should slice their own source bytes — this is a zero-copy
    /// text accessor.
    #[must_use]
    pub fn byte_range(&self) -> ByteRange {
        let r = self.raw.byte_range();
        ByteRange {
            start: r.start,
            end: r.end,
        }
    }

    /// Return the start [`Point`] (row, column).
    #[must_use]
    pub fn start_position(&self) -> Point {
        self.raw.start_position().into()
    }

    /// Return the end [`Point`] (row, column).
    #[must_use]
    pub fn end_position(&self) -> Point {
        self.raw.end_position().into()
    }

    /// True when this node is named (not punctuation/whitespace).
    #[must_use]
    pub fn is_named(&self) -> bool {
        self.raw.is_named()
    }

    /// True when this is an error node.
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.raw.is_error()
    }

    /// True when this is a missing-token node.
    #[must_use]
    pub fn is_missing(&self) -> bool {
        self.raw.is_missing()
    }

    /// True when this is an "extra" node (e.g. a comment).
    #[must_use]
    pub fn is_extra(&self) -> bool {
        self.raw.is_extra()
    }

    /// True when this node or any descendant is an error.
    #[must_use]
    pub fn has_error(&self) -> bool {
        self.raw.has_error()
    }

    /// Return this node's parent, if any.
    #[must_use]
    pub fn parent(&self) -> Option<Node> {
        // SAFETY: the returned Node holds Arc<Tree>, keeping the parent
        // tree alive while the lifetime-extended raw is used.
        self.raw.parent().map(|raw| Node {
            tree: Arc::clone(&self.tree),
            raw: unsafe { std::mem::transmute::<tree_sitter::Node<'_>, tree_sitter::Node<'static>>(raw) },
        })
    }

    /// Return the i-th child of this node, if any.
    #[must_use]
    pub fn child(&self, index: u32) -> Option<Node> {
        // SAFETY: see `parent`.
        self.raw.child(index).map(|raw| Node {
            tree: Arc::clone(&self.tree),
            raw: unsafe { std::mem::transmute::<tree_sitter::Node<'_>, tree_sitter::Node<'static>>(raw) },
        })
    }

    /// Total number of children (including unnamed).
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.raw.child_count()
    }

    /// Return the i-th named child of this node, if any.
    #[must_use]
    pub fn named_child(&self, index: u32) -> Option<Node> {
        // SAFETY: see `parent`.
        self.raw.named_child(index).map(|raw| Node {
            tree: Arc::clone(&self.tree),
            raw: unsafe { std::mem::transmute::<tree_sitter::Node<'_>, tree_sitter::Node<'static>>(raw) },
        })
    }

    /// Number of named children of this node.
    #[must_use]
    pub fn named_child_count(&self) -> usize {
        self.raw.named_child_count()
    }

    /// Look up a child by its grammar-defined field name.
    #[must_use]
    pub fn child_by_field_name(&self, name: &str) -> Option<Node> {
        // SAFETY: see `parent`.
        self.raw.child_by_field_name(name).map(|raw| Node {
            tree: Arc::clone(&self.tree),
            raw: unsafe { std::mem::transmute::<tree_sitter::Node<'_>, tree_sitter::Node<'static>>(raw) },
        })
    }

    /// Return the S-expression form of this node's subtree.
    #[must_use]
    pub fn to_sexp(&self) -> String {
        self.raw.to_sexp()
    }

    /// Return a [`TreeCursor`] positioned at this node.
    #[must_use]
    pub fn walk(&self) -> TreeCursor {
        // SAFETY: see `Tree::walk`. The cursor holds Arc<Tree>.
        let raw: tree_sitter::TreeCursor<'static> = unsafe { std::mem::transmute(self.raw.walk()) };
        TreeCursor {
            tree: Arc::clone(&self.tree),
            raw,
        }
    }
}

/// A cursor for traversing a [`Tree`].
pub struct TreeCursor {
    tree: Arc<tree_sitter::Tree>,
    raw: tree_sitter::TreeCursor<'static>,
}

impl TreeCursor {
    /// Return the [`Node`] at the cursor's current position.
    #[must_use]
    pub fn node(&self) -> Node {
        // SAFETY: see `Tree::root_node`.
        let raw: tree_sitter::Node<'static> = unsafe { std::mem::transmute(self.raw.node()) };
        Node {
            tree: Arc::clone(&self.tree),
            raw,
        }
    }

    /// Move the cursor to the first child of the current node.
    /// Returns `true` if a child existed.
    pub fn goto_first_child(&mut self) -> bool {
        self.raw.goto_first_child()
    }

    /// Move the cursor to the parent of the current node.
    /// Returns `true` if a parent existed.
    pub fn goto_parent(&mut self) -> bool {
        self.raw.goto_parent()
    }

    /// Move the cursor to the next sibling of the current node.
    /// Returns `true` if a sibling existed.
    pub fn goto_next_sibling(&mut self) -> bool {
        self.raw.goto_next_sibling()
    }

    /// Return the field name for the current node, if any.
    #[must_use]
    pub fn field_name(&self) -> Option<String> {
        self.raw.field_name().map(str::to_string)
    }
}
