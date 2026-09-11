//! C declaration extraction, for sources and headers.
//!
//! The C grammar names a definition and a prototype differently: a
//! `function_definition` has a body, while a prototype is a `declaration`
//! whose declarator is a `function_declarator`. Both are functions here,
//! because a header's prototypes are the declarations a reader of that header
//! is looking for.
//!
//! A prototype is not written only one way, so the declarator is followed
//! rather than matched: `int plain(int);` names its function directly,
//! `int (foo)(int);` wraps the name in redundant parentheses, and
//! `int (*factory(void))(int);` returns a function pointer and names its
//! function under an inner declarator. What separates those from
//! `int (*variable)(int);`, which is a function pointer VARIABLE and declares
//! no function, is whether a function declarator applies to the name. See
//! [`prototype_name`], which carries the measured shapes.
//!
//! A `struct`, `union` or `enum` is an item only where it is defined, which
//! the grammar marks with a `body`, and only where it has a name: a nameless
//! `typedef struct { ... } name_t;` is the typedef alone. A definition that
//! sits inside a `typedef` or a variable declaration is read out of the
//! `type` field, so `typedef struct node { ... } node_t;` is two items with
//! two names. Typedefs are `Other("Type")`. An object-like macro is
//! `Other("Constant")` and a function-like one `Other("Macro")`; macro bodies
//! never reach an item. Fields, enumerators, includes and anything inside a
//! function body produce nothing. Preprocessor conditionals and `extern "C"`
//! blocks are transparent, so a header guarded by `#ifndef` reads as its
//! contents.
//!
//! Entry point: [`structure`].

use tree_sitter::Node;

use super::intelligence::{node_text, span_between_from};
use super::types::*;
use super::walk::{Descend, walk_bounded, warn_if_truncated};

/// Every function, prototype, named aggregate, typedef and macro in `root`,
/// in source order.
pub(super) fn structure(root: &Node<'_>, source: &str) -> Vec<StructureItem> {
    let mut items = Vec::new();
    let truncated = walk_bounded(root, |node, _depth| {
        match node.kind() {
            "translation_unit"
            | "preproc_ifdef"
            | "preproc_if"
            | "preproc_elif"
            | "preproc_elifdef"
            | "preproc_else"
            | "linkage_specification"
            | "declaration_list"
            | "ERROR" => return Descend::Children,
            "function_definition" => {
                if let Some(name) = node.child_by_field_name("declarator").and_then(innermost_identifier) {
                    items.push(item(StructureKind::Function, &name, node, node.end_byte(), source));
                }
            }
            "declaration" => {
                let mut cursor = node.walk();
                for declarator in node.children_by_field_name("declarator", &mut cursor) {
                    if let Some(name) = prototype_name(&declarator) {
                        items.push(item(StructureKind::Function, &name, node, node.end_byte(), source));
                    }
                }
                items.extend(defined_aggregate(node, source));
            }
            "type_definition" => {
                let mut cursor = node.walk();
                for declarator in node.children_by_field_name("declarator", &mut cursor) {
                    if let Some(name) = innermost_identifier(declarator) {
                        items.push(item(
                            StructureKind::Other("Type".to_string()),
                            &name,
                            node,
                            node.end_byte(),
                            source,
                        ));
                    }
                }
                items.extend(defined_aggregate(node, source));
            }
            "struct_specifier" | "union_specifier" | "enum_specifier" => {
                items.extend(aggregate(node, source));
            }
            "preproc_def" | "preproc_function_def" => {
                if let Some(name) = node.child_by_field_name("name") {
                    let label = if node.kind() == "preproc_def" {
                        "Constant"
                    } else {
                        "Macro"
                    };
                    items.push(item(
                        StructureKind::Other(label.to_string()),
                        &name,
                        node,
                        node.end_byte(),
                        source,
                    ));
                }
            }
            _ => {}
        }
        Descend::Skip
    });
    warn_if_truncated(truncated, "intel::c", "c");
    items.sort_by_key(|item| item.span.start_byte);
    items
}

/// The identifier at the bottom of a declarator chain, whatever pointers,
/// arrays, parentheses and initialisers wrap it.
///
/// The chain is one grammar node per wrapper, so it is walked with an explicit
/// stack rather than by recursion: a declarator behind 100,000 pointers is a
/// valid parse, and a frame per pointer would overflow the 2 MiB stacks the
/// library runs on. The search is depth first in source order, first match
/// wins, which is the order the recursive form visited.
fn innermost_identifier<'tree>(node: Node<'tree>) -> Option<Node<'tree>> {
    let mut pending = vec![node];
    while let Some(current) = pending.pop() {
        match current.kind() {
            "identifier" | "type_identifier" | "field_identifier" => return Some(current),
            _ => {
                // ~keep A parenthesised declarator (`(*name)(int)`) holds its inner
                // ~keep declarator as an unnamed child rather than a field.
                if let Some(inner) = current.child_by_field_name("declarator") {
                    pending.push(inner);
                    continue;
                }
                let mut cursor = current.walk();
                let children: Vec<Node<'tree>> = current
                    .named_children(&mut cursor)
                    .filter(|child| child.kind().ends_with("declarator") || child.kind().ends_with("identifier"))
                    .collect();
                pending.extend(children.into_iter().rev());
            }
        }
    }
    None
}

