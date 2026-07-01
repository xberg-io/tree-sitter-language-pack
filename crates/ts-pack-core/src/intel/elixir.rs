//! Elixir-specific structure and import extraction.
//!
//! Elixir has no dedicated definition nodes: `defmodule`, `def`/`defp`,
//! `defmacro`, `defstruct`, `defprotocol`, `defimpl`, `defdelegate`, and the
//! guard family are all `(call target: (identifier) ...)` nodes, so the
//! extractor dispatches on the target keyword rather than the node kind. This
//! module owns that dispatch plus the name-resolution and body-resolution
//! helpers it needs; [`super::intelligence`] keeps the generic, language-neutral
//! tree walk and calls in here for the Elixir arms.
//!
//! Entry points: [`collect_structure_call`] (the structure walk's Elixir arm)
//! and [`collect_import_call`] (the import walk's Elixir arm). Both return
//! `true` when they have fully handled the node, signalling the caller to stop
//! its own descent.

use super::intelligence::{collect_imports, collect_structure, node_text, span_from_node};
use super::types::*;

/// Classify an Elixir definition `call` by its macro keyword. Elixir
/// definitions (`defmodule`, `def`/`defp`, `defdelegate`, `defguard`/
/// `defguardp`, `defmacro`/`defmacrop`, `defstruct`, `defprotocol`, `defimpl`)
/// are `(call target: (identifier) ...)` nodes, so dispatch on the target
/// keyword. Returns the kind, resolved name, visibility, and whether the kind
/// carries a body to recurse into, or `None` for a non-definition call.
///
/// `has_body` gates the body recursion at the call site. Every kind here
/// carries a body except `defstruct`: a struct's arguments are a keyword list
/// of field defaults, not a `do`-body, so a field named `do`
/// (`defstruct [do: ...]`) must not be mistaken for one. Keeping the
/// body-bearing decision on the same keyword dispatch that resolves the kind
/// keeps a single source of truth: `StructureKind` alone is lossy (several
/// keywords collapse to `Function`).
fn elixir_definition(
    node: &tree_sitter::Node,
    source: &str,
) -> Option<(StructureKind, Option<String>, Option<String>, bool)> {
    if node.kind() != "call" {
        return None;
    }
    let target = node.child_by_field_name("target")?;
    if target.kind() != "identifier" {
        return None;
    }

    let (kind, visibility, has_body) = match node_text(&target, source) {
        "defmodule" => (StructureKind::Module, None, true),
        "defprotocol" => (StructureKind::Interface, None, true),
        "defimpl" => (StructureKind::Impl, None, true),
        "defstruct" => (StructureKind::Struct, None, false),
        "def" | "defdelegate" | "defguard" => (StructureKind::Function, Some("public".to_string()), true),
        "defp" | "defguardp" => (StructureKind::Function, Some("private".to_string()), true),
        "defmacro" => (
            StructureKind::Other("Macro".to_string()),
            Some("public".to_string()),
            true,
        ),
        "defmacrop" => (
            StructureKind::Other("Macro".to_string()),
            Some("private".to_string()),
            true,
        ),
        _ => return None,
    };

    let name = match kind {
        StructureKind::Module | StructureKind::Interface | StructureKind::Impl => elixir_module_name(node, source),
        StructureKind::Struct => None,
        _ => elixir_callable_name(node, source),
    };

    Some((kind, name, visibility, has_body))
}

/// First child of `node` with the given kind. Elixir's `arguments` and
/// `do_block` are children (not fields) of a `call`, so they are reached by
/// kind rather than `child_by_field_name`.
fn elixir_child_by_kind<'a>(node: &tree_sitter::Node<'a>, kind: &str) -> Option<tree_sitter::Node<'a>> {
    let mut cursor = node.walk();
    node.children(&mut cursor).find(|c| c.kind() == kind)
}

