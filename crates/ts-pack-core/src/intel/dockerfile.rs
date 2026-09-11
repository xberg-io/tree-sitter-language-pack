//! Dockerfile declaration extraction.
//!
//! A Dockerfile is a flat list of instructions, but a build stage is a real
//! scope: it begins at `FROM ... AS name` and runs to the instruction before
//! the next `FROM`. A named stage is a module whose span covers that run, and
//! the `ARG` and `ENV` names declared inside it nest under it, so `ARG MIX_ENV`
//! repeated in four stages of one file keeps four identities. Instructions
//! before the first `FROM`, or under an unnamed `FROM`, are top-level items.
//!
//! `ARG` and every `ENV` pair are constants named by the variable and spanning
//! the whole instruction. Values never reach an item. No other instruction
//! declares a name, so `RUN`, `COPY` and the rest produce nothing.
//!
//! Entry point: [`structure`].

use tree_sitter::Node;

use super::intelligence::node_text;
use super::types::*;
use super::walk::{Descend, walk_bounded, warn_if_truncated};

/// Every named stage, argument and environment variable in `root`, in source
/// order, with a stage's declarations nested under it.
pub(super) fn structure(root: &Node<'_>, source: &str) -> Vec<StructureItem> {
    let mut items = Vec::new();
    let mut stage: Option<Stage> = None;
    let truncated = walk_bounded(root, |node, _depth| {
        match node.kind() {
            "from_instruction" => {
                close(&mut stage, &mut items, source);
                stage = node.child_by_field_name("as").map(|alias| {
                    let start = node.start_position();
                    Stage {
                        name: node_text(&alias, source).to_string(),
                        start: node.start_byte(),
                        start_position: (start.row, start.column),
                        end: node.end_byte(),
                        children: Vec::new(),
                    }
                });
            }
            "arg_instruction" => {
                if let Some(name) = node.child_by_field_name("name") {
                    push(&mut stage, &mut items, constant(&name, node, source));
                }
                extend(&mut stage, node);
            }
            "env_instruction" => {
                let mut cursor = node.walk();
                for pair in node
                    .named_children(&mut cursor)
                    .filter(|child| child.kind() == "env_pair")
                {
                    if let Some(name) = pair.child_by_field_name("name") {
                        push(&mut stage, &mut items, constant(&name, node, source));
                    }
                }
                extend(&mut stage, node);
            }
            // ~keep Error recovery and the root are transparent; a stage may sit inside either.
            "source_file" | "ERROR" => return Descend::Children,
            "comment" => {}
            kind if kind.ends_with("_instruction") => extend(&mut stage, node),
            _ => {}
        }
        Descend::Skip
    });
    warn_if_truncated(truncated, "intel::dockerfile", "dockerfile");
    close(&mut stage, &mut items, source);
    items
}

struct Stage {
    name: String,
    start: usize,
    /// The row and column of `start`, carried from the `FROM` node so closing
    /// the stage does not recount the newlines above it.
    start_position: (usize, usize),
    end: usize,
    children: Vec<StructureItem>,
}

fn constant(name: &Node<'_>, instruction: &Node<'_>, source: &str) -> StructureItem {
    StructureItem {
        kind: StructureKind::Other("Constant".to_string()),
        name: Some(node_text(name, source).to_string()),
        span: super::intelligence::span_trimmed(instruction, source),
        ..StructureItem::default()
    }
}

fn push(stage: &mut Option<Stage>, items: &mut Vec<StructureItem>, item: StructureItem) {
    match stage {
        Some(stage) => stage.children.push(item),
        None => items.push(item),
    }
}

fn extend(stage: &mut Option<Stage>, instruction: &Node<'_>) {
    if let Some(stage) = stage {
        stage.end = stage.end.max(instruction.end_byte());
    }
}

fn close(stage: &mut Option<Stage>, items: &mut Vec<StructureItem>, source: &str) {
    if let Some(stage) = stage.take() {
        items.push(StructureItem {
            kind: StructureKind::Module,
            name: Some(stage.name),
            span: super::intelligence::span_between_from(source, stage.start, stage.end, stage.start_position),
            children: stage.children,
            ..StructureItem::default()
        });
    }
}
