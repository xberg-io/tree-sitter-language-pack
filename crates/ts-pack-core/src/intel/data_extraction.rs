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

use super::types::{DataAttribute, DataNode, DataNodeKind, Span};
use super::walk::{MAX_TREE_DEPTH, warn_if_truncated};

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

fn extract_toml(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let children = toml_body_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn toml_body_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "pair" => {
                if let Some(n) = toml_pair_node(&child, source, depth + 1, truncated) {
                    result.push(n);
                }
            }
            "table" => {
                if let Some(n) = toml_table_node(&child, source, depth + 1, truncated) {
                    result.push(n);
                }
            }
            "table_array_element" => {
                if let Some(n) = toml_table_array_node(&child, source, depth + 1, truncated) {
                    result.push(n);
                }
            }
            _ => {}
        }
    }
    result
}

fn toml_pair_node(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Option<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return None;
    }
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key_node = &named[0];
    let key = toml_key(key_node, source);
    if named.len() == 1 {
        return Some(DataNode {
            kind: DataNodeKind::KeyValue,
            key: Some(key),
            value: None,
            attributes: vec![],
            children: vec![],
            span: span_from_node(node),
        });
    }
    let val_node = named.iter().skip(1).find(|child| !child.is_extra())?;
    toml_value_node(val_node, source, Some(key), depth + 1, truncated)
}

fn toml_value_node(
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
        "inline_table" => {
            let children = toml_body_children(node, source, depth + 1, truncated);
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
            let mut result = Vec::new();
            let mut cursor = node.walk();
            for (idx, child) in node
                .named_children(&mut cursor)
                .filter(|child| !child.is_extra())
                .enumerate()
            {
                if let Some(n) = toml_value_node(&child, source, Some(idx.to_string()), depth + 1, truncated) {
                    result.push(n);
                }
            }
            Some(DataNode {
                kind: DataNodeKind::Sequence,
                key,
                value: None,
                attributes: vec![],
                children: result,
                span: span_from_node(node),
            })
        }
        _ => Some(DataNode {
            kind: DataNodeKind::KeyValue,
            key,
            value: Some(node_text(node, source).to_string()),
            attributes: vec![],
            children: vec![],
            span: span_from_node(node),
        }),
    }
}

fn toml_table_node(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Option<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return None;
    }
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = toml_key(&named[0], source);
    let children: Vec<DataNode> = named[1..]
        .iter()
        .filter(|c| c.kind() == "pair")
        .filter_map(|c| toml_pair_node(c, source, depth + 1, truncated))
        .collect();
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: Some(key),
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(node),
    })
}

fn toml_table_array_node(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Option<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return None;
    }
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = toml_key(&named[0], source);
    let children: Vec<DataNode> = named[1..]
        .iter()
        .filter(|c| c.kind() == "pair")
        .filter_map(|c| toml_pair_node(c, source, depth + 1, truncated))
        .collect();
    Some(DataNode {
        kind: DataNodeKind::Sequence,
        key: Some(key),
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(node),
    })
}

/// Join a TOML key's segments with `.`, without recursing per segment.
///
/// The grammar nests a `dotted_key` one level per segment, so a valid
/// 10,000-segment key is a 10,000-level subtree. It sits inside a single pair,
/// which is why the depth guard the tree builders share never sees it; a
/// recursive join walked off a 2 MiB stack and aborted the process instead of
/// producing a long string. The explicit stack below lives on the heap, so the
/// key's length is the only cost. ~keep
fn toml_key(node: &Node, source: &str) -> String {
    let mut segments = Vec::new();
    let mut pending = vec![*node];
    while let Some(current) = pending.pop() {
        if current.kind() == "dotted_key" {
            let mut cursor = current.walk();
            let children: Vec<Node> = current.named_children(&mut cursor).collect();
            pending.extend(children.into_iter().rev());
        } else {
            segments.push(strip_quotes(node_text(&current, source)));
        }
    }
    segments.join(".")
}

