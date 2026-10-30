use super::*;

// ~keep A missing grammar reports `SKIPPED` on stderr; see `intel::test_support`.
fn parse_or_skip(source: &str, lang_name: &str) -> Option<tree_sitter::Tree> {
    crate::intel::test_support::parse_or_skip(source, lang_name)
}

#[test]
fn test_extract_python_function() {
    let source = "def foo():\n    pass\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.language, "python");
    assert!(!intel.structure.is_empty(), "should find at least one structure item");
    let func = &intel.structure[0];
    assert_eq!(func.kind, StructureKind::Function);
    assert_eq!(func.name.as_deref(), Some("foo"));
}

#[test]
fn test_extract_python_class() {
    let source = "class MyClass:\n    def method(self):\n        pass\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
    assert!(class.is_some(), "should find a class");
    let class = class.unwrap();
    assert_eq!(class.name.as_deref(), Some("MyClass"));
    assert!(!class.children.is_empty(), "class should have child methods");
    assert_eq!(class.children[0].kind, StructureKind::Function);
    assert_eq!(class.children[0].name.as_deref(), Some("method"));
}

#[test]
fn test_extract_ruby_module_class_and_methods() {
    let source = "module Outer\n  class Widget\n    def call\n      true\n    end\n\n    def self.build\n      new\n    end\n  end\nend\n";
    let Some(tree) = parse_or_skip(source, "ruby") else {
        return;
    };
    let intel = extract_intelligence(source, "ruby", &tree);

    let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
    assert!(module.is_some(), "should find a Ruby module entry");
    let module = module.unwrap();
    assert_eq!(module.name.as_deref(), Some("Outer"));

    let class = module.children.iter().find(|s| s.kind == StructureKind::Class);
    assert!(class.is_some(), "should find a Ruby class inside the module");
    let class = class.unwrap();
    assert_eq!(class.name.as_deref(), Some("Widget"));

    let method_names = class
        .children
        .iter()
        .filter(|s| s.kind == StructureKind::Method)
        .filter_map(|s| s.name.as_deref())
        .collect::<Vec<_>>();
    assert!(method_names.contains(&"call"), "should find an instance method");
    assert!(method_names.contains(&"build"), "should find a singleton method");
}

#[test]
fn test_extract_rust_function() {
    let source = "fn main() {\n    let x = 5;\n}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert!(!intel.structure.is_empty(), "should find at least one structure item");
    let func = &intel.structure[0];
    assert_eq!(func.kind, StructureKind::Function);
    assert_eq!(func.name.as_deref(), Some("main"));
}

#[test]
fn test_extract_python_imports() {
    let source = "import os\nfrom sys import path\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.imports.len(), 2, "should find 2 imports");
    assert!(intel.imports[0].source.contains("import os"));
    assert!(intel.imports[1].source.contains("from sys import path"));
}

#[test]
fn test_extract_rust_imports() {
    let source = "use std::collections::HashMap;\nuse std::io;\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(intel.imports.len(), 2, "should find 2 use declarations");
}

#[test]
fn test_extract_comments() {
    let source = "// This is a comment\nfn main() {}\n// Another comment\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert!(intel.comments.len() >= 2, "should find at least 2 comments");
    assert!(intel.comments[0].text.contains("This is a comment"));
}

#[test]
fn test_extract_doc_comments() {
    let source = "/// Documentation comment\nfn documented() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    let doc_comments: Vec<_> = intel.comments.iter().filter(|c| c.kind == CommentKind::Doc).collect();
    assert!(!doc_comments.is_empty(), "should find doc comments");
}

#[test]
fn test_metrics_counts() {
    let source = "fn foo() {}\n\n// comment\nfn bar() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert!(intel.metrics.total_lines >= 4, "should have at least 4 lines");
    assert!(intel.metrics.blank_lines >= 1, "should have at least 1 blank line");
    assert!(intel.metrics.comment_lines >= 1, "should have at least 1 comment line");
    assert!(intel.metrics.code_lines >= 2, "should have at least 2 code lines");
    assert!(intel.metrics.node_count > 0, "should have nodes");
    assert_eq!(intel.metrics.error_count, 0, "valid code should have 0 errors");
    assert!(intel.metrics.max_depth > 0, "tree should have depth > 0");
    assert_eq!(intel.metrics.total_bytes, source.len());
}

