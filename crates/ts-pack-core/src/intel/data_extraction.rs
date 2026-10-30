//! Hierarchical data extraction for data-format grammars.
//!
//! Entry point: [`extract_data`]. Dispatches on the language name and returns
//! a [`DataNode`] tree that mirrors the parsed file structure. Returns `None`
//! for languages outside the cut-1 support set — no error, no panic.
//!
//! # Supported languages (cut-1)
//!
//! **Bucket A — field-named pair grammars:**
//! `json`, `hjson`, `json5`, `toml`, `properties`, `hcl`, `terraform`, `hocon`, `kdl`
//!
//! **Bucket B — positional / two-child grammars:**
//! `yaml`, `ini`, `editorconfig`, `csv`, `psv`, `po`, `nginx`, `caddy`
//!
//! **Bucket C — element shape:**
//! `xml`, `dtd`
//!
//! **CUE:** uses `field` containing a `label` + `value` pair.
//!
//! **ndjson/jsonl:** not a separate grammar in this pack; users should split
//! on `\n` and call `process()` per line with `language = "json"`.

use tree_sitter::Node;

use super::types::{DataNode, DataNodeKind, Span};
use super::walk::{MAX_TREE_DEPTH, warn_if_truncated};

mod configuration;
mod xml;

use configuration::{extract_cue, extract_hcl, extract_kdl, extract_properties, extract_toml};
use xml::{extract_dtd, extract_xml};

/// Whether `depth` has reached the traversal limit, in which case `node`'s
/// subtree is dropped and counted.
///
/// These per-format builders mirror the shape of the data they produce, so they
/// stay recursive; the guard is what keeps the recursion off the stack cliff.
/// See [`super::walk`] for how [`MAX_TREE_DEPTH`] was chosen.
fn depth_exceeded(node: &Node, depth: usize, truncated: &mut usize) -> bool {
    if depth < MAX_TREE_DEPTH {
        return false;
    }
    *truncated += node.descendant_count();
    true
}

/// Extract a hierarchical data tree from a parsed data-format source file.
///
/// Returns `None` when the language is not in the supported cut-1 set, so
/// callers can leave [`ProcessResult::data`](crate::ProcessResult::data) absent
/// without any error.
///
/// # Arguments
///
/// * `root` — The root [`Node`] of the tree-sitter parse tree.
/// * `source` — The original source text (used for byte-range slices).
/// * `language` — Language name as recognised by the registry (e.g. `"json"`).
pub(crate) fn extract_data(root: &Node, source: &str, language: &str) -> Option<DataNode> {
    let truncated = &mut 0usize;
    let extracted = match language {
        "json" | "hjson" | "json5" => extract_json(root, source, truncated),
        "toml" => extract_toml(root, source, truncated),
        "properties" => extract_properties(root, source),
        "hcl" | "terraform" | "hocon" => extract_hcl(root, source, truncated),
        "kdl" => extract_kdl(root, source, truncated),
        "cue" => extract_cue(root, source, truncated),
        "yaml" => extract_yaml(root, source, truncated),
        "ini" | "editorconfig" => extract_ini(root, source, truncated),
        "csv" | "psv" => extract_csv(root, source),
        "po" => extract_po(root, source),
        "nginx" => extract_nginx(root, source, truncated),
        "caddy" => extract_caddy(root, source, truncated),
        "xml" => extract_xml(root, source, truncated),
        "dtd" => extract_dtd(root, source),
        "dotenv" => extract_dotenv(root, source),
        _ => None,
    };
    warn_if_truncated(*truncated, "intel::data_extraction", language);
    // ~keep `produced = false` is the normal answer for a non-data language, not a fault:
    // ~keep callers routinely enable data extraction across a mixed-language tree.
    tracing::debug!(
        target: "ts_pack::intel",
        operation = "intel::data_extraction",
        language,
        produced = extracted.is_some(),
        "data extraction complete"
    );
    extracted
}

fn span_from_node(node: &Node) -> Span {
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

fn node_text<'a>(node: &Node, source: &'a str) -> &'a str {
    let start = node.start_byte().min(source.len());
    let end = node.end_byte().min(source.len());
    &source[start..end]
}

/// Shortest string that can carry both an opening and a closing quote.
const MIN_QUOTED_LEN: usize = 2;