fn extract_properties(root: &Node, source: &str) -> Option<DataNode> {
    let mut children = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        if child.kind() == "property"
            && let Some(n) = properties_property_node(&child, source)
        {
            children.push(n);
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

fn properties_property_node(node: &Node, source: &str) -> Option<DataNode> {
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    let key = named.first().map(|n| node_text(n, source).to_string());
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

fn extract_hcl(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let children = hcl_body_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn hcl_body_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "attribute" => {
                if let Some(n) = hcl_attribute_node(&child, source) {
                    result.push(n);
                }
            }
            "block" => {
                if let Some(n) = hcl_block_node(&child, source, depth + 1, truncated) {
                    result.push(n);
                }
            }
            "body" => {
                result.extend(hcl_body_children(&child, source, depth + 1, truncated));
            }
            "pair" => {
                if let Some(n) = hocon_pair_node(&child, source) {
                    result.push(n);
                }
            }
            _ => {}
        }
    }
    result
}

fn hcl_attribute_node(node: &Node, source: &str) -> Option<DataNode> {
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = node_text(&named[0], source).to_string();
    let value = named.get(1).map(|n| node_text(n, source).to_string());
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: Some(key),
        value,
        attributes: vec![],
        children: vec![],
        span: span_from_node(node),
    })
}

fn hcl_block_node(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Option<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return None;
    }
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key_parts: Vec<&str> = named
        .iter()
        .filter(|n| n.kind() == "identifier" || n.kind() == "string_lit")
        .map(|n| strip_quotes(node_text(n, source)))
        .collect();
    let key = if key_parts.is_empty() {
        node_text(&named[0], source).to_string()
    } else {
        key_parts.join(".")
    };
    let children = named
        .iter()
        .filter(|n| n.kind() == "body")
        .flat_map(|body| hcl_body_children(body, source, depth + 1, truncated))
        .collect();
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: Some(key),
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(node),
    })
}

fn hocon_pair_node(node: &Node, source: &str) -> Option<DataNode> {
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = node_text(&named[0], source).to_string();
    let value = named.get(1).map(|n| node_text(n, source).to_string());
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: Some(key),
        value,
        attributes: vec![],
        children: vec![],
        span: span_from_node(node),
    })
}

fn extract_kdl(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let children = kdl_node_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn kdl_node_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "node"
            && let Some(n) = kdl_single_node(&child, source, depth + 1, truncated)
        {
            result.push(n);
        }
    }
    result
}

fn kdl_single_node(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Option<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return None;
    }
    let key = node
        .child_by_field_name("name")
        .map(|n| node_text(&n, source).to_string())
        .or_else(|| {
            let mut cursor = node.walk();
            node.named_children(&mut cursor)
                .find(|c| c.kind() == "identifier")
                .map(|c| node_text(&c, source).to_string())
        });

    let mut cursor = node.walk();
    let value = node
        .named_children(&mut cursor)
        .find(|c| {
            !matches!(c.kind(), "identifier" | "children" | "prop")
                && c.kind() != "single_line_comment"
                && c.kind() != "multi_line_comment"
        })
        .map(|v| node_text(&v, source).to_string());

    let sub_children = node
        .child_by_field_name("children")
        .map(|block| kdl_node_children(&block, source, depth + 1, truncated))
        .unwrap_or_default();

    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key,
        value,
        attributes: vec![],
        children: sub_children,
        span: span_from_node(node),
    })
}

fn extract_cue(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    let children = cue_body_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn cue_body_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "field" => {
                if let Some(n) = cue_field_node(&child, source) {
                    result.push(n);
                }
            }
            "struct_lit" | "source_file" => {
                result.extend(cue_body_children(&child, source, depth + 1, truncated));
            }
            _ => {}
        }
    }
    result
}