#[test]
fn should_count_each_go_import_once() {
    let source = "package main\n\nimport (\n\t\"fmt\"\n\t\"os\"\n)\n";
    let Some(tree) = parse_or_skip(source, "go") else {
        return;
    };
    let intel = extract_intelligence(source, "go", &tree);

    let sources: Vec<&str> = intel.imports.iter().map(|i| i.source.as_str()).collect();
    assert_eq!(
        sources,
        vec!["\"fmt\"", "\"os\""],
        "an import block must yield one entry per spec, not one per spec plus the block"
    );
}

#[test]
fn should_count_a_single_go_import_once() {
    let source = "package main\n\nimport \"fmt\"\n";
    let Some(tree) = parse_or_skip(source, "go") else {
        return;
    };
    let intel = extract_intelligence(source, "go", &tree);

    let sources: Vec<&str> = intel.imports.iter().map(|i| i.source.as_str()).collect();
    assert_eq!(sources, vec!["\"fmt\""]);
}

#[test]
fn should_populate_items_and_alias_for_a_single_name_python_import() {
    let source = "import numpy as np\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert_eq!(intel.imports[0].items, vec!["numpy".to_string()]);
    assert_eq!(intel.imports[0].alias.as_deref(), Some("np"));
}

#[test]
fn should_populate_items_without_an_alias_for_a_python_from_import() {
    let source = "from sys import path\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert_eq!(intel.imports[0].items, vec!["path".to_string()]);
    assert_eq!(intel.imports[0].alias, None);
}

#[test]
fn should_leave_alias_none_for_a_python_import_naming_several_modules() {
    let source = "import os, sys as s\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert_eq!(
        intel.imports[0].items,
        vec!["os".to_string(), "sys".to_string()],
        "items lists every entry's base name, aliased or not"
    );
    assert_eq!(
        intel.imports[0].alias, None,
        "a statement aliasing one of several names has no single alias to report"
    );
}

