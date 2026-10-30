use super::{DataNode, DataNodeKind, Node, depth_exceeded, node_text, span_from_node, strip_quotes};

pub(super) fn extract_toml(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
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

pub(super) fn extract_properties(root: &Node, source: &str) -> Option<DataNode> {
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

pub(super) fn extract_hcl(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
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

pub(super) fn extract_kdl(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
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

pub(super) fn extract_cue(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
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