fn cue_field_node(node: &Node, source: &str) -> Option<DataNode> {
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = named
        .iter()
        .find(|n| n.kind() == "label" || n.kind() == "identifier" || n.kind() == "string")
        .map(|n| strip_quotes(node_text(n, source)).to_string());
    let value = named
        .iter()
        .find(|n| n.kind() == "value" || n.kind() == "expression")
        .map(|n| node_text(n, source).to_string());
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key,
        value,
        attributes: vec![],
        children: vec![],
        span: span_from_node(node),
    })
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

fn extract_xml(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
    if let Some(top) = plist_top_value(root, source) {
        return Some(extract_plist(root, &top, source, truncated));
    }
    let children = xml_node_children(root, source, 0, truncated);
    Some(DataNode {
        kind: DataNodeKind::Element,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn xml_node_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "element" => {
                if let Some(n) = xml_element_node(&child, source, depth + 1, truncated) {
                    result.push(n);
                }
            }
            "document" | "content" => {
                result.extend(xml_node_children(&child, source, depth + 1, truncated));
            }
            _ => {}
        }
    }
    result
}

fn xml_element_node(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Option<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return None;
    }
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();

    let (tag_name, attributes) = named
        .iter()
        .find(|c| c.kind() == "STag" || c.kind() == "EmptyElemTag")
        .map(|stag| {
            let name = named_child_of_kind(stag, "Name")
                .map(|n| node_text(&n, source).to_string())
                .unwrap_or_default();
            let attrs = xml_attributes(stag, source);
            (name, attrs)
        })
        .unwrap_or_default();

    // ~keep The whole text run, not its first node: an entity reference or a
    // ~keep CDATA section splits it. Trimmed here, unlike a property list
    // ~keep scalar, because an element holding only layout whitespace between
    // ~keep its children carries no value.
    let text_value = xml_text(node, source)
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty());

    let children: Vec<DataNode> = named
        .iter()
        .filter(|c| c.kind() == "content")
        .flat_map(|content| xml_node_children(content, source, depth + 1, truncated))
        .collect();

    Some(DataNode {
        kind: DataNodeKind::Element,
        key: if tag_name.is_empty() { None } else { Some(tag_name) },
        value: text_value,
        attributes,
        children,
        span: span_from_node(node),
    })
}

/// The single value element inside a `<plist>` root, when the document is a
/// property list. Decided by the root tag, not the file name: a plist is XML
/// whose meaning lives in its `<key>` text rather than its tag names, so it
/// is read as keys and values wherever it appears.
fn plist_top_value<'a>(root: &Node<'a>, source: &str) -> Option<Node<'a>> {
    let element = root.child_by_field_name("root")?;
    if xml_tag_name(&element, source)? != "plist" {
        return None;
    }
    let content = named_child_of_kind(&element, "content")?;
    named_child_of_kind(&content, "element")
}

/// A property list as keys and values. A `dict` entry is keyed by the text
/// of its `<key>` element and spans from that key through its value; an
/// `array` item is keyed by its position. The containers themselves add no
/// segment, so a dict at the top reads like a JSON object and an array at
/// the top like a JSON array. `<string>`, `<integer>`, `<real>`, `<date>`
/// and `<data>` carry their text as the value; `<true/>` and `<false/>`
/// carry their tag.
fn extract_plist(root: &Node, top: &Node, source: &str, truncated: &mut usize) -> DataNode {
    let value = plist_value_node(top, None, DataNodeKind::KeyValue, source, 0, truncated);
    DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: value.value,
        attributes: vec![],
        children: value.children,
        span: span_from_node(root),
    }
}

fn plist_value_node(
    node: &Node,
    key: Option<String>,
    kind: DataNodeKind,
    source: &str,
    depth: usize,
    truncated: &mut usize,
) -> DataNode {
    let tag = xml_tag_name(node, source).unwrap_or_default();
    let (value, children) = match tag.as_str() {
        "dict" => (None, plist_dict_children(node, source, depth + 1, truncated)),
        "array" => (None, plist_array_children(node, source, depth + 1, truncated)),
        "true" | "false" => (Some(tag.clone()), vec![]),
        // ~keep A `<string>` carries its whitespace: " padded " is that value,
        // ~keep not "padded". The other scalars are a number or a date, which
        // ~keep an XML writer is free to indent onto its own line, so those
        // ~keep trim.
        "string" => (Some(xml_text(node, source).unwrap_or_default()), vec![]),
        _ => (Some(plist_trimmed_text(node, source)), vec![]),
    };
    DataNode {
        kind,
        key,
        value,
        attributes: vec![],
        children,
        span: span_from_node(node),
    }
}