#[test]
fn should_populate_alias_for_a_javascript_namespace_import() {
    let source = "import * as ns from 'mod';\n";
    let Some(tree) = parse_or_skip(source, "javascript") else {
        return;
    };
    let intel = extract_intelligence(source, "javascript", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert_eq!(intel.imports[0].items, Vec::<String>::new());
    assert_eq!(intel.imports[0].alias.as_deref(), Some("ns"));
}

#[test]
fn should_populate_items_for_javascript_named_imports() {
    let source = "import { a, b as c } from 'mod';\n";
    let Some(tree) = parse_or_skip(source, "javascript") else {
        return;
    };
    let intel = extract_intelligence(source, "javascript", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert_eq!(intel.imports[0].items, vec!["a".to_string(), "b".to_string()]);
    assert_eq!(
        intel.imports[0].alias, None,
        "more than one specifier has no single alias to report"
    );
}

#[test]
fn should_leave_items_and_alias_empty_for_a_javascript_default_import() {
    let source = "import fs from 'node:fs';\n";
    let Some(tree) = parse_or_skip(source, "javascript") else {
        return;
    };
    let intel = extract_intelligence(source, "javascript", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert_eq!(intel.imports[0].items, Vec::<String>::new());
    assert_eq!(intel.imports[0].alias, None);
}

#[test]
fn should_flag_a_python_wildcard_import() {
    let source = "from pkg import *\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert!(intel.imports[0].is_wildcard);
}

#[test]
fn should_flag_a_rust_glob_use_as_wildcard() {
    let source = "use std::collections::*;\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert!(intel.imports[0].is_wildcard);
}

#[test]
fn should_not_flag_a_glob_character_inside_an_import_path_string_as_wildcard() {
    let source = "import a from './glob*.js';\n";
    let Some(tree) = parse_or_skip(source, "javascript") else {
        return;
    };
    let intel = extract_intelligence(source, "javascript", &tree);

    assert_eq!(intel.imports.len(), 1);
    assert!(
        !intel.imports[0].is_wildcard,
        "a `*` inside the source path string is not a wildcard-import token"
    );
}

#[test]
fn should_populate_a_rust_function_signature_up_to_its_body() {
    let source = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(intel.structure.len(), 1);
    assert_eq!(
        intel.structure[0].signature.as_deref(),
        Some("fn add(a: i32, b: i32) -> i32")
    );
}

#[test]
fn should_populate_a_doc_comment_for_a_documented_rust_function() {
    let source = "/// Adds two numbers.\nfn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(intel.structure.len(), 1);
    assert_eq!(intel.structure[0].doc_comment.as_deref(), Some("/// Adds two numbers."));
}

#[test]
fn should_join_a_multiline_rust_doc_comment_in_source_order() {
    let source = "/// Line one.\n/// Line two.\nfn add() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(intel.structure.len(), 1);
    assert_eq!(
        intel.structure[0].doc_comment.as_deref(),
        Some("/// Line one.\n/// Line two.")
    );
}

#[test]
fn should_leave_doc_comment_none_when_a_blank_line_separates_the_comment() {
    let source = "/// Unrelated doc comment.\n\nfn add() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(intel.structure.len(), 1);
    assert_eq!(
        intel.structure[0].doc_comment, None,
        "a blank line breaks the association between a doc comment and the item after it"
    );
}

#[test]
fn should_leave_decorators_reserved_and_empty_for_a_python_decorated_function() {
    let source = "@staticmethod\ndef f():\n    pass\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert!(!intel.structure.is_empty(), "should still find the function item");
    assert!(
        intel.structure.iter().all(|item| item.decorators.is_empty()),
        "decorators is reserved and must stay empty until implemented"
    );
}

#[test]
fn should_leave_parsed_sections_reserved_and_empty_for_a_python_docstring() {
    let source = "def f(x):\n    \"\"\"Do a thing.\n\n    Args:\n        x: a value.\n    \"\"\"\n    pass\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.docstrings.len(), 1);
    assert!(
        intel.docstrings[0].parsed_sections.is_empty(),
        "parsed_sections is reserved and must stay empty until implemented"
    );
}

#[test]
fn should_resolve_javascript_export_kinds_and_names() {
    let source = "const helper = 2;\nexport const alpha = 1;\nexport default function beta() {}\nexport function gamma() {}\nexport { helper };\nexport * from './other.js';\n";
    let Some(tree) = parse_or_skip(source, "javascript") else {
        return;
    };
    let intel = extract_intelligence(source, "javascript", &tree);

    assert_eq!(intel.exports.len(), 5, "every export statement must be reported once");
    let names: Vec<&str> = intel.exports.iter().take(4).map(|e| e.name.as_str()).collect();
    assert_eq!(
        names,
        vec!["alpha", "beta", "gamma", "helper"],
        "the name must be the exported binding, not the whole statement"
    );
    let kinds: Vec<&ExportKind> = intel.exports.iter().map(|e| &e.kind).collect();
    assert_eq!(
        kinds,
        vec![
            &ExportKind::Named,
            &ExportKind::Default,
            &ExportKind::Named,
            &ExportKind::Named,
            &ExportKind::ReExport,
        ]
    );
}

#[test]
fn should_not_count_string_content_as_comment_lines() {
    let source = "text = \"\"\"\n# not a comment\n* not a comment\n// not a comment\n\"\"\"\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.metrics.total_lines, 5);
    assert_eq!(
        intel.metrics.comment_lines, 0,
        "a line prefix inside a string literal is not a comment"
    );
    assert_eq!(intel.metrics.code_lines, 5);
}

#[test]
fn should_not_count_a_yaml_block_scalar_line_as_a_comment() {
    let source = "script: |\n  # not a comment\n  echo hi\n";
    let Some(tree) = parse_or_skip(source, "yaml") else {
        return;
    };
    let intel = extract_intelligence(source, "yaml", &tree);

    assert_eq!(intel.metrics.total_lines, 3);
    assert_eq!(
        intel.metrics.comment_lines, 0,
        "a `#` inside a block scalar is data, not a comment"
    );
    assert_eq!(intel.metrics.code_lines, 3);
}

#[test]
fn should_count_a_trailing_comment_line_as_code() {
    let source = "# leading\nx = 1  # trailing\n\n# another\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.metrics.total_lines, 4);
    assert_eq!(intel.metrics.blank_lines, 1);
    assert_eq!(intel.metrics.comment_lines, 2, "only the two whole-line comments count");
    assert_eq!(intel.metrics.code_lines, 1);
}

#[test]
fn should_count_every_line_of_a_multi_line_block_comment() {
    let source = "/* first\n   second\n   third */\nfn main() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(intel.metrics.total_lines, 4);
    assert_eq!(intel.metrics.comment_lines, 3, "a block comment covers all of its rows");
    assert_eq!(intel.metrics.code_lines, 1);
}

#[test]
fn should_report_a_rust_doc_comment_once() {
    let source = "/// Documented\nfn documented() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(
        intel.comments.len(),
        1,
        "the grammar nests `doc_comment` inside `line_comment`; only the outer node is a comment"
    );
    assert_eq!(intel.comments[0].kind, CommentKind::Doc);
    // ~keep `text` is the raw node text. Whether a line comment's token includes its
    // ~keep trailing newline is grammar-dependent (tree-sitter-rust's does), so the
    // ~keep assertion trims rather than baking one grammar's token boundary in.
    assert_eq!(intel.comments[0].text.trim_end(), "/// Documented");
}

#[test]
fn test_extract_symbols() {
    let source = "fn alpha() {}\nfn beta() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    let func_symbols: Vec<_> = intel
        .symbols
        .iter()
        .filter(|s| s.kind == SymbolKind::Function)
        .collect();
    assert!(func_symbols.len() >= 2, "should find at least 2 function symbols");
    let names: Vec<_> = func_symbols.iter().map(|s| s.name.as_str()).collect();
    assert!(names.contains(&"alpha"));
    assert!(names.contains(&"beta"));
}

#[test]
fn test_extract_go_type_declarations_as_symbols() {
    let source = "type User struct{}\ntype Service interface{}\ntype ID string\n";
    let Some(tree) = parse_or_skip(source, "go") else {
        return;
    };
    let intel = extract_intelligence(source, "go", &tree);

    assert!(
        intel
            .symbols
            .iter()
            .any(|s| { s.kind == SymbolKind::Type && s.name == "User" })
    );
    assert!(
        intel
            .symbols
            .iter()
            .any(|s| { s.kind == SymbolKind::Interface && s.name == "Service" })
    );
    assert!(
        intel
            .symbols
            .iter()
            .any(|s| { s.kind == SymbolKind::Type && s.name == "ID" })
    );
}

#[test]
fn should_populate_symbol_doc_from_an_immediately_preceding_doc_comment() {
    let source = "/// Documented function.\nfn documented() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    let symbol = intel
        .symbols
        .iter()
        .find(|s| s.name == "documented")
        .expect("the documented function should be extracted as a symbol");
    assert_eq!(
        symbol.doc.as_deref(),
        Some("/// Documented function."),
        "SymbolInfo.doc must reuse doc_comment_at rather than stay hard-coded None"
    );
}

#[test]
fn should_leave_symbol_doc_none_for_a_symbol_with_no_preceding_doc_comment() {
    let source = "fn undocumented() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    let symbol = intel
        .symbols
        .iter()
        .find(|s| s.name == "undocumented")
        .expect("the undocumented function should be extracted as a symbol");
    assert_eq!(symbol.doc, None);
}

#[test]
fn test_error_nodes_detected() {
    let source = "def :\n    pass\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert!(
        intel.metrics.error_count > 0,
        "invalid syntax should produce error nodes"
    );
    assert!(!intel.diagnostics.is_empty(), "should have diagnostics for errors");
    assert!(
        intel
            .diagnostics
            .iter()
            .any(|d| d.severity == DiagnosticSeverity::Error)
    );
}

#[test]
fn test_valid_code_no_diagnostics() {
    let source = "def foo():\n    pass\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.metrics.error_count, 0);
    assert!(intel.diagnostics.is_empty(), "valid code should have no diagnostics");
}

#[test]
fn should_extract_a_python_function_docstring_with_its_text() {
    let source = "def greet():\n    \"\"\"Say hello.\"\"\"\n    pass\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.docstrings.len(), 1, "should find exactly one python docstring");
    let docstring = &intel.docstrings[0];
    assert_eq!(docstring.text, "\"\"\"Say hello.\"\"\"");
    assert_eq!(docstring.format, DocstringFormat::PythonTripleQuote);
    assert_eq!(docstring.associated_item.as_deref(), Some("greet"));
    assert_eq!(docstring.span.start_line, 1);
}

#[test]
fn should_extract_module_and_class_docstrings_without_duplicating_them() {
    let source = "\"\"\"Module doc.\"\"\"\n\n\nclass Widget:\n    \"\"\"Widget doc.\"\"\"\n\n    def render(self):\n        \"\"\"Render doc.\"\"\"\n        return 1\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    let texts: Vec<&str> = intel.docstrings.iter().map(|d| d.text.as_str()).collect();
    assert_eq!(
        texts,
        vec![
            "\"\"\"Module doc.\"\"\"",
            "\"\"\"Widget doc.\"\"\"",
            "\"\"\"Render doc.\"\"\""
        ],
        "one docstring per module, class and function, in source order"
    );
    let owners: Vec<Option<&str>> = intel.docstrings.iter().map(|d| d.associated_item.as_deref()).collect();
    assert_eq!(owners, vec![None, Some("Widget"), Some("render")]);
}

#[test]
fn should_not_report_a_nested_string_expression_as_a_docstring() {
    let source = "def greet():\n    if True:\n        pass\n    return \"not a docstring\"\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert!(
        intel.docstrings.is_empty(),
        "a returned string is not a docstring; got {:?}",
        intel.docstrings
    );
}

