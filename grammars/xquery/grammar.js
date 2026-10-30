// https://www.w3.org/TR/xquery-31/#lexical-structure
// When tokenizing, the longest possible match that is consistent with the EBNF
// is used. Keywords are not reserved—that is, any lexical QName may duplicate a
// keyword except as noted in A.3 Reserved Function Names.
/*
// https://www.w3.org/TR/xquery-31/#id-reserved-fn-names
// https://github.com/tree-sitter/tree-sitter/pull/246
//The reserved property
/* In order improve this error recovery, the grammar author needs a way to
explicitly indicate that certain keywords are reserved.
* That is - even if they are not technically valid, they should still be
recognized as separate from any other tokens that would match
* that string (such as an identifier */

const NON_DELIMITING_WORDS = [
  "NaN",
  "allowing",
  "ancestor",
  "ancestor-or-self",
  "and",
  "array",
  "as",
  "ascending",
  "at",
  "attribute",
  "base-uri",
  "boundary-space",
  "by",
  "case",
  "cast",
  "castable",
  "catch",
  "child",
  "collation",
  "comment",
  "construction",
  "context",
  "copy-namespaces",
  "count",
  "decimal-format",
  "decimal-separator",
  "declare",
  "default",
  "descendant",
  "descendant-or-self",
  "descending",
  "digit",
  "div",
  "document",
  "document-node",
  "element",
  "else",
  "empty",
  "empty-sequence",
  "encoding",
  "end",
  "eq",
  "every",
  "except",
  "exponent-separator",
  "external",
  "following",
  "following-sibling",
  "for",
  "function",
  "ge",
  "greatest",
  "group",
  "grouping-separator",
  "gt",
  "idiv",
  "if",
  "import",
  "in",
  "infinity",
  "inherit",
  "instance",
  "intersect",
  "is",
  "item",
  "lax",
  "le",
  "least",
  "let",
  "lt",
  "map",
  "minus-sign",
  "mod",
  "module",
  "namespace",
  "namespace-node",
  "ne",
  "next",
  "no-inherit",
  "no-preserve",
  "node",
  "of",
  "only",
  "option",
  "or",
  "order",
  "ordered",
  "ordering",
  "parent",
  "pattern-separator",
  "per-mille",
  "percent",
  "preceding",
  "preceding-sibling",
  "preserve",
  "previous",
  "processing-instruction",
  "return",
  "satisfies",
  "schema",
  "schema-attribute",
  "schema-element",
  "self",
  "sliding",
  "some",
  "stable",
  "start",
  "strict",
  "strip",
  "switch",
  "text",
  "then",
  "to",
  "treat",
  "try",
  "tumbling",
  "type",
  "typeswitch",
  "union",
  "unordered",
  "validate",
  "variable",
  "version",
  "when",
  "where",
  "window",
  "xquery",
  "zero-digit",
];