/// An element's text with surrounding layout whitespace removed, for the
/// plist parts that name rather than carry a value: a `<key>`, and the
/// scalars an XML writer may indent onto their own line.
fn plist_trimmed_text(node: &Node, source: &str) -> String {
    xml_text(node, source).map_or_else(String::new, |text| text.trim().to_string())
}

fn plist_dict_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    let mut result = Vec::new();
    if depth_exceeded(node, depth, truncated) {
        return result;
    }
    let elements = plist_elements(node);
    let mut index = 0;
    while index + 1 < elements.len() {
        let key = &elements[index];
        if xml_tag_name(key, source).as_deref() != Some("key") {
            index += 1;
            continue;
        }
        let value = &elements[index + 1];
        let mut entry = plist_value_node(
            value,
            Some(plist_trimmed_text(key, source)),
            DataNodeKind::KeyValue,
            source,
            depth,
            truncated,
        );
        let start = key.start_position();
        entry.span.start_byte = key.start_byte();
        entry.span.start_line = start.row;
        entry.span.start_column = start.column;
        result.push(entry);
        index += 2;
    }
    result
}

fn plist_array_children(node: &Node, source: &str, depth: usize, truncated: &mut usize) -> Vec<DataNode> {
    if depth_exceeded(node, depth, truncated) {
        return Vec::new();
    }
    plist_elements(node)
        .iter()
        .enumerate()
        .map(|(index, item)| {
            plist_value_node(
                item,
                Some(index.to_string()),
                DataNodeKind::Sequence,
                source,
                depth,
                truncated,
            )
        })
        .collect()
}

/// The element children of an element's content, in order.
fn plist_elements<'a>(node: &Node<'a>) -> Vec<Node<'a>> {
    named_child_of_kind(node, "content")
        .map(|content| {
            let mut cursor = content.walk();
            content
                .named_children(&mut cursor)
                .filter(|c| c.kind() == "element")
                .collect()
        })
        .unwrap_or_default()
}

fn xml_tag_name(node: &Node, source: &str) -> Option<String> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|c| c.kind() == "STag" || c.kind() == "EmptyElemTag")
        .and_then(|stag| named_child_of_kind(&stag, "Name"))
        .map(|name| node_text(&name, source).to_string())
}

/// The complete text of an element, in source order.
///
/// A text run is not one node. An entity reference (`&amp;`), a character
/// reference (`&#65;`) and a CDATA section each interrupt it, so `a&amp;b`
/// arrives as `CharData EntityRef CharData` and reading the first child alone
/// truncates the value at the first `&`. CDATA is a `CData` beneath a
/// `CDSect`, never a direct child of `content`, so a direct-child search for
/// it finds nothing and the value reads empty.
///
/// The text is returned as written, untrimmed: whitespace is significant in a
/// property list `<string>`, and the caller decides whether it matters.
fn xml_text(node: &Node, source: &str) -> Option<String> {
    let content = named_child_of_kind(node, "content")?;
    let mut cursor = content.walk();
    let mut text = String::new();
    for part in content.named_children(&mut cursor) {
        append_xml_text(&part, source, &mut text);
    }
    Some(text)
}

/// Append one `content` child's contribution to `text`. Nested one level for
/// `CDSect`, which is as deep as the grammar puts character data; nothing
/// here recurses on tree depth.
fn append_xml_text(part: &Node, source: &str, text: &mut String) {
    match part.kind() {
        "CharData" | "CData" => text.push_str(node_text(part, source)),
        "EntityRef" | "CharRef" => text.push_str(&resolve_xml_reference(node_text(part, source))),
        "CDSect" => {
            let mut cursor = part.walk();
            for inner in part.named_children(&mut cursor) {
                if inner.kind() == "CData" {
                    text.push_str(node_text(&inner, source));
                }
            }
        }
        _ => {}
    }
}