#[test]
fn should_report_a_module_docstring_after_a_shebang_line() {
    let source = "#!/usr/bin/env python3\n\"\"\"Module doc.\"\"\"\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(
        intel.docstrings.len(),
        1,
        "a leading shebang must not hide the module docstring; got {:?}",
        intel.docstrings
    );
    assert_eq!(intel.docstrings[0].text, "\"\"\"Module doc.\"\"\"");
}

#[test]
fn should_report_a_module_docstring_after_leading_license_comments() {
    let source = "# Copyright 2026 Example Corp.\n# SPDX-License-Identifier: Apache-2.0\n\"\"\"Module doc.\"\"\"\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(
        intel.docstrings.len(),
        1,
        "leading license comments must not hide the module docstring; got {:?}",
        intel.docstrings
    );
    assert_eq!(intel.docstrings[0].text, "\"\"\"Module doc.\"\"\"");
}

#[test]
// ~keep This case does NOT exercise the leading-comment bug, and is kept as a shape
// ~keep guard rather than a regression test. Measured tree for this exact source:
// ~keep (function_definition ... (comment) body: (block (string ...) (pass_statement)))
// ~keep — the comment attaches to `function_definition`, NOT to the `block`, so the
// ~keep block's first named child was already the docstring and the old
// ~keep `named_child(0)` check passed here too. Verified by dumping the s-expression.
// ~keep Only the MODULE-level tests above discriminate, because there a comment is a
// ~keep direct child of `module`. This test pins that attachment shape: if a grammar
// ~keep update ever moves the comment inside the block, it starts failing and tells us.
fn should_report_a_function_docstring_after_a_leading_comment_in_its_body() {
    let source = "def greet():\n    # set up\n    \"\"\"Doc.\"\"\"\n    pass\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(
        intel.docstrings.len(),
        1,
        "a leading comment inside the body must not hide the function docstring; got {:?}",
        intel.docstrings
    );
    assert_eq!(intel.docstrings[0].text, "\"\"\"Doc.\"\"\"");
    assert_eq!(intel.docstrings[0].associated_item.as_deref(), Some("greet"));
}

