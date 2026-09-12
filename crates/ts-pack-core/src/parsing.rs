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
    /// Zero-indexed column, counted in **bytes** from the start of the row.
    ///
    /// This is `tree_sitter::Point::column` verbatim, and tree-sitter defines it
    /// as a byte offset — not characters, and not UTF-16 code units. Measured on
    /// a row of the form `x = '<c>'` where `<c>` is a single 4-byte character
    /// (an emoji), the string node reports columns `4..10`, identical to its
    /// byte range; the character columns would be `4..7` and the UTF-16 columns
    /// `4..8`.
    ///
    /// Anything that builds LSP positions (UTF-16) or editor caret columns
    /// (characters) from this field is silently wrong on every row containing a
    /// non-ASCII byte, and must re-measure the row against the source text
    /// instead. Earlier releases documented this field as UTF-16 code units;
    /// that was never what the value contained. ~keep
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
    max_source_bytes: Option<usize>,
    timeout_ms: Option<u64>,
    language_name: Option<String>,
}

impl Parser {
    /// Construct a new parser with no language set and no parse limits.
    ///
    /// Call [`Parser::set_language`] before parsing. Limits are opt-in via
    /// [`set_max_source_bytes`](Self::set_max_source_bytes) and
    /// [`set_parse_timeout_ms`](Self::set_parse_timeout_ms); both default to
    /// unbounded so that existing callers are unaffected. ~keep
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: tree_sitter::Parser::new(),
            max_source_bytes: None,
            timeout_ms: None,
            language_name: None,
        }
    }

    /// Refuse to parse sources longer than `max_bytes`.
    ///
    /// `None` (the default) means no limit. Over-limit input makes
    /// [`parse`](Self::parse) and [`parse_bytes`](Self::parse_bytes) return
    /// `None` after emitting a `WARN`; input is never silently truncated.
    ///
    /// See [`RECOMMENDED_MAX_SOURCE_BYTES`](crate::process_config::RECOMMENDED_MAX_SOURCE_BYTES).
    pub fn set_max_source_bytes(&mut self, max_bytes: Option<usize>) {
        self.max_source_bytes = max_bytes;
    }

    /// Cancel a parse that exceeds `timeout_ms` milliseconds of wall clock.
    ///
    /// `None` (the default) means no budget. Cancellation runs through
    /// tree-sitter's parse progress callback, so it is granular to that
    /// callback's interval rather than exact.
    ///
    /// See [`RECOMMENDED_PARSE_TIMEOUT_MS`](crate::process_config::RECOMMENDED_PARSE_TIMEOUT_MS).
    pub fn set_parse_timeout_ms(&mut self, timeout_ms: Option<u64>) {
        self.timeout_ms = timeout_ms;
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
            .map_err(|e| Error::ParserSetup(format!("{e}")))?;
        self.language_name = Some(name.to_string());
        Ok(())
    }

    /// Parse a UTF-8 source string.
    ///
    /// Returns `None` if no language is set, if the parse was cancelled by the
    /// configured [timeout](Self::set_parse_timeout_ms), or if the source
    /// exceeds the configured [size limit](Self::set_max_source_bytes). Each
    /// non-parse outcome is logged, so an empty result is never silent.
    ///
    /// # Concurrency
    ///
    /// Parsing runs fully in parallel across threads for almost every language — no lock is
    /// taken. The exception is the small set of grammars whose external scanner keeps mutable
    /// process-global state (currently just `properties`): parses of that language are
    /// serialized against each other through an internal per-language lock, so that concurrent
    /// threads can't corrupt shared scanner state. Every other language is unaffected by that
    /// lock and never waits on it.
    #[must_use]
    pub fn parse(&mut self, source: &str) -> Option<Tree> {
        self.parse_bytes(source.as_bytes())
    }

    /// Parse a raw byte slice.
    ///
    /// Same outcomes as [`parse`](Self::parse), including the concurrency behaviour documented
    /// there.
    #[must_use]
    pub fn parse_bytes(&mut self, source: &[u8]) -> Option<Tree> {
        if let Some(limit) = self.max_source_bytes
            && source.len() > limit
        {
            tracing::warn!(
                source_bytes = source.len(),
                limit,
                "source exceeds the configured max_source_bytes; refusing to parse"
            );
            return None;
        }

        let _guard = self
            .language_name
            .as_deref()
            .and_then(crate::parse::lock_for_stateful_scanner);

        match crate::parse::run_parse(&mut self.inner, source, self.timeout_ms) {
            Ok(tree) => Some(Tree(Arc::new(tree))),
            Err(error) => {
                tracing::debug!(error = %error, "parse produced no tree");
                None
            }
        }
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

// `tree_sitter::Parser` is a `NonNull` newtype with no `Debug` of its own, so
// this cannot be derived; the pointer is elided and only the configuration a
// caller set is reported. ~keep
impl std::fmt::Debug for Parser {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("Parser")
            .field("language_name", &self.language_name)
            .field("max_source_bytes", &self.max_source_bytes)
            .field("timeout_ms", &self.timeout_ms)
            .finish_non_exhaustive()
    }
}

/// A parsed syntax tree. Cheap to clone (refcount bump).
#[derive(Clone)]
pub struct Tree(Arc<tree_sitter::Tree>);

impl Tree {
    /// Return the root [`Node`] of this tree.
    #[must_use]
    pub fn root_node(&self) -> Node {
        // ~keep SAFETY: Node holds an Arc<Tree>, keeping the backing tree alive for the lifetime-extended raw.
        let raw: tree_sitter::Node<'static> = unsafe { std::mem::transmute(self.0.root_node()) };
        Node {
            tree: Arc::clone(&self.0),
            raw,
        }
    }

    /// Return a [`TreeCursor`] positioned at the root.
    #[must_use]
    pub fn walk(&self) -> TreeCursor {
        // ~keep SAFETY: same as `root_node`; TreeCursor owns the Arc<Tree> that keeps storage alive.
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
        // ~keep SAFETY: the returned Node holds Arc<Tree>, keeping the lifetime-extended raw valid.
        self.raw.parent().map(|raw| Node {
            tree: Arc::clone(&self.tree),
            raw: unsafe { std::mem::transmute::<tree_sitter::Node<'_>, tree_sitter::Node<'static>>(raw) },
        })
    }

    /// Return the i-th child of this node, if any.
    #[must_use]
    pub fn child(&self, index: u32) -> Option<Node> {
        // ~keep SAFETY: see `parent`.
        self.raw.child(index).map(|raw| Node {
            tree: Arc::clone(&self.tree),
            raw: unsafe { std::mem::transmute::<tree_sitter::Node<'_>, tree_sitter::Node<'static>>(raw) },
        })
    }

    /// Total number of children (including unnamed).
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.raw.child_count() as usize
    }

    /// Return the i-th named child of this node, if any.
    #[must_use]
    pub fn named_child(&self, index: u32) -> Option<Node> {
        // ~keep SAFETY: see `parent`.
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
        // ~keep SAFETY: see `parent`.
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
        // ~keep SAFETY: see `Tree::walk`; the cursor holds Arc<Tree>.
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
        // ~keep SAFETY: see `Tree::root_node`.
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