/// The name a prototype declares, or `None` where the declaration declares a
/// variable instead.
///
/// C spells a function declaration more than one way, and the grammar mirrors
/// each. Measured against the real parser:
///
/// - `int plain(int);` is `function_declarator > identifier`.
/// - `int (foo)(int);` wraps the name in parentheses, which are redundant but
///   legal: `function_declarator > parenthesized_declarator > identifier`.
/// - `int (*factory(void))(int);` is a function returning a function pointer:
///   the OUTER `function_declarator` belongs to the returned pointer, and the
///   name sits under an inner one,
///   `function_declarator > parenthesized_declarator > pointer_declarator >
///   function_declarator > identifier`.
/// - `int (*variable)(int);` is a function POINTER VARIABLE and declares no
///   function: `function_declarator > parenthesized_declarator >
///   pointer_declarator > identifier`.
///
/// The last two differ only in what sits below the pointer, so the rule is
/// what a pointer is crossed *to*: reaching an identifier through a pointer
/// with no function declarator in between is a variable, while an inner
/// function declarator is a function and names it. Requiring an immediate
/// identifier, as this did before, dropped the middle two.
///
/// Iterative for the same reason as [`innermost_identifier`].
fn prototype_name<'tree>(declarator: &Node<'tree>) -> Option<Node<'tree>> {
    let mut current = *declarator;
    loop {
        match current.kind() {
            // ~keep A pointer between here and the name belongs to a
            // ~keep variable: `int (*variable)(int);` is a function pointer,
            // ~keep and the function declarator above it describes the type
            // ~keep pointed at, not something this file declares. The name is
            // ~keep only reachable through an inner function declarator, so
            // ~keep descend and let that arm decide.
            "pointer_declarator" | "parenthesized_declarator" => {
                current = current
                    .child_by_field_name("declarator")
                    .or_else(|| sole_declarator(&current))?;
            }
            "function_declarator" => {
                let inner = current.child_by_field_name("declarator")?;
                // ~keep The declarator this function applies to. A bare
                // ~keep identifier, or one wrapped only in parentheses, is
                // ~keep the function's own name.
                match inner.kind() {
                    "identifier" => return Some(inner),
                    "parenthesized_declarator" => {
                        let sole = sole_declarator(&inner)?;
                        if sole.kind() == "identifier" {
                            return Some(sole);
                        }
                        // ~keep A further declarator, so the name below it is
                        // ~keep named by ITS function declarator if it has
                        // ~keep one: `int (*factory(void))(int);` returns a
                        // ~keep function pointer and declares `factory`.
                        // ~keep Looping rather than recursing, because a
                        // ~keep declarator chain is as deep as the source
                        // ~keep makes it.
                        current = sole;
                    }
                    _ => current = inner,
                }
            }
            _ => return None,
        }
    }
}

/// The declarator inside a `parenthesized_declarator`, which the grammar
/// holds as an unnamed child rather than in the `declarator` field.
fn sole_declarator<'tree>(node: &Node<'tree>) -> Option<Node<'tree>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|child| child.kind().ends_with("declarator") || child.kind() == "identifier")
}

/// A named aggregate defined in the `type` field of a typedef or declaration.
fn defined_aggregate(node: &Node<'_>, source: &str) -> Option<StructureItem> {
    let specifier = node.child_by_field_name("type")?;
    aggregate(&specifier, source)
}

/// A `struct`, `union` or `enum` specifier that both names and defines its
/// type. A top-level specifier's terminator is its sibling, and the item
/// runs through it.
fn aggregate(node: &Node<'_>, source: &str) -> Option<StructureItem> {
    let kind = match node.kind() {
        "struct_specifier" => StructureKind::Struct,
        "union_specifier" => StructureKind::Other("Union".to_string()),
        "enum_specifier" => StructureKind::Enum,
        _ => return None,
    };
    let name = node.child_by_field_name("name")?;
    node.child_by_field_name("body")?;
    let end = node
        .next_sibling()
        .filter(|sibling| sibling.kind() == ";")
        .map_or(node.end_byte(), |terminator| terminator.end_byte());
    Some(item(kind, &name, node, end, source))
}

fn item(kind: StructureKind, name: &Node<'_>, node: &Node<'_>, end: usize, source: &str) -> StructureItem {
    let start = node.start_position();
    let span = span_between_from(source, node.start_byte(), end, (start.row, start.column));
    StructureItem {
        kind,
        name: Some(node_text(name, source).to_string()),
        span,
        ..StructureItem::default()
    }
}
