//! Justfile declaration extraction.
//!
//! A recipe is the callable unit of a Justfile, so each `recipe` node is a
//! function whose span runs from its first attribute through its last body
//! line: the grammar's node swallows the blank lines after the body, and those
//! are trimmed off. `alias` and `mod` are the other two named declarations.
//! Settings, assignments and imports declare nothing callable and are skipped.
//!
//! Entry point: [`structure`].

use tree_sitter::Node;

use super::intelligence::node_text;
use super::types::*;
use super::walk::{Descend, walk_bounded, warn_if_truncated};

/// Every recipe, alias and module in `root`, in source order.
pub(super) fn structure(root: &Node<'_>, source: &str) -> Vec<StructureItem> {
    let mut items = Vec::new();
    let truncated = walk_bounded(root, |node, _depth| {
        let named = match node.kind() {
            "recipe" => {
                let header = child_of_kind(node, "recipe_header");
                header
                    .and_then(|header| header.child_by_field_name("name"))
                    .map(|name| (StructureKind::Function, name))
            }
            "alias" => node
                .child_by_field_name("left")
                .map(|name| (StructureKind::Other("Alias".to_string()), name)),
            "module" => node
                .child_by_field_name("name")
                .map(|name| (StructureKind::Module, name)),
            _ => return Descend::Children,
        };
        if let Some((kind, name)) = named {
            items.push(StructureItem {
                kind,
                name: Some(node_text(&name, source).to_string()),
                span: super::intelligence::span_trimmed(node, source),
                ..StructureItem::default()
            });
        }
        Descend::Skip
    });
    warn_if_truncated(truncated, "intel::just", "just");
    items
}

fn child_of_kind<'tree>(node: &Node<'tree>, kind: &str) -> Option<Node<'tree>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor).find(|child| child.kind() == kind)
}
