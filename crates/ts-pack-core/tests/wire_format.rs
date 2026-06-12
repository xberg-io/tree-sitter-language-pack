//! Wire-format snapshots for every public DTO serialized into bindings.
//!
//! These tests lock the JSON shape of the public intel types. Every binding
//! (NAPI-RS, PyO3, Magnus, ext-php-rs, Rustler, JNI, ffi, wasm, swift, dart,
//! go, kotlin-android) has hand-written deserializers — including Jackson
//! sealed-class hierarchies in JVM-land — that match this exact shape. Any
//! accidental change to `#[serde(...)]` attributes or field names breaks
//! every binding's `process()` tests simultaneously.
//!
//! If a test here fails, the JSON wire format changed. Either revert the
//! Rust change, or — if the change is deliberate — accept it explicitly
//! with `cargo insta review`, commit the `.snap` diff so the intent is
//! visible in code review, and regen every binding with `task alef:generate`.

use insta::assert_json_snapshot;
use tree_sitter_language_pack::{
    ChunkContext, CodeChunk, CommentInfo, CommentKind, Diagnostic, DiagnosticSeverity, DocSection, DocstringFormat,
    DocstringInfo, ExportInfo, ExportKind, FileMetrics, ImportInfo, ProcessConfig, ProcessResult, Span, StructureItem,
    StructureKind, SymbolInfo, SymbolKind,
};

fn span() -> Span {
    Span {
        start_byte: 10,
        end_byte: 20,
        start_line: 1,
        start_column: 0,
        end_line: 2,
        end_column: 5,
    }
}

// -- Heterogeneous enums (the three Phase F was almost broken on) --

#[test]
fn structure_kind_unit_variant_serializes_as_bare_string() {
    assert_json_snapshot!(StructureKind::Function);
    assert_json_snapshot!(StructureKind::Method);
    assert_json_snapshot!(StructureKind::Class);
    assert_json_snapshot!(StructureKind::Struct);
    assert_json_snapshot!(StructureKind::Interface);
    assert_json_snapshot!(StructureKind::Enum);
    assert_json_snapshot!(StructureKind::Module);
    assert_json_snapshot!(StructureKind::Trait);
    assert_json_snapshot!(StructureKind::Impl);
    assert_json_snapshot!(StructureKind::Namespace);
}

#[test]
fn structure_kind_other_serializes_as_single_keyed_object() {
    assert_json_snapshot!(StructureKind::Other("macro".to_string()));
}

#[test]
fn docstring_format_unit_variant_serializes_as_bare_string() {
    assert_json_snapshot!(DocstringFormat::PythonTripleQuote);
    assert_json_snapshot!(DocstringFormat::JSDoc);
    assert_json_snapshot!(DocstringFormat::Rustdoc);
    assert_json_snapshot!(DocstringFormat::GoDoc);
    assert_json_snapshot!(DocstringFormat::JavaDoc);
}

#[test]
fn docstring_format_other_serializes_as_single_keyed_object() {
    assert_json_snapshot!(DocstringFormat::Other("rst".to_string()));
}

#[test]
fn symbol_kind_unit_variant_serializes_as_bare_string() {
    assert_json_snapshot!(SymbolKind::Variable);
    assert_json_snapshot!(SymbolKind::Constant);
    assert_json_snapshot!(SymbolKind::Function);
    assert_json_snapshot!(SymbolKind::Class);
    assert_json_snapshot!(SymbolKind::Type);
    assert_json_snapshot!(SymbolKind::Interface);
    assert_json_snapshot!(SymbolKind::Enum);
    assert_json_snapshot!(SymbolKind::Module);
}

#[test]
fn symbol_kind_other_serializes_as_single_keyed_object() {
    assert_json_snapshot!(SymbolKind::Other("macro".to_string()));
}

// -- Unit-only enums --

#[test]
fn comment_kind_serializes_as_bare_string() {
    assert_json_snapshot!(CommentKind::Line);
    assert_json_snapshot!(CommentKind::Block);
    assert_json_snapshot!(CommentKind::Doc);
}

#[test]
fn export_kind_serializes_as_bare_string() {
    assert_json_snapshot!(ExportKind::Named);
    assert_json_snapshot!(ExportKind::Default);
    assert_json_snapshot!(ExportKind::ReExport);
}

#[test]
fn diagnostic_severity_serializes_as_bare_string() {
    assert_json_snapshot!(DiagnosticSeverity::Error);
    assert_json_snapshot!(DiagnosticSeverity::Warning);
    assert_json_snapshot!(DiagnosticSeverity::Info);
}

// -- Structs --

#[test]
fn span_shape() {
    assert_json_snapshot!(span());
}

#[test]
fn file_metrics_shape() {
    assert_json_snapshot!(FileMetrics {
        total_lines: 50,
        code_lines: 30,
        comment_lines: 10,
        blank_lines: 10,
        total_bytes: 1024,
        node_count: 200,
        error_count: 0,
        max_depth: 5,
    });
}