/// Strip one layer of surrounding quotes from a string (JSON, YAML, etc.).
fn strip_quotes(s: &str) -> &str {
    let s = s.trim();
    // ~keep A truncated file yields lone quote tokens (`msgstr "`); `1..len-1` would be `1..0`.
    if s.len() < MIN_QUOTED_LEN {
        return s;
    }
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

/// Collect the first named child of a given `kind` from `node`.
fn named_child_of_kind<'a>(node: &Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor).find(|c| c.kind() == kind)
}

fn extract_json(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let mut cursor = root.walk();
    let mut result = None;
    for child in root.named_children(&mut cursor) {
        if let Some(node) = json_value_node(&child, source, None, 0, truncated) {
            // The grammar accepts concatenated values, but they are not one JSON document.
            if result.is_some() {
                return None;
            }
            result = Some(node);
        }
    }
    if result.is_some() {
        return result;
    }
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children: vec![],
        span: span_from_node(root),
    })
}

fn json_value_node(
    node: &Node,
    source: &str,
    key: Option<String>,
    depth: usize,
    truncated: &mut usize,
) -> Option<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return None;
    }
    match node.kind() {
        "object" => {
            let children = json_object_children(node, source, depth + 1, truncated);
            Some(DataNode {
                kind: DataNodeKind::KeyValue,
                key,
                value: None,
                attributes: vec![],
                children,
                span: span_from_node(node),
            })
        }
        "array" => {
            let children = json_array_children(node, source, depth + 1, truncated);
            Some(DataNode {
                kind: DataNodeKind::Sequence,
                key,
                value: None,
                attributes: vec![],
                children,
                span: span_from_node(node),
            })
        }
        "pair" => {
            let k = node
                .child_by_field_name("key")
                .map(|n| strip_quotes(node_text(&n, source)).to_string());
            let v_node = node.child_by_field_name("value");
            if let Some(v) = v_node {
                let mut value = json_value_node(&v, source, k, depth + 1, truncated)?;
                value.span = span_from_node(node);
                Some(value)
            } else {
                None
            }
        }
        "string" | "number" | "true" | "false" | "null" => Some(DataNode {
            kind: DataNodeKind::KeyValue,
            key,
            value: Some(node_text(node, source).to_string()),
            attributes: vec![],
            children: vec![],
            span: span_from_node(node),
        }),
        _ => None,
    }
}

fn json_object_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "pair"
            && let Some(n) = json_value_node(&child, source, None, depth, truncated)
        {
            result.push(n);
        }
    }
    result
}

fn json_array_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for (idx, child) in node.named_children(&mut cursor).enumerate() {
        let key = Some(idx.to_string());
        if let Some(n) = json_value_node(&child, source, key, depth, truncated) {
            result.push(n);
        }
    }
    result
}

fn extract_yaml(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let children = yaml_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn yaml_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "block_mapping_pair" | "flow_pair" => {
                if let Some(n) = yaml_mapping_pair(&child, source, depth + 1, truncated) {
                    result.push(n);
                }
            }
            "block_mapping" | "flow_mapping" => {
                result.extend(yaml_children(&child, source, depth + 1, truncated));
            }
            "block_sequence" | "flow_sequence" => {
                let items = yaml_sequence_items(&child, source, depth + 1, truncated);
                result.extend(items);
            }
            "document" | "block_node" | "flow_node" => {
                result.extend(yaml_children(&child, source, depth + 1, truncated));
            }
            "stream" => {
                result.extend(yaml_children(&child, source, depth + 1, truncated));
            }
            _ => {}
        }
    }
    result
}

fn yaml_mapping_pair(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Option<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return None;
    }
    let key_node = node.child_by_field_name("key");
    let val_node = node.child_by_field_name("value");

    // A structured YAML key has no scalar path. Copying its source into key
    // would misclassify nested values as key metadata.
    let key = Some(yaml_scalar_key(&key_node?, source)?);

    if let Some(val) = val_node {
        let val_kind = val.kind();
        if val_kind == "block_node" || val_kind == "flow_node" {
            let sub = yaml_children(&val, source, depth + 1, truncated);
            if !sub.is_empty() {
                return Some(DataNode {
                    kind: DataNodeKind::KeyValue,
                    key,
                    value: None,
                    attributes: vec![],
                    children: sub,
                    span: span_from_node(node),
                });
            }
        }
        let value = Some(yaml_scalar_text(&val, source));
        return Some(DataNode {
            kind: DataNodeKind::KeyValue,
            key,
            value,
            attributes: vec![],
            children: vec![],
            span: span_from_node(node),
        });
    }

    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key,
        value: None,
        attributes: vec![],
        children: vec![],
        span: span_from_node(node),
    })
}

