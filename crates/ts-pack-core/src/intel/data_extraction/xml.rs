use super::super::types::DataAttribute;
use super::{
    DataNode, DataNodeKind, Node, depth_exceeded, named_child_of_kind, node_text, span_from_node, strip_quotes,
};

pub(super) fn extract_xml(root: &Node, source: &str, truncated: &mut usize) -> Option<DataNode> {
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

pub(super) fn extract_dtd(root: &Node, source: &str) -> Option<DataNode> {
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
