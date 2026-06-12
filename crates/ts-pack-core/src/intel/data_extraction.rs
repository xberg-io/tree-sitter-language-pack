//! Hierarchical data extraction for data-format grammars.
//!
//! Entry point: [`extract_data`]. Dispatches on the language name and returns
//! a [`DataNode`] tree that mirrors the parsed file structure. Returns `None`
//! for languages outside the cut-1 support set — no error, no panic.
//!
//! # Supported languages (cut-1)
//!
//! **Bucket A — field-named pair grammars:**
//! `json`, `hjson`, `json5`, `toml`, `properties`, `hcl`, `hocon`, `kdl`
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
    match language {
        "json" | "hjson" | "json5" => extract_json(root, source),
        "toml" => extract_toml(root, source),
        "properties" => extract_properties(root, source),
        "hcl" | "hocon" => extract_hcl(root, source),
        "kdl" => extract_kdl(root, source),
        "cue" => extract_cue(root, source),
        "yaml" => extract_yaml(root, source),
        "ini" | "editorconfig" => extract_ini(root, source),
        "csv" | "psv" => extract_csv(root, source),
        "po" => extract_po(root, source),
        "nginx" => extract_nginx(root, source),
        "caddy" => extract_caddy(root, source),
        "xml" => extract_xml(root, source),
        "dtd" => extract_dtd(root, source),
        _ => None,
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

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

/// Strip one layer of surrounding quotes from a string (JSON, YAML, etc.).
fn strip_quotes(s: &str) -> &str {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

/// Collect the first named child of a given `kind` from `node`.
fn named_child_of_kind<'a>(node: &Node<'a>, kind: &str) -> Option<Node<'a>> {
    let mut cursor = node.walk();
    node.named_children(&mut cursor)
        .find(|c| c.kind() == kind)
        .map(|c| c)
}

// ── Bucket A: JSON / HJSON / JSON5 ───────────────────────────────────────────

fn extract_json(root: &Node, source: &str) -> Option<DataNode> {
    // Root is `document` or `object` or `array` depending on the grammar.
    // Walk directly into the first named child value node.
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        let node = json_value_node(&child, source, None);
        if node.is_some() {
            return node;
        }
    }
    // Fallback: treat root itself.
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children: vec![],
        span: span_from_node(root),
    })
}

fn json_value_node(node: &Node, source: &str, key: Option<String>) -> Option<DataNode> {
    match node.kind() {
        "object" => {
            let children = json_object_children(node, source);
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
            let children = json_array_children(node, source);
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
            // json: pair { key, value }
            let k = node
                .child_by_field_name("key")
                .map(|n| strip_quotes(node_text(&n, source)).to_string());
            let v_node = node.child_by_field_name("value");
            if let Some(v) = v_node {
                json_value_node(&v, source, k)
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

fn json_object_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "pair" {
            if let Some(n) = json_value_node(&child, source, None) {
                result.push(n);
            }
        }
    }
    result
}

fn json_array_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for (idx, child) in node.named_children(&mut cursor).enumerate() {
        let key = Some(idx.to_string());
        if let Some(n) = json_value_node(&child, source, key) {
            result.push(n);
        }
    }
    result
}

// ── Bucket A: TOML ───────────────────────────────────────────────────────────

fn extract_toml(root: &Node, source: &str) -> Option<DataNode> {
    let children = toml_body_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn toml_body_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "pair" => {
                if let Some(n) = toml_pair_node(&child, source) {
                    result.push(n);
                }
            }
            "table" => {
                if let Some(n) = toml_table_node(&child, source) {
                    result.push(n);
                }
            }
            "table_array_element" => {
                if let Some(n) = toml_table_array_node(&child, source) {
                    result.push(n);
                }
            }
            _ => {}
        }
    }
    result
}

fn toml_pair_node(node: &Node, source: &str) -> Option<DataNode> {
    // TOML pair: positional children — first is key, last is value
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key_node = &named[0];
    let key = node_text(key_node, source).to_string();
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
    let val_node = &named[named.len() - 1];
    toml_value_node(val_node, source, Some(key))
}