/// The path segment a YAML key contributes, or `None` for a structured key.
///
/// An alias key (`*name : value`) is kept as its alias text: its source holds
/// no nested values to leak into the key, and dropping the pair would lose the
/// value it maps to.
fn yaml_scalar_key(node: &Node, source: &str) -> Option<String> {
    match node.kind() {
        "plain_scalar" | "single_quote_scalar" | "double_quote_scalar" | "block_scalar" | "alias" => {
            Some(strip_quotes(node_text(node, source)).to_string())
        }
        "flow_node" | "block_node" => {
            let mut cursor = node.walk();
            node.named_children(&mut cursor)
                .find_map(|child| yaml_scalar_key(&child, source))
        }
        _ => None,
    }
}

/// The scalar text of a YAML value, without the anchor or tag that may precede it.
///
/// `name: &key value` wraps the scalar in a `flow_node` whose first named child
/// is the anchor; the value is the content beside it, not the whole span.
fn yaml_scalar_text(node: &Node, source: &str) -> String {
    let mut cursor = node.walk();
    let content = if matches!(node.kind(), "flow_node" | "block_node") {
        node.named_children(&mut cursor)
            .find(|child| !matches!(child.kind(), "anchor" | "tag"))
    } else {
        None
    };
    strip_quotes(node_text(content.as_ref().unwrap_or(node), source)).to_string()
}

fn yaml_sequence_items(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for (idx, child) in node
        .named_children(&mut cursor)
        .filter(|child| matches!(child.kind(), "block_sequence_item" | "flow_node" | "flow_pair"))
        .enumerate()
    {
        let sub = if child.kind() == "flow_pair" {
            yaml_mapping_pair(&child, source, depth + 1, truncated)
                .into_iter()
                .collect()
        } else {
            yaml_children(&child, source, depth + 1, truncated)
        };
        let value = if sub.is_empty() {
            let mut c2 = child.walk();
            let content = if child.kind() == "flow_node" {
                Some(child)
            } else {
                child.named_children(&mut c2).next()
            };
            content.map(|n| yaml_scalar_text(&n, source))
        } else {
            None
        };
        result.push(DataNode {
            kind: DataNodeKind::Sequence,
            key: Some(idx.to_string()),
            value,
            attributes: vec![],
            children: sub,
            span: span_from_node(&child),
        });
    }
    result
}

fn extract_ini(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let children = ini_top_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn ini_top_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "section" => {
                if let Some(n) = ini_section_node(&child, source) {
                    result.push(n);
                }
            }
            "setting" | "property" | "pair" => {
                if let Some(n) = ini_setting_node(&child, source) {
                    result.push(n);
                }
            }
            "preamble" => {
                result.extend(ini_top_children(&child, source, depth + 1, truncated));
            }
            _ => {}
        }
    }
    result
}

fn ini_section_node(node: &Node, source: &str) -> Option<DataNode> {
    let key = named_child_of_kind(node, "section_name")
        .or_else(|| named_child_of_kind(node, "glob"))
        .map(|n| node_text(&n, source).to_string());
    let mut cursor = node.walk();
    let children = node
        .named_children(&mut cursor)
        .filter(|c| c.kind() == "setting" || c.kind() == "property" || c.kind() == "pair")
        .filter_map(|c| ini_setting_node(&c, source))
        .collect();
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(node),
    })
}

fn ini_setting_node(node: &Node, source: &str) -> Option<DataNode> {
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = Some(node_text(&named[0], source).to_string());
    let value = named.get(1).map(|n| node_text(n, source).to_string());
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key,
        value,
        attributes: vec![],
        children: vec![],
        span: span_from_node(node),
    })
}

fn extract_csv(root: &Node, source: &str) -> Option<DataNode> {
    let mut rows = Vec::new();
    let mut cursor = root.walk();
    for (row_idx, child) in root.named_children(&mut cursor).enumerate() {
        if child.kind() == "row" {
            let cells = csv_row_cells(&child, source);
            rows.push(DataNode {
                kind: DataNodeKind::Sequence,
                key: Some(row_idx.to_string()),
                value: None,
                attributes: vec![],
                children: cells,
                span: span_from_node(&child),
            });
        }
    }
    Some(DataNode {
        kind: DataNodeKind::Sequence,
        key: None,
        value: None,
        attributes: vec![],
        children: rows,
        span: span_from_node(root),
    })
}