#[test]
fn structure_item_shape() {
    assert_json_snapshot!(StructureItem {
        kind: StructureKind::Function,
        name: Some("foo".to_string()),
        visibility: Some("pub".to_string()),
        span: span(),
        children: vec![],
        decorators: vec!["#[inline]".to_string()],
        doc_comment: Some("/// A function.".to_string()),
        signature: Some("fn foo()".to_string()),
        body_span: Some(span()),
    });
}

#[test]
fn structure_item_with_other_kind_shape() {
    assert_json_snapshot!(StructureItem {
        kind: StructureKind::Other("macro".to_string()),
        name: Some("my_macro".to_string()),
        visibility: None,
        span: span(),
        children: vec![],
        decorators: vec![],
        doc_comment: None,
        signature: None,
        body_span: None,
    });
}

#[test]
fn comment_info_shape() {
    assert_json_snapshot!(CommentInfo {
        text: "// a comment".to_string(),
        kind: CommentKind::Line,
        span: span(),
        associated_node: Some("function_definition".to_string()),
    });
}

#[test]
fn doc_section_shape() {
    assert_json_snapshot!(DocSection {
        kind: "args".to_string(),
        name: Some("x".to_string()),
        description: "The input value.".to_string(),
    });
}

#[test]
fn docstring_info_shape() {
    assert_json_snapshot!(DocstringInfo {
        text: "\"\"\"foo\"\"\"".to_string(),
        format: DocstringFormat::PythonTripleQuote,
        span: span(),
        associated_item: Some("my_func".to_string()),
        parsed_sections: vec![DocSection {
            kind: "returns".to_string(),
            name: None,
            description: "the result".to_string(),
        }],
    });
}

#[test]
fn import_info_shape() {
    assert_json_snapshot!(ImportInfo {
        source: "numpy".to_string(),
        items: vec!["array".to_string()],
        alias: Some("np".to_string()),
        is_wildcard: false,
        span: span(),
    });
}

#[test]
fn export_info_shape() {
    assert_json_snapshot!(ExportInfo {
        name: "foo".to_string(),
        kind: ExportKind::Named,
        span: span(),
    });
}

#[test]
fn symbol_info_shape() {
    assert_json_snapshot!(SymbolInfo {
        name: "x".to_string(),
        kind: SymbolKind::Variable,
        span: span(),
        type_annotation: Some("int".to_string()),
        doc: Some("the answer".to_string()),
    });
}

#[test]
fn diagnostic_shape() {
    assert_json_snapshot!(Diagnostic {
        message: "Syntax error: unexpected '}'".to_string(),
        severity: DiagnosticSeverity::Error,
        span: span(),
    });
}

#[test]
fn chunk_context_shape() {
    assert_json_snapshot!(ChunkContext {
        language: "python".to_string(),
        chunk_index: 0,
        total_chunks: 3,
        node_types: vec!["function_definition".to_string()],
        context_path: vec!["MyClass".to_string(), "my_method".to_string()],
        symbols_defined: vec!["x".to_string()],
        comments: vec![],
        docstrings: vec![],
        has_error_nodes: false,
    });
}

#[test]
fn code_chunk_shape() {
    assert_json_snapshot!(CodeChunk {
        content: "def foo():\n    pass".to_string(),
        start_byte: 0,
        end_byte: 19,
        start_line: 0,
        end_line: 1,
        metadata: ChunkContext {
            language: "python".to_string(),
            chunk_index: 0,
            total_chunks: 1,
            node_types: vec!["function_definition".to_string()],
            context_path: vec![],
            symbols_defined: vec!["foo".to_string()],
            comments: vec![],
            docstrings: vec![],
            has_error_nodes: false,
        },
    });
}

#[test]
fn process_config_shape() {
    assert_json_snapshot!(ProcessConfig {
        language: std::borrow::Cow::Borrowed("python"),
        structure: true,
        imports: true,
        exports: true,
        comments: false,
        docstrings: false,
        symbols: false,
        diagnostics: false,
        chunk_max_size: None,
    });
}

#[test]
fn process_result_shape() {
    assert_json_snapshot!(ProcessResult {
        language: "python".to_string(),
        metrics: FileMetrics {
            total_lines: 2,
            code_lines: 2,
            comment_lines: 0,
            blank_lines: 0,
            total_bytes: 19,
            node_count: 5,
            error_count: 0,
            max_depth: 2,
        },
        structure: vec![StructureItem {
            kind: StructureKind::Function,
            name: Some("foo".to_string()),
            visibility: None,
            span: span(),
            children: vec![],
            decorators: vec![],
            doc_comment: None,
            signature: None,
            body_span: None,
        }],
        imports: vec![],
        exports: vec![],
        comments: vec![],
        docstrings: vec![],
        symbols: vec![],
        diagnostics: vec![],
        chunks: vec![],
    });
}