#[test]
fn should_still_report_exactly_one_docstring_when_a_stray_string_follows_it() {
    let source = "def greet():\n    \"\"\"Real doc.\"\"\"\n    name = \"cafe\"\n    \"stray string\"\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(
        intel.docstrings.len(),
        1,
        "the skip-extras fix must not resurrect the fabricated stray docstring; got {:?}",
        intel.docstrings
    );
    assert_eq!(intel.docstrings[0].text, "\"\"\"Real doc.\"\"\"");
    assert_eq!(intel.docstrings[0].associated_item.as_deref(), Some("greet"));
}

#[test]
fn should_not_mark_a_blank_line_after_a_newline_swallowing_comment_as_a_comment_row() {
    let source = "// comment\n\nfn main() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let root = tree.root_node();
    let comment = root.named_child(0).expect("source must start with a comment node");
    assert_eq!(
        comment.kind(),
        "line_comment",
        "test setup must target the comment node"
    );

    let mut rows = BTreeSet::new();
    mark_comment_rows(&comment, source, &mut rows);

    assert_eq!(
        rows,
        BTreeSet::from([0]),
        "the blank row after the comment must not be reported as a comment row"
    );
}

#[test]
fn test_intelligence_language_field() {
    let source = "x = 1";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);
    assert_eq!(intel.language, "python");
}

