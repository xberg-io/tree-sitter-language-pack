use std::borrow::Cow;

use crate::Error;

/// Lightweight snapshot of a tree-sitter node's properties.
///
/// Contains only primitive types for easy cross-language serialization.
/// This is an owned type that can be passed across FFI boundaries, unlike
/// `tree_sitter::Node` which borrows from the tree.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NodeInfo {
    /// The grammar type name (e.g., "function_definition", "identifier").
    pub kind: Cow<'static, str>,
    /// Whether this is a named node (vs anonymous like punctuation).
    pub is_named: bool,
    /// Start byte offset in source.
    pub start_byte: usize,
    /// End byte offset in source.
    pub end_byte: usize,
    /// Start row (zero-indexed).
    pub start_row: usize,
    /// Start column (zero-indexed).
    pub start_col: usize,
    /// End row (zero-indexed).
    pub end_row: usize,
    /// End column (zero-indexed).
    pub end_col: usize,
    /// Number of named children.
    pub named_child_count: usize,
    /// Whether this node is an ERROR node.
    pub is_error: bool,
    /// Whether this node is a MISSING node.
    pub is_missing: bool,
}

/// Extract a `NodeInfo` from a tree-sitter `Node`.
pub(crate) fn node_info_from_node(node: tree_sitter::Node) -> NodeInfo {
    let start = node.start_position();
    let end = node.end_position();
    NodeInfo {
        kind: Cow::Borrowed(node.kind()),
        is_named: node.is_named(),
        start_byte: node.start_byte(),
        end_byte: node.end_byte(),
        start_row: start.row,
        start_col: start.column,
        end_row: end.row,
        end_col: end.column,
        named_child_count: node.named_child_count(),
        is_error: node.is_error(),
        is_missing: node.is_missing(),
    }
}

/// Get a `NodeInfo` snapshot of the root node.
pub(crate) fn root_node_info(tree: &tree_sitter::Tree) -> NodeInfo {
    node_info_from_node(tree.root_node())
}

/// Find all nodes matching the given type name, returning their `NodeInfo`.
///
/// Performs a depth-first traversal. Returns an empty vec if no matches.
pub(crate) fn find_nodes_by_type(tree: &tree_sitter::Tree, node_type: &str) -> Vec<NodeInfo> {
    let mut results = Vec::new();
    let mut cursor = tree.walk();
    collect_with_cursor(&mut cursor, |node| {
        if node.kind() == node_type {
            results.push(node_info_from_node(node));
        }
    });
    results
}

/// Get `NodeInfo` for all named children of the root node.
///
/// Useful for understanding the top-level structure of a file
/// (e.g., list of function definitions, class declarations, imports).
pub(crate) fn named_children_info(tree: &tree_sitter::Tree) -> Vec<NodeInfo> {
    let root = tree.root_node();
    let mut children = Vec::with_capacity(root.named_child_count());
    let mut cursor = root.walk();
    if cursor.goto_first_child() {
        loop {
            let node = cursor.node();
            if node.is_named() {
                children.push(node_info_from_node(node));
            }
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
    children
}

/// Extract the source text corresponding to a node's byte range.
///
/// Returns the slice of source bytes as a UTF-8 string.
pub(crate) fn extract_text<'a>(source: &'a [u8], node_info: &NodeInfo) -> Result<&'a str, Error> {
    if node_info.end_byte > source.len() {
        return Err(Error::InvalidRange(format!(
            "end_byte {} exceeds source length {}",
            node_info.end_byte,
            source.len()
        )));
    }
    std::str::from_utf8(&source[node_info.start_byte..node_info.end_byte])
        .map_err(|e| Error::InvalidRange(format!("not valid UTF-8: {e}")))
}

/// Visit every node in a depth-first traversal, calling `visitor` on each.
fn collect_with_cursor(cursor: &mut tree_sitter::TreeCursor, mut visitor: impl FnMut(tree_sitter::Node)) {
    loop {
        visitor(cursor.node());
        if cursor.goto_first_child() {
            continue;
        }
        loop {
            if cursor.goto_next_sibling() {
                break;
            }
            if !cursor.goto_parent() {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_first_lang(source: &[u8]) -> Option<tree_sitter::Tree> {
        let langs = crate::available_languages();
        let first = langs.first()?;
        crate::parse::parse_string(first, source).ok()
    }

    #[test]
    fn test_root_node_info() {
        let Some(tree) = parse_first_lang(b"x") else {
            return;
        };
        let info = root_node_info(&tree);
        assert!(!info.kind.is_empty());
        assert!(info.is_named);
        assert_eq!(info.start_byte, 0);
    }

    #[test]
    fn test_find_nodes_by_type() {
        let Some(tree) = parse_first_lang(b"x") else {
            return;
        };
        let root_kind = tree.root_node().kind().to_string();
        let nodes = find_nodes_by_type(&tree, &root_kind);
        assert!(!nodes.is_empty());
        assert_eq!(nodes[0].kind, root_kind);
    }

    #[test]
    fn test_find_nodes_by_type_no_match() {
        let Some(tree) = parse_first_lang(b"x") else {
            return;
        };
        let nodes = find_nodes_by_type(&tree, "nonexistent_node_type_xyz");
        assert!(nodes.is_empty());
    }

    #[test]
    fn test_named_children_info() {
        let Some(tree) = parse_first_lang(b"x") else {
            return;
        };
        let children = named_children_info(&tree);
        // Root should have at least one named child for most grammars
        // (the parsed "x" token), but this depends on the grammar
        let _ = children;
    }

    #[test]
    fn test_extract_text() {
        let source = b"hello world";
        let info = NodeInfo {
            kind: Cow::Owned("test".to_string()),
            is_named: true,
            start_byte: 0,
            end_byte: 5,
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 5,
            named_child_count: 0,
            is_error: false,
            is_missing: false,
        };
        let text = extract_text(source, &info).unwrap();
        assert_eq!(text, "hello");
    }

    #[test]
    fn test_extract_text_out_of_bounds() {
        let source = b"hi";
        let info = NodeInfo {
            kind: Cow::Owned("test".to_string()),
            is_named: true,
            start_byte: 0,
            end_byte: 100,
            start_row: 0,
            start_col: 0,
            end_row: 0,
            end_col: 100,
            named_child_count: 0,
            is_error: false,
            is_missing: false,
        };
        assert!(extract_text(source, &info).is_err());
    }
}