fn csv_row_cells(row: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = row.walk();
    for (col_idx, child) in row.named_children(&mut cursor).enumerate() {
        if child.kind() == "field" {
            let text = {
                let mut c2 = child.walk();
                child
                    .named_children(&mut c2)
                    .next()
                    .map(|n| node_text(&n, source).to_string())
                    .unwrap_or_else(|| node_text(&child, source).to_string())
            };
            result.push(DataNode {
                kind: DataNodeKind::Sequence,
                key: Some(col_idx.to_string()),
                value: Some(text),
                attributes: vec![],
                children: vec![],
                span: span_from_node(&child),
            });
        }
    }
    result
}

fn extract_po(root: &Node, source: &str) -> Option<DataNode> {
    let mut messages = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        if child.kind() == "message"
            && let Some(n) = po_message_node(&child, source)
        {
            messages.push(n);
        }
    }
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children: messages,
        span: span_from_node(root),
    })
}

fn po_message_node(node: &Node, source: &str) -> Option<DataNode> {
    let msgid = named_child_of_kind(node, "msgid")
        .and_then(|n| {
            let mut c = n.walk();
            n.named_children(&mut c).find(|c| c.kind() == "string")
        })
        .map(|n| strip_quotes(node_text(&n, source)).to_string());

    let msgstr = named_child_of_kind(node, "msgstr")
        .and_then(|n| {
            let mut c = n.walk();
            n.named_children(&mut c).find(|c| c.kind() == "string")
        })
        .map(|n| strip_quotes(node_text(&n, source)).to_string());

    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: msgid,
        value: msgstr,
        attributes: vec![],
        children: vec![],
        span: span_from_node(node),
    })
}

fn extract_nginx(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let children = nginx_body_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn nginx_body_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "directive" => {
                if let Some(n) = nginx_directive_node(&child, source) {
                    result.push(n);
                }
            }
            "block" | "http" | "events" | "server" | "location" | "map" | "if" => {
                result.extend(nginx_body_children(&child, source, depth + 1, truncated));
            }
            _ => {}
        }
    }
    result
}

fn nginx_directive_node(node: &Node, source: &str) -> Option<DataNode> {
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = node_text(&named[0], source).to_string();
    let value = if named.len() > 1 {
        let args: Vec<&str> = named[1..].iter().map(|n| node_text(n, source)).collect();
        Some(args.join(" "))
    } else {
        None
    };
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: Some(key),
        value,
        attributes: vec![],
        children: vec![],
        span: span_from_node(node),
    })
}

fn extract_caddy(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let children = caddy_body_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn caddy_body_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        let kind = child.kind();
        if kind.starts_with("directive_") || kind == "directive" {
            if let Some(n) = caddy_directive_node(&child, source) {
                result.push(n);
            }
        } else if kind == "server" || kind == "route" || kind == "block" {
            result.extend(caddy_body_children(&child, source, depth + 1, truncated));
        }
    }
    result
}

fn caddy_directive_node(node: &Node, source: &str) -> Option<DataNode> {
    let key = node
        .child_by_field_name("directive_type")
        .map(|n| node_text(&n, source).to_string())
        .unwrap_or_else(|| {
            let mut cursor = node.walk();
            node.named_children(&mut cursor)
                .next()
                .map(|n| node_text(&n, source).to_string())
                .unwrap_or_default()
        });

    let mut cursor = node.walk();
    let args: Vec<String> = node
        .named_children(&mut cursor)
        .filter(|c| c.kind() != "directive_type" && c.child_count() == 0)
        .map(|c| node_text(&c, source).to_string())
        .collect();
    let value = if args.is_empty() { None } else { Some(args.join(" ")) };

    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: Some(key),
        value,
        attributes: vec![],
        children: vec![],
        span: span_from_node(node),
    })
}

/// A dotenv file is a flat list of `KEY=value` lines, so every assignment is
/// a keyed scalar and nothing nests. The value is carried as written, quotes
/// and placeholders included; an assignment with nothing after `=` carries
/// an empty value rather than none, so it still reads as a scalar.
fn extract_dotenv(root: &Node, source: &str) -> Option<DataNode> {
    let mut children = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        if child.kind() == "assignment"
            && let Some(key) = child.child_by_field_name("key")
        {
            let value = child
                .child_by_field_name("value")
                .map_or_else(String::new, |value| node_text(&value, source).to_string());
            children.push(DataNode {
                kind: DataNodeKind::KeyValue,
                key: Some(node_text(&key, source).to_string()),
                value: Some(value),
                attributes: vec![],
                children: vec![],
                span: span_from_node(&child),
            });
        }
    }
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

#[cfg(test)]
mod tests;