#[test]
fn collect_structure_kotlin_package_and_class() {
    let source = "package foo.bar\n\nclass Widget {}\n";
    let Some(tree) = parse_or_skip(source, "kotlin") else {
        return;
    };
    let intel = extract_intelligence(source, "kotlin", &tree);

    let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
    assert!(module.is_some(), "should find a Module entry for the package header");
    assert_eq!(module.unwrap().name.as_deref(), Some("foo.bar"));

    let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
    assert!(class.is_some(), "should find a Class entry");
    assert_eq!(class.unwrap().name.as_deref(), Some("Widget"));
}

#[test]
fn collect_structure_java_package_and_class() {
    let source = "package com.example;\n\npublic class Widget {}\n";
    let Some(tree) = parse_or_skip(source, "java") else {
        return;
    };
    let intel = extract_intelligence(source, "java", &tree);

    let module = intel.structure.iter().find(|s| s.kind == StructureKind::Module);
    assert!(
        module.is_some(),
        "should find a Module entry for the package declaration"
    );
    assert_eq!(module.unwrap().name.as_deref(), Some("com.example"));

    let class = intel.structure.iter().find(|s| s.kind == StructureKind::Class);
    assert!(class.is_some(), "should find a Class entry");
    assert_eq!(class.unwrap().name.as_deref(), Some("Widget"));
}

#[test]
fn should_report_exactly_one_docstring_when_a_stray_string_follows_it() {
    let source = "def greet():\n    \"\"\"Real doc.\"\"\"\n    name = \"cafe\"\n    \"stray string\"\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(
        intel.docstrings.len(),
        1,
        "only the first statement is a docstring; got {:?}",
        intel.docstrings
    );
    assert_eq!(intel.docstrings[0].text, "\"\"\"Real doc.\"\"\"");
    assert_eq!(intel.docstrings[0].associated_item.as_deref(), Some("greet"));
}

#[test]
fn should_use_a_non_triple_quote_format_when_docstring_is_single_quoted() {
    let source = "def f():\n    'x'\n";
    let Some(tree) = parse_or_skip(source, "python") else {
        return;
    };
    let intel = extract_intelligence(source, "python", &tree);

    assert_eq!(intel.docstrings.len(), 1);
    assert_eq!(
        intel.docstrings[0].format,
        DocstringFormat::Other("python-single-quote".to_string()),
        "a single-quoted string statement is not the triple-quote convention"
    );
}

#[test]
fn should_count_code_lines_when_a_block_comment_closes_mid_line() {
    let source = "/* a\n*/ fn main() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(
        intel.metrics.comment_lines, 1,
        "only the opening row is wholly a comment; the closing row carries code"
    );
    assert_eq!(
        intel.metrics.code_lines, 1,
        "the line containing `fn main` must not be entirely swallowed by the comment"
    );
}

#[test]
fn should_count_closing_row_as_comment_when_blank_after_close() {
    let source = "/* a\n*/   \nfn main() {}\n";
    let Some(tree) = parse_or_skip(source, "rust") else {
        return;
    };
    let intel = extract_intelligence(source, "rust", &tree);

    assert_eq!(
        intel.metrics.comment_lines, 2,
        "the closing row has only whitespace after `*/` so it is still a comment row"
    );
    assert_eq!(intel.metrics.code_lines, 1);
}