/// The character an XML reference stands for.
///
/// The five predefined entities and numeric character references resolve. Any
/// other entity is declared in a DTD this reader does not read, so it is kept
/// as written rather than dropped, which would silently lose text.
fn resolve_xml_reference(reference: &str) -> String {
    match reference {
        "&amp;" => return "&".to_string(),
        "&lt;" => return "<".to_string(),
        "&gt;" => return ">".to_string(),
        "&quot;" => return "\"".to_string(),
        "&apos;" => return "'".to_string(),
        _ => {}
    }
    reference
        .strip_prefix("&#")
        .and_then(|rest| rest.strip_suffix(';'))
        .and_then(|digits| match digits.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok(),
            None => digits.parse::<u32>().ok(),
        })
        .and_then(char::from_u32)
        .map_or_else(|| reference.to_string(), String::from)
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

fn xml_attributes(stag: &Node, source: &str) -> Vec<DataAttribute> {
    let mut result = Vec::new();
    let mut cursor = stag.walk();
    for child in stag.named_children(&mut cursor) {
        if child.kind() == "Attribute" {
            let mut c2 = child.walk();
            let named: Vec<Node> = child.named_children(&mut c2).collect();
            if named.len() >= 2 {
                let name = node_text(&named[0], source).to_string();
                let value = strip_quotes(node_text(&named[1], source)).to_string();
                result.push(DataAttribute {
                    name,
                    value,
                    span: span_from_node(&child),
                });
            }
        }
    }
    result
}