/// The body of an Elixir definition: the `do_block`, or, for a compact keyword
/// form (`defmodule M, do: ...`), the value of the `do:` pair in the call's
/// arguments. The keywords live directly under `arguments`, or one level deeper
/// inside a `list` for the explicit bracketed form (`defmodule M, [do: ...]`).
/// Without this the children of a compact-form definition are dropped.
fn elixir_definition_body<'a>(call: &tree_sitter::Node<'a>, source: &str) -> Option<tree_sitter::Node<'a>> {
    if let Some(do_block) = elixir_child_by_kind(call, "do_block") {
        return Some(do_block);
    }
    let args = elixir_child_by_kind(call, "arguments")?;
    // `arguments -> keywords` (bare `, do:`) or `arguments -> list -> keywords`
    // (bracketed `, [do: ...]`).
    let keywords = elixir_child_by_kind(&args, "keywords")
        .or_else(|| elixir_child_by_kind(&args, "list").and_then(|l| elixir_child_by_kind(&l, "keywords")))?;
    let mut cursor = keywords.walk();
    for pair in keywords.named_children(&mut cursor) {
        if pair.kind() == "pair"
            && let Some(key) = pair.child_by_field_name("key")
            // The keyword key renders as `do:` with trailing whitespace, e.g.
            // "do: ", so trim whitespace and the trailing colon before matching.
            && node_text(&key, source).trim().trim_end_matches(':') == "do"
        {
            return pair.child_by_field_name("value");
        }
    }
    None
}

/// Whether an Elixir `call` is a `quote` block. The quoted body is generated
/// AST, not real structure or imports, so it must not be walked.
fn is_quote(node: &tree_sitter::Node, source: &str) -> bool {
    node.kind() == "call"
        && node
            .child_by_field_name("target")
            .is_some_and(|t| t.kind() == "identifier" && node_text(&t, source) == "quote")
}

