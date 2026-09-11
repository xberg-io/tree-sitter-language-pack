//! Cedar policy extraction.
//!
//! A policy set is a flat list of `permit` and `forbid` policies, and Cedar
//! itself identifies a policy by its `@id("...")` annotation, so that is the
//! name here. A policy without an `@id` has no name Cedar would report it by
//! and produces nothing; two policies carrying the same `@id` are two items,
//! told apart by their spans. Templates (`?principal`, `?resource`) are
//! policies to the grammar and are read the same way. Comments, including one
//! that spells `@id(...)` in prose, are never policies.
//!
//! The span is the policy node: its annotations through its terminator. The
//! effect, scope and conditions never reach an item.
//!
//! Entry point: [`structure`].

use tree_sitter::Node;

use super::intelligence::{node_text, span_trimmed};
use super::types::*;
use super::walk::{Descend, walk_bounded, warn_if_truncated};

/// Every policy in `root` that carries an `@id`, in source order.
pub(super) fn structure(root: &Node<'_>, source: &str) -> Vec<StructureItem> {
    let mut items = Vec::new();
    let truncated = walk_bounded(root, |node, _depth| {
        if node.kind() != "policy" {
            return Descend::Children;
        }
        if let Some(id) = policy_id(node, source) {
            items.push(StructureItem {
                kind: StructureKind::Other("Policy".to_string()),
                name: Some(id),
                span: span_trimmed(node, source),
                ..StructureItem::default()
            });
        }
        Descend::Skip
    });
    warn_if_truncated(truncated, "intel::cedar", "cedar");
    items
}

/// The content of the first `@id("...")` annotation on `policy`, as written.
fn policy_id(policy: &Node<'_>, source: &str) -> Option<String> {
    let mut cursor = policy.walk();
    policy
        .named_children(&mut cursor)
        .filter(|child| child.kind() == "annotation")
        .find_map(|annotation| {
            let mut inner = annotation.walk();
            let mut parts = annotation.named_children(&mut inner);
            parts.next().filter(|key| node_text(key, source) == "id")?;
            let string = parts.next().filter(|value| value.kind() == "string")?;
            let mut chars = string.walk();
            string
                .named_children(&mut chars)
                .find(|part| part.kind() == "string_content")
                .map(|content| node_text(&content, source).to_string())
        })
}