fn toml_value_node(node: &Node, source: &str, key: Option<String>) -> Option<DataNode> {
    match node.kind() {
        "inline_table" => {
            let children = toml_body_children(node, source);
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
            for (idx, child) in node.named_children(&mut cursor).enumerate() {
                if let Some(n) = toml_value_node(&child, source, Some(idx.to_string())) {
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

fn toml_table_node(node: &Node, source: &str) -> Option<DataNode> {
    // TOML table: first named child is the key; remaining children are pairs.
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = node_text(&named[0], source).to_string();
    let children: Vec<DataNode> = named[1..]
        .iter()
        .filter(|c| c.kind() == "pair")
        .filter_map(|c| toml_pair_node(c, source))
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

fn toml_table_array_node(node: &Node, source: &str) -> Option<DataNode> {
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    let key = node_text(&named[0], source).to_string();
    let children: Vec<DataNode> = named[1..]
        .iter()
        .filter(|c| c.kind() == "pair")
        .filter_map(|c| toml_pair_node(c, source))
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

// ── Bucket A: Properties ──────────────────────────────────────────────────────

fn extract_properties(root: &Node, source: &str) -> Option<DataNode> {
    let mut children = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        if child.kind() == "property" {
            if let Some(n) = properties_property_node(&child, source) {
                children.push(n);
            }
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
    // properties: property { key, value } — positional named children
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

// ── Bucket A: HCL / HOCON ────────────────────────────────────────────────────

fn extract_hcl(root: &Node, source: &str) -> Option<DataNode> {
    let children = hcl_body_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn hcl_body_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "attribute" => {
                if let Some(n) = hcl_attribute_node(&child, source) {
                    result.push(n);
                }
            }
            "block" => {
                if let Some(n) = hcl_block_node(&child, source) {
                    result.push(n);
                }
            }
            "body" => {
                result.extend(hcl_body_children(&child, source));
            }
            // HOCON pairs
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
    // HCL attribute: positional named children — identifier then expression
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

fn hcl_block_node(node: &Node, source: &str) -> Option<DataNode> {
    // HCL block: identifier(s), then body
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();
    if named.is_empty() {
        return None;
    }
    // First identifier(s) form the key; last child is body.
    let key_parts: Vec<&str> = named
        .iter()
        .filter(|n| n.kind() == "identifier" || n.kind() == "string_lit")
        .map(|n| node_text(n, source))
        .collect();
    let key = if key_parts.is_empty() {
        node_text(&named[0], source).to_string()
    } else {
        key_parts.join(".")
    };
    let children = named
        .iter()
        .filter(|n| n.kind() == "body")
        .flat_map(|body| hcl_body_children(body, source))
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
    // HOCON pair: positional — path then value
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

// ── Bucket A: KDL ────────────────────────────────────────────────────────────

fn extract_kdl(root: &Node, source: &str) -> Option<DataNode> {
    let children = kdl_node_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn kdl_node_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        if child.kind() == "node" {
            if let Some(n) = kdl_single_node(&child, source) {
                result.push(n);
            }
        }
    }
    result
}

fn kdl_single_node(node: &Node, source: &str) -> Option<DataNode> {
    // KDL node: identifier field, then optional children field (block of nodes).
    let key = node
        .child_by_field_name("name")
        .map(|n| node_text(&n, source).to_string())
        .or_else(|| {
            let mut cursor = node.walk();
            node.named_children(&mut cursor)
                .find(|c| c.kind() == "identifier")
                .map(|c| node_text(&c, source).to_string())
        });

    // Value: first non-identifier named child that isn't the children block.
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
        .map(|block| kdl_node_children(&block, source))
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

// ── Bucket A: CUE ────────────────────────────────────────────────────────────

fn extract_cue(root: &Node, source: &str) -> Option<DataNode> {
    let children = cue_body_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn cue_body_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "field" => {
                if let Some(n) = cue_field_node(&child, source) {
                    result.push(n);
                }
            }
            "struct_lit" | "source_file" => {
                result.extend(cue_body_children(&child, source));
            }
            _ => {}
        }
    }
    result
}

fn cue_field_node(node: &Node, source: &str) -> Option<DataNode> {
    // CUE field: label (possibly attribute), then value
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

// ── Bucket B: YAML ───────────────────────────────────────────────────────────

fn extract_yaml(root: &Node, source: &str) -> Option<DataNode> {
    let children = yaml_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn yaml_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "block_mapping_pair" | "flow_pair" => {
                if let Some(n) = yaml_mapping_pair(&child, source) {
                    result.push(n);
                }
            }
            "block_mapping" | "flow_mapping" => {
                result.extend(yaml_children(&child, source));
            }
            "block_sequence" => {
                let items = yaml_sequence_items(&child, source);
                result.extend(items);
            }
            "document" | "block_node" | "flow_node" => {
                result.extend(yaml_children(&child, source));
            }
            "stream" => {
                result.extend(yaml_children(&child, source));
            }
            _ => {}
        }
    }
    result
}

fn yaml_mapping_pair(node: &Node, source: &str) -> Option<DataNode> {
    let key_node = node.child_by_field_name("key");
    let val_node = node.child_by_field_name("value");

    let key = key_node.map(|n| {
        let raw = node_text(&n, source);
        strip_quotes(raw).to_string()
    });

    // Check if value is a mapping (nested) or sequence.
    if let Some(val) = val_node {
        let val_kind = val.kind();
        if val_kind == "block_node" || val_kind == "flow_node" {
            let sub = yaml_children(&val, source);
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
        // Leaf value.
        let value = Some(strip_quotes(node_text(&val, source)).to_string());
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

fn yaml_sequence_items(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for (idx, child) in node.named_children(&mut cursor).enumerate() {
        if child.kind() == "block_sequence_item" {
            let sub = yaml_children(&child, source);
            let value = if sub.is_empty() {
                let mut c2 = child.walk();
                child
                    .named_children(&mut c2)
                    .next()
                    .map(|n| strip_quotes(node_text(&n, source)).to_string())
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
    }
    result
}

// ── Bucket B: INI / editorconfig ─────────────────────────────────────────────

fn extract_ini(root: &Node, source: &str) -> Option<DataNode> {
    let children = ini_top_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn ini_top_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
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
                result.extend(ini_top_children(&child, source));
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
    // INI setting: positional named children — setting_name, setting_value
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

// ── Bucket B: CSV / PSV ──────────────────────────────────────────────────────

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

// ── Bucket B: PO (gettext) ───────────────────────────────────────────────────

fn extract_po(root: &Node, source: &str) -> Option<DataNode> {
    let mut messages = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        if child.kind() == "message" {
            if let Some(n) = po_message_node(&child, source) {
                messages.push(n);
            }
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

// ── Bucket B: nginx ───────────────────────────────────────────────────────────

fn extract_nginx(root: &Node, source: &str) -> Option<DataNode> {
    let children = nginx_body_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn nginx_body_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "directive" => {
                if let Some(n) = nginx_directive_node(&child, source) {
                    result.push(n);
                }
            }
            "block" | "http" | "events" | "server" | "location" | "map" | "if" => {
                result.extend(nginx_body_children(&child, source));
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
    // Remaining named children are arguments.
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

// ── Bucket B: caddy ───────────────────────────────────────────────────────────

fn extract_caddy(root: &Node, source: &str) -> Option<DataNode> {
    let children = caddy_body_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::KeyValue,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn caddy_body_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        let kind = child.kind();
        // Caddy uses per-directive node types like directive_respond, directive_root, etc.
        if kind.starts_with("directive_") || kind == "directive" {
            if let Some(n) = caddy_directive_node(&child, source) {
                result.push(n);
            }
        } else if kind == "server" || kind == "route" || kind == "block" {
            result.extend(caddy_body_children(&child, source));
        }
    }
    result
}

fn caddy_directive_node(node: &Node, source: &str) -> Option<DataNode> {
    // The `directive_type` field holds the keyword; remaining named children are arguments.
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

// ── Bucket C: XML ─────────────────────────────────────────────────────────────

fn extract_xml(root: &Node, source: &str) -> Option<DataNode> {
    let children = xml_node_children(root, source);
    Some(DataNode {
        kind: DataNodeKind::Element,
        key: None,
        value: None,
        attributes: vec![],
        children,
        span: span_from_node(root),
    })
}

fn xml_node_children(node: &Node, source: &str) -> Vec<DataNode> {
    let mut result = Vec::new();
    let mut cursor = node.walk();
    for child in node.named_children(&mut cursor) {
        match child.kind() {
            "element" => {
                if let Some(n) = xml_element_node(&child, source) {
                    result.push(n);
                }
            }
            "document" | "content" => {
                result.extend(xml_node_children(&child, source));
            }
            _ => {}
        }
    }
    result
}

fn xml_element_node(node: &Node, source: &str) -> Option<DataNode> {
    // XML element: STag (with Name and Attributes), content, ETag.
    // Or EmptyElemTag (self-closing).
    let mut cursor = node.walk();
    let named: Vec<Node> = node.named_children(&mut cursor).collect();

    // Tag name and attributes come from STag or EmptyElemTag.
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

    // Text content from the content node (direct CharData children).
    let text_value = named
        .iter()
        .find(|c| c.kind() == "content")
        .and_then(|content| {
            let mut c2 = content.walk();
            content
                .named_children(&mut c2)
                .find(|gc| gc.kind() == "CharData" || gc.kind() == "CData")
                .map(|n| node_text(&n, source).trim().to_string())
        })
        .filter(|s| !s.is_empty());

    // Recurse into child elements.
    let children: Vec<DataNode> = named
        .iter()
        .filter(|c| c.kind() == "content")
        .flat_map(|content| xml_node_children(content, source))
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

// ── Bucket C: DTD ─────────────────────────────────────────────────────────────

fn extract_dtd(root: &Node, source: &str) -> Option<DataNode> {
    // DTD is declaration-based. Emit each top-level declaration as a KeyValue node.
    let mut children = Vec::new();
    let mut cursor = root.walk();
    for child in root.named_children(&mut cursor) {
        let kind = child.kind();
        // Cover element and attribute list declarations.
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

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str, lang: &str) -> Option<(tree_sitter::Language, tree_sitter::Tree)> {
        let registry = crate::LanguageRegistry::new();
        let language = registry.get_language(lang).ok()?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&language).ok()?;
        let tree = parser.parse(source, None)?;
        Some((language, tree))
    }

    fn extract(source: &str, lang: &str) -> Option<DataNode> {
        let (_, tree) = parse(source, lang)?;
        extract_data(&tree.root_node(), source, lang)
    }

    // JSON

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

    // TOML

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

    // Properties

    #[test]
    fn test_properties_flat() {
        let source = "host=localhost\nport=8080\n";
        let Some(root) = extract(source, "properties") else { return };
        assert!(root.children.iter().any(|c| c.key.as_deref() == Some("host")));
        assert!(root.children.iter().any(|c| c.key.as_deref() == Some("port")));
    }

    // YAML

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

    // CSV

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

    // INI

    #[test]
    fn test_ini_flat() {
        let source = "host=localhost\nport=8080\n";
        let Some(root) = extract(source, "ini") else { return };
        assert!(!root.children.is_empty(), "should have settings");
    }

    // Unsupported language returns None

    #[test]
    fn test_unsupported_language_returns_none() {
        // "python" is not in the data-extraction set
        let result = extract_data(
            &{
                let registry = crate::LanguageRegistry::new();
                let lang = match registry.get_language("python") {
                    Ok(l) => l,
                    Err(_) => return,
                };
                let mut parser = tree_sitter::Parser::new();
                if parser.set_language(&lang).is_err() {
                    return;
                }
                match parser.parse("x = 1", None) {
                    Some(t) => t,
                    None => return,
                }
            }
            .root_node(),
            "x = 1",
            "python",
        );
        assert!(result.is_none(), "python should return None for data extraction");
    }
}