/// The module name from the call's `arguments`, for
/// `defmodule`/`defprotocol`/`defimpl`. The head is an `alias` (`Foo.Bar`), an
/// `identifier` (`__MODULE__`), or a `dot` (`__MODULE__.Inner`).
fn elixir_module_name(call: &tree_sitter::Node, source: &str) -> Option<String> {
    let args = elixir_child_by_kind(call, "arguments")?;
    let mut cursor = args.walk();
    for child in args.named_children(&mut cursor) {
        if matches!(child.kind(), "alias" | "identifier" | "dot") {
            let text = node_text(&child, source);
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
    }
    None
}

/// The callable name for the function-family definitions (`def`, `defp`,
/// `defdelegate`, `defguard`, `defguardp`, `defmacro`, `defmacrop`). The name
/// is the `target` of the inner call in `arguments`; under a `when` guard the head is
/// the `left` of a `binary_operator`, which is either that inner call (with
/// args) or a bare `identifier` (zero-arg-with-guard); a zero-arity clause with
/// no parentheses is a bare `identifier` argument. Operator definitions
/// (`def a + b`) have no plain name and yield `None`.
fn elixir_callable_name(call: &tree_sitter::Node, source: &str) -> Option<String> {
    let args = elixir_child_by_kind(call, "arguments")?;
    let mut cursor = args.walk();
    for child in args.named_children(&mut cursor) {
        // `def f(args) when guard` / `def f when guard`: the head is the `left`
        // of a `when` binary_operator. Only unwrap on `when` - an operator
        // definition (`def a + b`) is also a binary_operator but its `left` is
        // an operand, not the name, so those stay unnamed (operator defs have
        // no plain name).
        let is_when = child.kind() == "binary_operator"
            && child
                .child_by_field_name("operator")
                .is_some_and(|op| node_text(&op, source) == "when");
        let head = if is_when {
            child.child_by_field_name("left")
        } else if child.kind() == "binary_operator" {
            None
        } else {
            Some(child)
        };
        let Some(head) = head else { continue };

        // `f(args)` -> a call whose target identifier is the name.
        if head.kind() == "call"
            && let Some(t) = head.child_by_field_name("target")
            && t.kind() == "identifier"
        {
            let text = node_text(&t, source);
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
        // `f` with no parens (optionally under a guard) -> a bare identifier.
        if head.kind() == "identifier" {
            let text = node_text(&head, source);
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
    }
    None
}

/// Handle the Elixir arm of the structure walk for `node`.
///
/// Returns `true` when `node` is Elixir-significant and has been fully handled
/// (a `quote` block to skip, or a definition `call` that was emitted), in which
/// case the caller must not descend further. Returns `false` for a non-Elixir
/// node so the generic walk takes over. Body recursion routes back through
/// [`super::intelligence::collect_structure`] so nested items use the same
/// language-neutral walk.
pub(super) fn collect_structure_call(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    items: &mut Vec<StructureItem>,
) -> bool {
    // A `quote` block's body is generated AST, not real structure - do not
    // descend into it.
    if is_quote(node, source) {
        return true;
    }

    // Elixir definitions are `call` nodes; dispatch on the macro keyword and
    // recurse into the body (a `do_block` or a compact `do:` keyword value)
    // for nested items. Leaf kinds (`defstruct`) carry no body - their
    // arguments are field defaults, so skip the body descent to avoid
    // misreading a `do:` field default as a body.
    if let Some((sk, name, visibility, has_body)) = elixir_definition(node, source) {
        let body = if has_body {
            elixir_definition_body(node, source)
        } else {
            None
        };
        let body_span = body.as_ref().map(span_from_node);
        let mut children = Vec::new();
        if let Some(body) = body {
            collect_structure(&body, source, language, &mut children);
        }
        items.push(StructureItem {
            kind: sk,
            name,
            visibility,
            span: span_from_node(node),
            children,
            decorators: Vec::new(),
            doc_comment: None,
            signature: None,
            body_span,
        });
        return true;
    }

    false
}

/// Handle the Elixir arm of the import walk for a `call` node.
///
/// Elixir `import`/`alias`/`require`/`use` are `call` nodes; dispatch on the
/// target keyword. A `quote` block's body is generated AST, so its directives
/// are not real imports - do not descend into it. Descent into the call's
/// children routes back through [`super::intelligence::collect_imports`] so the
/// generic walk continues underneath. Returns `true` once the `call` node has
/// been fully handled.
pub(super) fn collect_import_call(
    node: &tree_sitter::Node,
    source: &str,
    language: &str,
    imports: &mut Vec<ImportInfo>,
) -> bool {
    if is_quote(node, source) {
        return true;
    }
    if let Some(target) = node.child_by_field_name("target")
        && target.kind() == "identifier"
        && matches!(node_text(&target, source), "import" | "alias" | "require" | "use")
    {
        let text = node_text(node, source);
        imports.push(ImportInfo {
            source: text.to_string(),
            items: Vec::new(),
            alias: None,
            is_wildcard: false,
            span: span_from_node(node),
        });
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_imports(&child, source, language, imports);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::super::intelligence::extract_intelligence;
    use super::super::types::*;

    /// Helper: parse source using the global registry (avoids Language lifetime issues).
    fn parse_with_language(source: &str, lang_name: &str) -> Option<(tree_sitter::Language, tree_sitter::Tree)> {
        let registry = crate::LanguageRegistry::new();
        let lang = registry.get_language(lang_name).ok()?;
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&lang).ok()?;
        let tree = parser.parse(source, None)?;
        Some((lang, tree))
    }

    fn parse_or_skip(source: &str, lang_name: &str) -> Option<tree_sitter::Tree> {
        parse_with_language(source, lang_name).map(|(_, tree)| tree)
    }

    fn collect_names(items: &[StructureItem]) -> Vec<&str> {
        let mut out = Vec::new();
        for s in items {
            if let Some(n) = s.name.as_deref() {
                out.push(n);
            }
            out.extend(collect_names(&s.children));
        }
        out
    }

    #[test]
    fn test_extract_elixir_module_def_and_defp() {
        let source = "defmodule Calc do\n  def add(a, b), do: a + b\n\n  defp helper(x) do\n    x * 2\n  end\nend\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);

        let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
        assert!(module.is_some(), "should find an Elixir module");
        let module = module.unwrap();
        assert_eq!(module.name.as_deref(), Some("Calc"));
        assert_eq!(module.span.start_line, 0);
        assert_eq!(module.span.end_line, 6);

        let funcs: Vec<&StructureItem> = module
            .children
            .iter()
            .filter(|s| s.kind == StructureKind::Function)
            .collect();
        let names: Vec<&str> = funcs.iter().filter_map(|s| s.name.as_deref()).collect();
        assert!(names.contains(&"add"), "should find def add; got {names:?}");
        assert!(names.contains(&"helper"), "should find defp helper; got {names:?}");

        let add = funcs.iter().find(|s| s.name.as_deref() == Some("add")).unwrap();
        let helper = funcs.iter().find(|s| s.name.as_deref() == Some("helper")).unwrap();
        assert_eq!(add.visibility.as_deref(), Some("public"));
        assert_eq!(helper.visibility.as_deref(), Some("private"));

        assert_eq!(helper.span.start_line, 3);
        assert_eq!(helper.span.end_line, 5);
    }

    #[test]
    fn test_extract_elixir_multiple_clauses_and_guards() {
        let source = "defmodule M do\n  def f(0), do: :zero\n  def f(n) when n > 0, do: :pos\nend\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);
        let module = intel
            .structure
            .iter()
            .find(|s| s.kind == StructureKind::Module)
            .unwrap();
        let f_clauses: Vec<&StructureItem> = module
            .children
            .iter()
            .filter(|s| s.name.as_deref() == Some("f"))
            .collect();
        assert_eq!(
            f_clauses.len(),
            2,
            "both clauses of f/1 should be emitted (one per clause)"
        );
        assert!(f_clauses.iter().all(|s| s.kind == StructureKind::Function));
    }

    #[test]
    fn test_extract_elixir_nested_module() {
        let source = "defmodule Outer do\n  defmodule Inner do\n    def f, do: 1\n  end\nend\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);
        let outer = intel
            .structure
            .iter()
            .find(|s| s.name.as_deref() == Some("Outer"))
            .unwrap();
        let inner = outer.children.iter().find(|s| s.kind == StructureKind::Module);
        assert!(inner.is_some(), "should find the nested Inner module");
        let inner = inner.unwrap();
        assert_eq!(inner.name.as_deref(), Some("Inner"));
        let f = inner.children.iter().find(|s| s.kind == StructureKind::Function);
        assert_eq!(f.and_then(|s| s.name.as_deref()), Some("f"));
    }

    #[test]
    fn test_extract_elixir_macros_struct_protocol_impl() {
        let source = "defmodule M do\n  defstruct [:a, :b]\n  defmacro mac(x), do: x\n  defmacrop pmac(x), do: x\nend\n\ndefprotocol P do\n  def go(x)\nend\n\ndefimpl P, for: Integer do\n  def go(x), do: x\nend\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);

        let module = intel.structure.iter().find(|s| s.name.as_deref() == Some("M")).unwrap();
        assert!(
            module.children.iter().any(|s| s.kind == StructureKind::Struct),
            "should find a defstruct as Struct"
        );
        let macros: Vec<&StructureItem> = module
            .children
            .iter()
            .filter(|s| s.kind == StructureKind::Other("Macro".to_string()))
            .collect();
        assert_eq!(macros.len(), 2, "both defmacro and defmacrop should be Macro items");
        assert!(
            macros
                .iter()
                .any(|s| s.name.as_deref() == Some("mac") && s.visibility.as_deref() == Some("public"))
        );
        assert!(
            macros
                .iter()
                .any(|s| s.name.as_deref() == Some("pmac") && s.visibility.as_deref() == Some("private"))
        );

        let protocol = intel.structure.iter().find(|s| s.kind == StructureKind::Interface);
        assert!(protocol.is_some(), "defprotocol should be an Interface");
        assert_eq!(protocol.unwrap().name.as_deref(), Some("P"));
        assert!(
            intel.structure.iter().any(|s| s.kind == StructureKind::Impl),
            "defimpl should be an Impl"
        );
    }

    #[test]
    fn test_extract_elixir_defstruct_no_body_recursion() {
        // `defstruct` arguments are a keyword list of field defaults, not a
        // `do`-body. A field named `do` (`defstruct [do: ...]`) must NOT be
        // descended into as a body - doing so would emit a phantom nested def.
        for source in [
            "defstruct [:a, :b]\n",
            "defstruct [name: nil]\n",
            "defstruct [do: (def fake, do: 1)]\n",
        ] {
            let Some(tree) = parse_or_skip(source, "elixir") else {
                return;
            };
            let intel = extract_intelligence(source, "elixir", &tree);
            let s = intel
                .structure
                .iter()
                .find(|s| s.kind == StructureKind::Struct)
                .unwrap_or_else(|| panic!("defstruct should be a Struct for {source:?}"));
            assert!(
                s.children.is_empty(),
                "defstruct is a leaf with no body recursion; got children {:?} for {source:?}",
                s.children.iter().map(|c| c.name.as_deref()).collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn test_extract_elixir_delegate_and_guards() {
        let source = "defmodule M do\n  defdelegate g(x), to: Other\n  defguard is_even(n) when rem(n, 2) == 0\n  defguardp is_odd(n) when rem(n, 2) == 1\nend\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);
        let module = intel.structure.iter().find(|s| s.name.as_deref() == Some("M")).unwrap();

        let by_name = |name: &str| module.children.iter().find(|s| s.name.as_deref() == Some(name));
        let g = by_name("g").expect("defdelegate should be a Function");
        assert_eq!(g.kind, StructureKind::Function);
        assert_eq!(g.visibility.as_deref(), Some("public"));
        let is_even = by_name("is_even").expect("defguard should be a Function");
        assert_eq!(is_even.visibility.as_deref(), Some("public"));
        let is_odd = by_name("is_odd").expect("defguardp should be a Function");
        assert_eq!(is_odd.visibility.as_deref(), Some("private"));
    }

    #[test]
    fn test_extract_elixir_compact_do_form() {
        let source = "defmodule M, do: (def f, do: 1)\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);
        let module = intel.structure.iter().find(|s| s.name.as_deref() == Some("M")).unwrap();
        let f = module.children.iter().find(|s| s.kind == StructureKind::Function);
        assert_eq!(
            f.and_then(|s| s.name.as_deref()),
            Some("f"),
            "compact `do:` form must still yield the nested def"
        );
    }

    #[test]
    fn test_extract_elixir_module_name_forms() {
        for (source, expected) in [
            ("defmodule __MODULE__ do\n  def f, do: 1\nend\n", "__MODULE__"),
            (
                "defmodule __MODULE__.Inner do\n  def f, do: 1\nend\n",
                "__MODULE__.Inner",
            ),
            ("defmodule A.B.C do\n  def f, do: 1\nend\n", "A.B.C"),
        ] {
            let Some(tree) = parse_or_skip(source, "elixir") else {
                return;
            };
            let intel = extract_intelligence(source, "elixir", &tree);
            let module = intel
                .structure
                .iter()
                .find(|s| s.kind == StructureKind::Module)
                .unwrap();
            assert_eq!(module.name.as_deref(), Some(expected), "module name for {source:?}");
        }
    }

    #[test]
    fn test_extract_elixir_guard_and_operator_heads() {
        let source = "defmodule M do\n  def foo when is_nil(x), do: 1\n  def a + b, do: a - b\nend\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);
        let module = intel.structure.iter().find(|s| s.name.as_deref() == Some("M")).unwrap();
        let funcs: Vec<&StructureItem> = module
            .children
            .iter()
            .filter(|s| s.kind == StructureKind::Function)
            .collect();
        // The guarded zero-arg clause resolves its name from the `when` head.
        assert!(
            funcs.iter().any(|s| s.name.as_deref() == Some("foo")),
            "guarded def foo should resolve its name"
        );
        // An operator definition has no plain name and is intentionally unnamed
        // (the item and span are still produced).
        assert!(
            funcs.iter().any(|s| s.name.is_none()),
            "operator def should be an unnamed Function item"
        );
    }

    #[test]
    fn test_extract_elixir_quote_block_not_walked() {
        let source = "defmodule M do\n  defmacro gen do\n    quote do\n      import Foo.Bar\n      def generated, do: :ok\n    end\n  end\nend\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);
        // The quoted body is generated AST, not real structure/imports.
        assert!(intel.imports.is_empty(), "import inside quote must not be reported");
        let names: Vec<&str> = collect_names(&intel.structure);
        assert!(names.contains(&"gen"), "the defmacro itself is real");
        assert!(!names.contains(&"generated"), "def inside quote must not be reported");
    }

    #[test]
    fn test_extract_elixir_compact_do_bracketed_form() {
        // The explicit bracketed keyword form puts the keywords inside a `list`
        // (`arguments -> list -> keywords`), one level deeper than the bare
        // `, do:` form. The nested def must still be captured.
        for source in [
            "defmodule M, [do: (def f, do: 1)]\n",
            "defmodule(M, [do: (def f, do: 1)])\n",
        ] {
            let Some(tree) = parse_or_skip(source, "elixir") else {
                return;
            };
            let intel = extract_intelligence(source, "elixir", &tree);
            let module = intel.structure.iter().find(|s| s.name.as_deref() == Some("M")).unwrap();
            let f = module.children.iter().find(|s| s.kind == StructureKind::Function);
            assert_eq!(
                f.and_then(|s| s.name.as_deref()),
                Some("f"),
                "bracketed compact form must yield the nested def for {source:?}"
            );
        }
    }

    #[test]
    fn test_extract_elixir_imports() {
        let source = "defmodule M do\n  import Enum\n  alias Foo.Bar\n  require Logger\n  use GenServer\nend\n";
        let Some(tree) = parse_or_skip(source, "elixir") else {
            return;
        };
        let intel = extract_intelligence(source, "elixir", &tree);
        let sources: Vec<&str> = intel.imports.iter().map(|i| i.source.as_str()).collect();
        assert!(
            sources.iter().any(|s| s.starts_with("import Enum")),
            "import; got {sources:?}"
        );
        assert!(
            sources.iter().any(|s| s.starts_with("alias Foo.Bar")),
            "alias; got {sources:?}"
        );
        assert!(
            sources.iter().any(|s| s.starts_with("require Logger")),
            "require; got {sources:?}"
        );
        assert!(
            sources.iter().any(|s| s.starts_with("use GenServer")),
            "use; got {sources:?}"
        );
    }
}