module.exports = grammar({
  name: "xquery",
  // Whitespace and Comments function as symbol separators
  extras: ($) => [$.comment, /[\s\p{Zs}\uFEFF\u2060\u200B]/],
  // ~keep Upstream declared a top-level `reserved` array of XQuery keywords,
  // but ~keep XQuery keywords are NOT reserved (any lexical QName may duplicate
  // one, per the ~keep header note). Older tree-sitter ignored the property;
  // tree-sitter 0.26 requires ~keep `reserved` to be an object and would
  // otherwise start reserving these words, ~keep changing parse behaviour.
  // Removed to keep the parser faithful to upstream output.
  word: ($) => $.identifier,
  supertypes: ($) => [
    $._setter,
    $._kind_test,
    $._primary_expr,
    $._value_comp,
    $._node_comp,
    $._general_comp,
    $._node_constructor,
    $._computed_constructor,
    $._item_type,
    $._common_content,
    $._numeric_literal,
  ],
  rules: {
    module: ($) => seq(optional($.version_declaration), choice($.main_module, $.library_module)), // 1
    version_declaration: ($) =>
      seq(
        "xquery",
        choice(
          seq("encoding", $.string_literal),
          seq("version", $.string_literal, optional(seq("encoding", $.string_literal))),
        ),
        ";",
      ), // 2
    library_module: ($) => seq($.module_declaration, $.prolog), // 4
    main_module: ($) => seq(optional($.prolog), $.query_body), // 3
    prolog: ($) =>
      prec.right(
        choice(
          seq($._prolog_part_one, optional($._prolog_part_two)),
          seq(optional($._prolog_part_one), $._prolog_part_two),
        ),
      ), // 6
    module_declaration: ($) => seq("module", $._namespace_define, field("uri", $.string_literal), ";"), // 5
    _namespace_define: ($) => seq("namespace", $._ncname, "="), // repeated seq
    _prolog_part_one: ($) =>
      repeat1(
        seq(
          choice($.default_namespace_declaration, $._setter, $.namespace_declaration, $.module_import, $.schema_import),
          ";",
        ),
      ), // 6
    _prolog_part_two: ($) =>
      repeat1(
        seq(
          choice($.context_item_declaration, $.variable_declaration, $.function_declaration, $.option_declaration),
          ";",
        ),
      ), // 6
    // separator: ';'  // 7
    _setter: ($) =>
      choice(
        $.boundary_space_declaration, // 9
        $.default_collation_declaration, // 10
        $.base_uri_declaration, // 11
        $.construction_declaration, // 12
        $.ordering_mode_declaration, // 13
        $.empty_order_declaration, // 14
        $.copy_namespaces_declaration, // 15
        $.decimal_format_declaration, // 18
      ), // 8
    boundary_space_declaration: (_$) => seq("declare", "boundary-space", choice("preserve", "strip")),
    default_collation_declaration: ($) => seq("declare", "default", "collation", field("uri", $.string_literal)),
    base_uri_declaration: ($) => seq("declare", "base-uri", field("uri", $.string_literal)),
    construction_declaration: (_$) => seq("declare", "construction", choice("preserve", "strip")),
    ordering_mode_declaration: (_$) => seq("declare", "ordering", choice("ordered", "unordered")),
    empty_order_declaration: (_$) => seq("declare", "default", "order", "empty", choice("greatest", "least")),
    copy_namespaces_declaration: (_$) =>
      seq("declare", "copy-namespaces", choice("preserve", "no-preserve"), ",", choice("inherit", "no-inherit")),
    decimal_format_declaration: ($) =>
      prec.right(
        seq(
          "declare",
          choice(seq("decimal-format", $._EQName), seq("default", "decimal-format")),
          repeat($.df_property_define),
        ),
      ),
    df_property_define: ($) =>
      seq(
        choice(
          "decimal-separator",
          "grouping-separator",
          "infinity",
          "minus-sign",
          "NaN",
          "percent",
          "per-mille",
          "zero-digit",
          "digit",
          "pattern-separator",
          "exponent-separator",
        ),
        "=",
        $.string_literal,
      ),
    schema_import: ($) =>
      seq(
        "import",
        "schema",
        optional($.schema_prefix),
        field("uri", $.string_literal),
        optional(seq("at", $.string_literal, repeat(seq(",", $.string_literal)))),
      ), // 21
    schema_prefix: ($) => choice($._namespace_define, seq("default", "element", "namespace")),
    module_import: ($) =>
      seq("import", "module", optional($._namespace_define), field("uri", $.string_literal), optional($.source_at)), // 23
    source_at: ($) => seq("at", $.string_literal, repeat(seq(",", $.string_literal))), // repeated seq
    namespace_declaration: ($) => seq("declare", $._namespace_define, field("uri", $.string_literal)), // 24
    default_namespace_declaration: ($) =>
      seq("declare", "default", choice("element", "function"), "namespace", field("uri", $.string_literal)), // 25
    context_item_declaration: ($) =>
      seq(
        "declare",
        "context",
        "item",
        optional($.type_declaration),
        choice(seq(":=", field("var_value", $._expr)), seq("external", optional(seq(":=", $._expr)))),
      ), // 31
    variable_declaration: ($) =>
      seq(
        "declare",
        repeat($.annotation),
        "variable",
        $.variable,
        optional($.type_declaration),
        choice(seq(":=", field("value", $._expr_single)), seq("external", optional(seq(":=", $._expr_single)))),
      ), // 28
    function_declaration: ($) =>
      seq(
        "declare",
        repeat($.annotation),
        "function",
        $._EQName,
        "(",
        optional($.param_list),
        ")",
        optional(field("return_type", $.type_declaration)),
        field("body", choice($.enclosed_expr, "external")),
      ), // 32
    param_list: ($) => seq($._param, repeat(seq(",", $._param))), // 33
    _param: ($) => seq("$", $._EQName, optional(field("param_type", $.type_declaration))), // 34
    option_declaration: ($) => seq("declare", "option", field("name", $._EQName), field("value", $.string_literal)), // 37
    query_body: ($) => $._expr, // 38
    _expr: ($) => prec(1, seq($._expr_single, repeat(seq(",", $._expr_single)))), // 39
    _expr_single: ($) =>
      choice(
        $.flwor_expr, // 41          prec: 2
        $.quantified_expr, // 70     prec: 2
        $.switch_expr, // 71         prec: 2
        $.typeswitch_expr, // 74      prec: 2
        $.if_expr, // 77             prec: 2
        $.try_catch_expr, // 78      prec: 2
        $.or_expr, // 83             prec: 3
        $.and_expr, // 84            prec: 4
        $.comparison_expr, // 85     prec: 5
        $.string_concat_expr, // 86  prec: 6
        $.range_expr, // 87          prec: 7
        $.additive_expr, // 88       prec: 8
        $.multiplicative_expr, // 89 prec: 9
        $.union_expr, // 90          prec: 10
        $.intersect_except_expr, // 91 prec: 11
        $.instance_of_expr, // 92    prec: 12
        $.treat_expr, // 93          prec: 13
        $.castable_expr, // 94       prec: 14
        $.cast_expr, // 95           prec: 15
        $.arrow_expr, // 96          prec: 16
        $.unary_expr, //  97         prec: 17 '-' '+' arithmetic
        //  prefix right to left */
        $.bang_expr, // 107           prec: 18
        seq($._postfix_expr, optional($.absolute_path_expr)),
        $.absolute_path_expr,
        $.rel_path_expr,
        $._primary_expr,
        //$.qname
      ),
    flwor_expr: ($) => prec(2, seq($._initial_clause, repeat($._intermediate_clause), $.return_clause)), // 41
    _initial_clause: ($) => choice($.for_clause, $.let_clause, $._window_clause), // 42
    _intermediate_clause: ($) =>
      choice($._initial_clause, $.where_clause, $.group_by_clause, $.order_by_clause, $.count_clause), // 43',
    for_clause: ($) => seq("for", $.for_binding, repeat(seq(",", $.for_binding))), // 44',
    for_binding: ($) =>
      seq(
        "$",
        $._EQName,
        optional($.type_declaration),
        optional(seq("allowing", "empty")),
        optional($._positional_var),
        "in",
        field("binding_sequence", $._expr_single),
      ), // 45
    let_clause: ($) => seq("let", $.let_binding, repeat(seq(",", $.let_binding))), // 48
    let_binding: ($) => seq("$", $._EQName, optional($.type_declaration), ":=", $._expr_single), // 49
    _window_clause: ($) => choice($.tumbling_window_clause, $.sliding_window_clause),
    tumbling_window_clause: ($) =>
      seq(
        "for",
        "tumbling",
        "window",
        $.variable,
        optional($.type_declaration),
        "in",
        $._expr_single,
        $.window_start_condition,
        optional($.window_end_condition),
      ),
    sliding_window_clause: ($) =>
      seq(
        "for",
        "sliding",
        "window",
        $.variable,
        optional($.type_declaration),
        "in",
        $._expr_single,
        $.window_start_condition,
        optional($.window_end_condition),
      ),
    window_start_condition: ($) =>
      seq(
        "start",
        optional(field("current_item", $.variable)),
        optional($._positional_var),
        optional(seq("previous", field("previous_item", $.variable))),
        optional(seq("next", field("next_item", $.variable))),
        "when",
        $._expr_single,
      ),
    window_end_condition: ($) =>
      seq(
        optional("only"),
        "end",
        optional(field("current_item", $.variable)),
        optional($._positional_var),
        optional(seq("previous", field("previous_item", $.variable))),
        optional(seq("next", field("next_item", $.variable))),
        "when",
        $._expr_single,
      ),
    _positional_var: ($) => seq("at", field("positional_variable", $.variable)),
    current_item: ($) => seq("$", $._EQName),
    previous_item: (_$) => seq(""),
    next_item: (_$) => seq(""),
    count_clause: ($) => seq("count", $.variable), //   59
    where_clause: ($) => seq("where", $._expr_single), // 60
    group_by_clause: ($) => seq("group", "by", $.grouping_spec, repeat(seq(",", $.grouping_spec))), // 61
    grouping_spec: ($) =>
      seq(
        field("grouping_var", $.variable),
        optional(seq(optional($.type_declaration), ":=", $._expr_single)),
        optional(seq("collation", field("uri", $.string_literal))),
      ), // 63
    order_by_clause: ($) => prec.left(seq(optional("stable"), "order", "by", $._order_spec_list)), // 65
    _order_spec_list: ($) => seq($._order_spec, repeat(seq(",", $._order_spec))), // 66
    _order_spec: ($) =>
      seq(
        field("order_expr", $._expr_single),
        field(
          "order_modifier",
          seq(optional($.order_direction), optional($.order_length), optional($.order_collation)),
        ),
      ), // 67
    order_direction: (_$) => seq(choice("ascending", "descending")),
    order_length: (_$) => seq("empty", choice("greatest", "least")),
    order_collation: ($) => seq("collation", field("uri", $.string_literal)),
    return_clause: ($) => seq("return", $._expr_single), // 69
    // end flwor
    quantified_expr: ($) =>
      prec(
        2,
        seq(
          choice("some", "every"),
          field("quantifier", $.variable),
          optional($.type_declaration),
          "in",
          field(
            "in_binding",
            seq($._expr_single, repeat(seq(",", $.variable, optional($.type_declaration), "in", $._expr_single))),
          ),
          "satisfies",
          field("satisfy_conditional", $._expr_single),
        ),
      ), // 70
    switch_expr: ($) =>
      prec(
        2,
        seq(
          "switch",
          field("switch_operand", seq("(", $._expr, ")")),
          repeat1($.switch_clause),
          field("switch_default", seq("default", "return", field("default_return", $._expr_single))),
        ),
      ), // 71
    switch_clause: ($) =>
      seq(repeat1(seq("case", field("case_operand", $._expr_single))), "return", field("case_return", $._expr_single)), // 72
    typeswitch_expr: ($) =>
      prec(
        2,
        seq(
          "typeswitch",
          field("operand", seq("(", $._expr, ")")),
          repeat1($.typeswitch_case_clause),
          seq("default", optional($.variable), "return", $._expr_single),
        ),
      ), // 74
    typeswitch_case_clause: ($) =>
      seq(
        "case",
        optional(seq($.variable, "as")),
        $.sequence_type,
        repeat(seq("|", $.sequence_type)),
        "return",
        $._expr_single,
      ), // 75
    if_expr: ($) =>
      prec(
        2,
        seq(
          "if",
          field("if_test", seq("(", $._expr, ")")),
          "then",
          field("if_consequence", $._expr_single),
          "else",
          field("if_alternative", $._expr_single),
        ),
      ), // 77
    try_catch_expr: ($) => prec(2, seq($.try_clause, $.catch_clause)), // 78
    try_clause: ($) => seq("try", $.enclosed_expr), // 79
    catch_clause: ($) => seq("catch", $.catch_error_list, $.enclosed_expr), // 81
    catch_error_list: ($) => seq($.name_test, repeat(seq("|", $.name_test))), // 82
    or_expr: ($) => prec.left(3, seq(field("lhs", $._expr_single), "or", field("rhs", $._expr_single))), // 83
    and_expr: ($) => prec.left(4, seq(field("lhs", $._expr_single), "and", field("rhs", $._expr_single))), // 84
    comparison_expr: ($) =>
      prec.left(5, seq(field("lhs", $._expr_single), $._comparison_ops, field("rhs", $._expr_single))), // 85
    _comparison_ops: ($) => choice($._value_comp, $._general_comp, $._node_comp),
    _value_comp: (_$) => choice("eq", "ne", "lt", "le", "gt", "ge"), // 100
    _general_comp: (_$) => choice("=", "!=", "<", "<=", ">", ">="), // 99
    _node_comp: (_$) => choice("is", "<<", ">>"), // 101
    string_concat_expr: ($) =>
      prec.left(6, seq(field("lhs", $._expr_single), repeat1(seq("||", field("rhs", $._expr_single))))), // 86
    range_expr: ($) => prec.left(7, seq(field("lhs", $._expr_single), "to", field("rhs", $._expr_single))), // 87
    additive_expr: ($) =>
      prec.left(8, seq(field("lhs", $._expr_single), repeat1(seq(choice("+", "-"), field("rhs", $._expr_single))))), // 88
    multiplicative_expr: ($) =>
      prec.left(
        9,
        seq(
          field("lhs", $._expr_single),
          repeat1(seq(choice("*", "div", "idiv", "mod"), field("rhs", $._expr_single))),
        ),
      ),
    union_expr: ($) =>
      prec.left(
        10,
        seq(field("lhs", $._expr_single), repeat1(seq(choice("union", "|"), field("rhs", $._expr_single)))),
      ), // 90
    intersect_except_expr: ($) =>
      prec.left(
        11,
        seq(field("lhs", $._expr_single), repeat1(seq(choice("intersect", "except"), field("rhs", $._expr_single)))),
      ), // 91
    instance_of_expr: ($) =>
      prec.left(12, seq(field("lhs", $._expr_single), "instance", "of", field("rhs", $.sequence_type))), // 92
    treat_expr: ($) => prec(13, seq(field("lhs", $._expr_single), "treat", "as", field("rhs", $.sequence_type))), // 93
    castable_expr: ($) =>
      prec(
        14,
        seq(
          field("lhs", $._expr_single),
          "castable",
          "as",
          field("simple_type", seq($._EQName, optional(token.immediate("?")))),
        ),
      ), // 94
    cast_expr: ($) =>
      prec(
        14,
        seq(
          field("lhs", $._expr_single),
          "cast",
          "as",
          field("simple_type", seq($._EQName, optional(token.immediate("?")))),
        ),
      ), // 94
    arrow_expr: ($) =>
      prec.left(16, seq(field("expression_value", $._expr_single), repeat1(seq("=>", $.arrow_function)))), // 94
    arrow_function: ($) => seq(choice($._EQName, $.var_ref, $.parenthesized_expr), $.arg_list), // 127
    unary_expr: ($) => prec.right(17, seq(choice("+", "-"), $._expr_single)),
    bang_expr: ($) =>
      prec.left(
        18,
        seq(field("sequence", $._expr_single), repeat1(seq("!", field("dynamic_context", $._expr_single)))),
      ), // 107
    _postfix_expr: ($) => prec.left(20, seq($._primary_expr, repeat($._postfix))),
    _postfix: ($) =>
      seq(
        choice(
          prec.left(20, field("filter_expr", $.predicate)),
          prec.left(20, field("dynamic_function_call", $.arg_list)),
          prec.left(20, $.postfix_lookup),
        ),
      ),
    predicate: ($) => prec.left(20, seq("[", $._expr_single, "]")), // 124
    postfix_lookup: ($) =>
      prec.left(
        20,
        seq("?", field("key", choice($._ncname, $.integer_literal, $.parenthesized_expr, alias("*", $.wildcard)))),
      ),
    // A path expression that starts with "/" or "//"is often referred to as an
    // absolute path expression
    absolute_path_expr: ($) => prec.left(19, choice(seq(choice("//", "/"), $.rel_path_expr), "/")),
    rel_path_expr: ($) => prec.left(19, seq($._step_expr, repeat(seq(choice("/", "//"), $._step_expr)))),
    _step_expr: ($) => prec.left(19, seq(choice($._postfix_expr, $._axis_step))),
    _axis_step: ($) =>
      seq(field("axis", choice($._forward_step, $._reverse_step)), repeat(prec.left(20, field("filter", $.predicate)))), //  predicate within steps 123
    _reverse_step: ($) => choice($.reverse_axis, $.abbrev_reverse_step),
    abbrev_reverse_step: (_$) => "..",
    _forward_step: ($) => choice($.forward_axis, $.abbrev_forward_step),
    forward_axis: ($) =>
      seq(
        choice("child", "descendant", "attribute", "self", "descendant-or-self", "following-sibling", "following"),
        "::",
        $._node_test,
      ), // 113
    reverse_axis: ($) =>
      seq(choice("parent", "ancestor", "preceding-sibling", "preceding", "ancestor-or-self"), "::", $._node_test), // 116
    abbrev_forward_step: ($) => prec.right(seq(optional("@"), $._node_test)),
    _node_test: ($) => choice($.name_test, $._kind_test), // 118
    name_test: ($) => choice($._EQName, $.wildcard), //  199
    wildcard: ($) => choice("*", seq($._ncname, ":*"), seq("*:", $._ncname), seq($.braced_uri_literal, "*")), // 120
    _primary_expr: ($) =>
      choice(
        $._literal,
        $.var_ref,
        $.parenthesized_expr, // 133
        $.context_item_expr, // 134 LSP If the context item is
        // absent, a context item expression
        // raises a dynamic error
        $.function_call, // 137
        $.ordered_expr, // 135
        $.unordered_expr, // 136
        $._node_constructor, // 140 node constructors 140
        $._func_constructors,
        field("constructor", $.string_constructor), // 177
        $.unary_lookup, // 181
      ),
    _node_constructor: ($) => choice($.direct_constructor, $._computed_constructor), // 140
    _func_constructors: ($) =>
      field(
        "constructor",
        choice(
          $.function_item_expr, // 167
          $.map_constructor, // 170
          $.square_array_constructor, // 175
          // array_constructor
          // 174
          $.curly_array_constructor, // 176
          // array_constructor
          // 174
        ),
      ),
    direct_constructor: ($) => choice($.direct_element, $.direct_comment), // 141
    direct_comment: (_$) => seq("<!--", repeat(/[^->]|[^-]>|[^-]->/), "-->"), // 149
    // 1 '[^->] any symbol except reserved
    //'2 [^-]> allow > when not a comment end
    // 3 [^-]->  allow - when not a comment comment end
    // 4 -[^>]   allow - when not follows by > ~keep
    // TODO
    direct_element: ($) => choice(seq($.start_tag, repeat($._direct_element_content), $.end_tag), $.empty_tag),
    _direct_element_content: ($) => choice($.direct_constructor, $._common_content, $._element_content_char),
    _element_content_char: ($) => field("content", alias(/[^{}<&]+/, $.char_data)),
    start_tag: ($) => seq("<", $._allowed_qnames, repeat($.direct_attribute), ">"),
    end_tag: ($) => seq("</", $._allowed_qnames, ">"),
    empty_tag: ($) => seq("<", $._allowed_qnames, repeat($.direct_attribute), "/>"),
    direct_attribute: ($) => seq($._allowed_qnames, "=", $.attribute_value),
    attribute_value: ($) => choice($._attr_quote_value, $._attr_apos_value),
    _attr_quote_value: ($) =>
      seq('"', repeat(choice($._common_content, $.escape_quote, alias(/[^"{}<&]+/, $.char_data))), '"'),
    _attr_apos_value: ($) =>
      seq("'", repeat(choice($._common_content, $.escape_apos, alias(/[^'{}<&]+/, $.char_data))), "'"),
    _common_content: ($) =>
      field("content", choice($.predefined_entity_ref, $.char_ref, $.escape_enclosed, $.enclosed_expr)),
    escape_enclosed: (_$) => field("content", choice("{{", "}}")),
    _computed_constructor: ($) =>
      field(
        "computed_constructor",
        choice(
          $.comp_doc_constructor, // 156
          $.comp_elem_constructor, // 157
          $.comp_attr_constructor, // 158
          $.comp_namespace_constructor, // 160
          $.comp_text_constructor, // 164
          $.comp_comment_constructor, // 165
          $.comp_pi_constructor, // 166
        ),
      ), // 155
    comp_doc_constructor: ($) => seq("document", field("content", $.enclosed_expr)), // 156
    comp_elem_constructor: ($) => prec.left(2, seq("element", $._construct)), // 157
    comp_attr_constructor: ($) => prec.left(2, seq("attribute", $._construct)), // 158
    _construct: ($) => seq(choice($._EQName, seq("{", $._expr, "}")), field("content", $.enclosed_expr)),
    comp_text_constructor: ($) => seq("text", field("content", $.enclosed_expr)),
    comp_comment_constructor: ($) => seq("comment", field("content", $.enclosed_expr)),
    comp_pi_constructor: ($) =>
      seq(
        "processing-instruction",
        field("name", choice($._ncname, seq("{", $._expr, "}"))),
        field("content", $.enclosed_expr),
      ), // 166
    comp_namespace_constructor: ($) =>
      seq("namespace", field("name", choice($._ncname, seq("{", $._expr, "}"))), field("content", $.enclosed_expr)), // 160
    _literal: ($) => choice($._numeric_literal, $.string_literal),
    var_ref: ($) => seq("$", $._var_name), // 131
    parenthesized_expr: ($) => seq("(", optional($._expr), ")"), // 133
    context_item_expr: (_$) => ".",
    function_call: ($) => seq($._EQName, $.arg_list), // grammar-note: parens
    // lookahead comment pragma
    arg_list: ($) => seq("(", optional(seq($._argument, repeat(seq(",", $._argument)))), ")"), // 122
    _argument: ($) => field("arg", choice($._expr_single, $.placeholder)),
    placeholder: (_$) => "?",
    ordered_expr: ($) => seq("ordered", $.enclosed_expr), // 135
    unordered_expr: ($) => seq("unordered", $.enclosed_expr), // 136
    unary_lookup: ($) =>
      prec.left(
        21,
        seq("?", field("key", choice($._ncname, $.integer_literal, $.parenthesized_expr, alias("*", $.wildcard)))),
      ), // 181
    function_item_expr: ($) => choice($.named_function_ref, $.inline_function_expr), // 167
    named_function_ref: ($) => seq($._EQName, token.immediate("#"), field("signature", $.integer_literal)), // 168
    inline_function_expr: ($) =>
      seq(
        optional($.annotation),
        "function",
        "(",
        optional($.param_list),
        ")",
        optional(field("return_type", $.type_declaration)),
        field("body", $.enclosed_expr),
      ), // 169
    // 2.5.4 SequenceType Syntax
    type_declaration: ($) => seq("as", $.sequence_type), // 183
    sequence_type: ($) =>
      choice(seq("empty-sequence", "(", ")"), prec.right(seq($._item_type, optional($.occurrence_indicator)))), // 184
    _item_type: ($) =>
      choice(
        $._kind_test,
        $.any_item,
        $._func_test, // maps arrays are functions
        $.atomic_or_union_type,
        $.parenthesized_item_type, // 216
      ), // 186
    occurrence_indicator: (_$) => choice("?", "*", "+"), // 185
    atomic_or_union_type: ($) => $._EQName, // 187
    _func_test: ($) =>
      field(
        "func_test",
        choice(
          $.any_function_test,
          $.typed_function_test,
          $.any_map_test,
          $.typed_map_test,
          $.any_array_test,
          $.typed_array_test,
        ),
      ),
    any_item: (_$) => seq("item", "(", ")"),
    _kind_test: ($) =>
      field(
        "kind_test",
        choice(
          $.document_test,
          $.element_test,
          $.attribute_test,
          $.schema_element_test,
          $.schema_attribute_test,
          $.pi_test,
          $.any_kind_test,
          $.comment_test,
          $.namespace_node_test,
          $.text_test,
        ),
      ),
    any_kind_test: (_$) => seq("node", token.immediate("("), ")"), // 189
    text_test: (_$) => seq("text", token.immediate("("), ")"), // 191
    comment_test: (_$) => seq("comment", "(", ")"), // 192
    namespace_node_test: (_$) => seq("namespace-node", "(", ")"), // 193
    document_test: ($) => seq("document-node", "(", optional(choice($.element_test, $.schema_element_test)), ")"), // 190
    element_test: ($) => seq("element", seq("(", optional($.element_test_params), ")")), // 199
    element_test_params: ($) =>
      seq(
        field("name", choice(alias("*", $.wildcard), $._EQName)),
        optional(seq(",", field("type", seq($._EQName, optional(alias("?", $.occurrence_indicator)))))),
      ),
    // same as element but no nilled test as attributes don't have children
    attribute_test: ($) =>
      seq(
        "attribute",
        "(",
        optional(seq($._attrib_name_or_wildcard, optional(seq(",", field("type_name", $._EQName))))),
        ")",
      ), // 195
    _attrib_name_or_wildcard: ($) => choice(field("attribute_name", $._EQName), alias("*", $.wildcard)), // 196
    schema_element_test: ($) => seq("schema-element", "(", field("element_name", $._EQName), ")"), // 197
    schema_attribute_test: ($) => seq("schema-attribute", "(", field("attribute_name", $._EQName), ")"), // 201
    pi_test: ($) =>
      seq("processing-instruction", seq("(", optional(field("param", choice($._ncname, $.string_literal))), ")")), // 194
    any_function_test: ($) => seq(repeat($.annotation), "function", seq("(", alias("*", $.wildcard), ")")), //  // 207
    typed_function_test: ($) =>
      seq(
        repeat($.annotation),
        "function",
        "(",
        $.sequence_type,
        repeat(seq(",", $.sequence_type)),
        ")",
        $.type_declaration,
      ),
    any_map_test: ($) => seq("map", "(", alias("*", $.wildcard), ")"), // 210
    typed_map_test: ($) => seq("map", "(", $.atomic_or_union_type, ",", $.sequence_type, ")"), // 212
    any_array_test: ($) => seq("array", "(", alias("*", $.wildcard), ")"),
    typed_array_test: ($) => seq("array", "(", $.sequence_type, ")"),
    parenthesized_item_type: ($) => seq("(", $._item_type, ")"), // 216
    // END SequenceType Syntax
    annotation: ($) => seq("%", $._EQName, optional(seq("(", $._literal, repeat(seq(",", $._literal)), ")"))), // 27
    enclosed_expr: ($) => seq("{", optional($._expr), "}"), // 5
    map_constructor: ($) =>
      prec.left(2, seq("map", seq("{", optional($.map_entry), repeat(seq(",", $.map_entry)), "}"))), // 170
    map_entry: ($) => seq(field("key", $._expr_single), ":", field("value", $._expr_single)),
    curly_array_constructor: ($) => seq("array", field("content", $.enclosed_expr)),
    square_array_constructor: ($) => seq("[", optional($._expr_single), repeat(seq(",", $._expr_single)), "]"),
    string_constructor: ($) =>
      seq("``[", $.string_constructor_chars, repeat(seq($.interpolation, $.string_constructor_chars)), "]``"), // 177
    string_constructor_chars: (_$) => repeat1(/[^`{\]]|[\]][^`]|[\]]`[^`]|[^`][{]|[{][`{\]]|`[^{]/),
    // 1  [^`{\]] // any symbol except reserved
    // 2 [\]][^`] allow closing ] which is not a constructor end
    // 3 [\]]`[^`] allow closing seq ]` without second ` which is not a
    // constructor end 4 [^`][{] allow { if not opening interpolation seq 5
    // [{][`{\]] allow standalone { or {` or {{ or {[ which is not a
    // interpolation start 6 `[^{] allow standalone ` which is not a
    // interpolation start
    interpolation: ($) => seq("`{", $._expr, "}`"), // 180',
    string_literal: ($) => choice($._string_quote, $._string_apos),
    _string_quote: ($) => seq('"', repeat($._string_quote_content), '"'),
    _string_quote_content: ($) =>
      choice($.predefined_entity_ref, $.char_ref, $.escape_quote, alias(/[^"&]+/, $.char_data)),
    _string_apos: ($) => seq("'", repeat($._string_apos_content), "'"),
    _string_apos_content: ($) =>
      choice($.predefined_entity_ref, $.char_ref, $.escape_apos, alias(/[^'&]+/, $.char_data)),
    predefined_entity_ref: (_$) => /&(lt|gt|amp|quot|apos);/,
    escape_quote: (_$) => '""',
    escape_apos: (_$) => "''",
    char_ref: (_$) => /&#[0-9]+;|&#x[0-9a-fA-F]+;/,
    _numeric_literal: ($) => choice($.double_literal, $.decimal_literal, $.integer_literal),
    double_literal: (_$) => /(\.\d+)|(\d+\.\d*|\d+)[eE][+-]{0,1}\d+/,
    decimal_literal: (_$) => /(\.\d+)|(\d+\.\d*)/,
    integer_literal: (_$) => /\d+/,
    variable: ($) => seq("$", $._var_name),
    _var_name: ($) => $._EQName,
    _EQName: ($) => choice($._allowed_qnames, $.uri_qualified_name),
    _allowed_qnames: ($) =>
      choice(
        field("ncname", $._ncname),
        seq(field("prefixed", $._ncname), token.immediate(":"), field("local", $._ncname)),
      ),
    _ncname: ($) => choice($.identifier, alias($._non_delimiting_word, $.identifier)),
    uri_qualified_name: ($) => seq(field("braced_uri_literal", alias($.braced_uri_literal, $.identifier)), $._ncname), // ws explicitly
    braced_uri_literal: (_$) =>
      seq("Q{", repeat1(/&(#[0-9]+|#x[0-9a-fA-F]+);|&(lt|gt|amp|quot|apos);|[^&{}]/), token.immediate("}")),
    _non_delimiting_word: ($) => prec.right(alias(choice(...NON_DELIMITING_WORDS), $.identifier)),
    char_data: (_$) => /[^\p{C}]/,
    // ^\p{C}  no invisible control characters and unused code points

    identifier: (_$) => /[_\p{XID_Start}][-_\p{XID_Continue}]*/,
    comment: ($) =>
      seq("(:", repeat1(choice($.comment, prec.right(/[^:()]|[^:][)]|[(][^:]|[:][^)]/))), token.immediate(":)")),
    //'[^:()]', // any symbol except reserved
    //'[^:][)]', // allow closing peren which is not a comment end
    // '[(][^:]', // allow opening peren which is not a comment start
    // '[:][^)]', // allow ':' which is not a comment end
  },
});