fn extract_dtd(root: &Node, source: &str) -> Option<DataNode> {
    let mut children = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        let kind = child.kind();
        if kind == "ElementDecl" || kind == "AttlistDecl" || kind == "GEDecl" || kind == "PEDecl" {
            let mut c2 = child.walk();
            let first_named = child.named_children(&mut c2).next();
            let key = first_named.map(|n| node_text(&n, source).to_string());
            children.push(DataNode {
                kind: DataNodeKind::KeyValue,
                key,
                value: Some(node_text(&child, source).to_string()),
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
mod tests {
    use super::*;

    // ~keep A missing grammar reports `SKIPPED` on stderr; see `intel::test_support`.
    fn parse(source: &str, lang: &str) -> Option<tree_sitter::Tree> {
        crate::intel::test_support::parse_or_skip(source, lang)
    }

    /// Extract, or skip when the grammar is missing.
    ///
    /// ~keep A present grammar that yields no data tree is a defect, not a skip:
    /// ~keep it fails here instead of leaving the caller's `else { return }` silent.
    fn extract(source: &str, lang: &str) -> Option<DataNode> {
        let tree = parse(source, lang)?;
        let extracted = extract_data(&tree.root_node(), source, lang);
        assert!(
            extracted.is_some(),
            "'{lang}' is loaded but produced no data tree for {source:?}"
        );
        extracted
    }

    #[test]
    fn should_not_panic_when_a_quoted_token_is_unterminated() {
        assert_eq!(strip_quotes("\""), "\"", "a lone quote has nothing to strip");
        assert_eq!(strip_quotes("'"), "'");
        assert_eq!(strip_quotes(""), "");
        assert_eq!(strip_quotes("\"\""), "");
        assert_eq!(strip_quotes("\"a\""), "a");
        assert_eq!(strip_quotes("bare"), "bare");
    }

    #[test]
    fn should_extract_a_truncated_po_file_without_panicking() {
        let source = "msgid \"a\"\nmsgstr \"\n";
        let Some(tree) = parse(source, "po") else {
            return;
        };
        let extracted = extract_data(&tree.root_node(), source, "po");
        assert!(
            extracted.is_some(),
            "a truncated translation file must still yield a data tree"
        );
    }

    #[test]
    fn test_json_flat_object() {
        let source = r#"{"host": "localhost", "port": 8080}"#;
        let Some(root) = extract(source, "json") else { return };
        assert_eq!(root.kind, DataNodeKind::KeyValue);
        assert_eq!(root.children.len(), 2);
        let host = root.children.iter().find(|c| c.key.as_deref() == Some("host"));
        assert!(host.is_some(), "should find 'host' key");
        assert_eq!(host.unwrap().value.as_deref(), Some(r#""localhost""#));
    }

    #[test]
    fn test_json_nested_object() {
        let source = r#"{"server": {"host": "x", "port": 8080}}"#;
        let Some(root) = extract(source, "json") else { return };
        let server = root.children.iter().find(|c| c.key.as_deref() == Some("server"));
        assert!(server.is_some());
        let server = server.unwrap();
        assert!(!server.children.is_empty(), "server should have children");
    }

    #[test]
    fn test_json_array() {
        let source = r#"[1, 2, 3]"#;
        let Some(root) = extract(source, "json") else { return };
        assert_eq!(root.kind, DataNodeKind::Sequence);
        assert_eq!(root.children.len(), 3);
        assert_eq!(root.children[0].key.as_deref(), Some("0"));
    }

    #[test]
    fn test_toml_flat() {
        let source = "host = \"localhost\"\nport = 8080\n";
        let Some(root) = extract(source, "toml") else { return };
        assert!(
            root.children.iter().any(|c| c.key.as_deref() == Some("host")),
            "should find 'host'"
        );
        assert!(
            root.children.iter().any(|c| c.key.as_deref() == Some("port")),
            "should find 'port'"
        );
    }

    #[test]
    fn test_toml_table() {
        let source = "[server]\nhost = \"localhost\"\n";
        let Some(root) = extract(source, "toml") else { return };
        let server = root.children.iter().find(|c| c.key.as_deref() == Some("server"));
        assert!(server.is_some(), "should find [server] table");
    }

    #[test]
    fn test_properties_flat() {
        let source = "host=localhost\nport=8080\n";
        let Some(root) = extract(source, "properties") else {
            return;
        };
        assert!(root.children.iter().any(|c| c.key.as_deref() == Some("host")));
        assert!(root.children.iter().any(|c| c.key.as_deref() == Some("port")));
    }

    #[test]
    fn test_yaml_flat() {
        let source = "host: localhost\nport: 8080\n";
        let Some(root) = extract(source, "yaml") else { return };
        assert!(root.children.iter().any(|c| c.key.as_deref() == Some("host")));
    }

    #[test]
    fn test_yaml_nested() {
        let source = "server:\n  host: localhost\n  port: 8080\n";
        let Some(root) = extract(source, "yaml") else { return };
        let server = root.children.iter().find(|c| c.key.as_deref() == Some("server"));
        assert!(server.is_some(), "should find nested server key");
    }

    #[test]
    fn test_csv_rows() {
        let source = "a,b,c\n1,2,3\n";
        let Some(root) = extract(source, "csv") else { return };
        assert_eq!(root.kind, DataNodeKind::Sequence);
        assert!(!root.children.is_empty(), "should have rows");
        let row0 = &root.children[0];
        assert_eq!(row0.kind, DataNodeKind::Sequence);
        assert_eq!(row0.children.len(), 3);
        assert_eq!(row0.children[0].key.as_deref(), Some("0"));
    }

    #[test]
    fn test_ini_flat() {
        let source = "host=localhost\nport=8080\n";
        let Some(root) = extract(source, "ini") else { return };
        assert!(!root.children.is_empty(), "should have settings");
    }

    #[test]
    fn test_unsupported_language_returns_none() {
        let source = "x = 1";
        let Some(tree) = parse(source, "python") else {
            return;
        };
        let result = extract_data(&tree.root_node(), source, "python");
        assert!(result.is_none(), "python should return None for data extraction");
    }
}
