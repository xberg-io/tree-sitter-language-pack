// ~keep Modified by tree-sitter-language-pack (see ATTRIBUTIONS.md): the spaced relational and
// ~keep shift operators are lexed by the external scanner instead of space-prefixed tokens.
const PREC = {
  PAT_VARIANT: 103,
  PAT_VARIANT_NULLARY: 102,
  PAT_OPTION: 101,

  TRY: 0,
  IF: 0,
  LOOP: 0,
  ELSE: 1,
  CATCH: 1,
  WHILE: 1,
  FINALLY: 2,
  ASSIGN: 2,
  COLON: 3,
  IMPLIES: 4,
  PIPE: 5,
  OR: 6,
  AND: 7,
  RELOP: 8,
  ADDSUB: 9,
  CONCAT: 9,
  ADDSUBWRAP: 9,
  MULDIV: 10,
  MOD: 10,
  MULWRAP: 10,
  BOR: 21,
  BAND: 22,
  BXOR: 23,
  BSHIFT: 24,
  BROTATE: 24,
  EXP: 25,
  EXPWRAP: 25,
  INDEX: 32,
  CALL: 32,

  TYPEASSIGN: 29,
  TYPEAND: 30,
  TYPEOR: 31,
};

const _list1 = (token, sep) => seq(token, _rlist(token, sep), optional(sep));
const _list = (token, sep) => optional(_list1(token, sep));
// const _list_s = (token, sep) => repeat(seq(
//   token,
//   sep
// ));
const _rlist = (token, sep) => repeat(seq(sep, token));
const _binop = (token, op) => seq(field("left", token), field("op", op), field("right", token));
// const _annotation = (name, token) => seq(":", field("type_annotation", token));
const _fish = (token) => seq("<", _list(token, ","), ">");

// const WHITESPACE = /[ \r\n\t\f]/;
const NAME = /[a-zA-Z_][0-9a-zA-Z_]*/;
const DECIMAL_DIGIT = /[0-9]/;
const HEX_DIGIT = /[0-9a-f-A-F]/;

// const ASCII = /[\x00-\x7f]/u;
// const ASCII_NO_NL = /[\x00-\x09\x0b-\x7f]/u;

const ESCAPE = /[nrt\\'"]/;

const DECIMAL_NUM = /[0-9][0-9_]*/;
const HEX_NUM = /0x[0-9a-fA-F][0-9a-fA-F_]*/;

const FLOAT = [
  /[0-9][0-9_]*\.(?:[0-9][0-9_]*)?/,
  /[0-9][0-9_]*(?:\.(?:[0-9][0-9_]*)?)?(?:e|E)(?:\+|-)?[0-9][0-9_]*/,
  /0x[0-9a-fA-F][0-9a-fA-F_]*\.(?:[0-9a-fA-F][0-9a-fA-F_]*)/,
  /0x[0-9a-fA-F][0-9a-fA-F_]*(?:\.(?:[0-9a-fA-F][0-9a-fA-F_]*)?)?(?:p|P)(?:\+|-)?[0-9a-fA-F][0-9a-fA-F_]*/,
];

// const UTF8_ENC = [
//   /[\xc2-\xdf][\x80-\xbf]/u,
//   /[\xe0][\xa0-\xbf][\x80-\xbf]/u,
//   /[\xed][\x80-\x9f][\x80-\xbf]/u,
//   /[\xe1-\xec\xee-\xef][\x80-\xbf]{2}/u,
//   /[\xf0][\x90-\xbf][\x80-\xbf]{2}/u,
//   /[\xf4][\x80-\x8f][\x80-\xbf]{2}/u,
//   /[\xf1-\xf3][\x80-\xbf]{3}/u
// ];

/*
Two variants of very similar pattern-like rules exist.
The rules are invariant except for the 'wrapped name node type' (i.e. "xyz") of:
  (xyz (name))
In a function parameter context, this is "parameter".
In a variable binding context, the field name is "variable".
This trivializes the scenario wherein a user would have to do an ascending series
of compares to confirm whether or not a given pattern name refers to a function
parameter or a variable binding. Through aliasing, the two parallel classes become
almost indistinguishable.
*/
const make_patternlike_rule_class = (wrapped_name_node_type, suf) => {
  const nonstd = suf !== "";

  const nmap = Object.fromEntries(
    [
      "pat",
      "_pat_bin_raw",
      "pat_bin",
      "_pat_or",
      "_pat_annotated",
      "_pat_un_raw",
      "pat_un",
      /*"_pat_variant",*/ "_pat_variant_nullary",
      "_pat_option",
      "_pat_nullary",
      "pat_obj",
      "pat_field",
      "_pat_plain",
      "pat_tuple",
    ].map((rule_name) => [
      rule_name,
      `${nonstd && rule_name.charAt(0) !== "_" ? "_" : ""}${rule_name}${suf ? `_${suf}` : ""}`,
    ]),
  );

  const vmapf = ($, key) => {
    return nonstd && key.charAt(0) !== "_" ? alias($[nmap[key]], $[key]) : $[nmap[key]];
  };

  return {
    [nmap.pat]: ($) => vmapf($, "_pat_bin_raw"),

    [nmap._pat_bin_raw]: ($) => choice(vmapf($, "_pat_un_raw"), vmapf($, "pat_bin")),

    [nmap.pat_bin]: ($) => choice(vmapf($, "_pat_or"), vmapf($, "_pat_annotated")),
    [nmap._pat_or]: ($) => prec.left(1, _binop(vmapf($, "_pat_bin_raw"), "or")),
    [nmap._pat_annotated]: ($) => seq(vmapf($, "_pat_bin_raw"), $._annotation),

    [nmap._pat_un_raw]: ($) => choice(vmapf($, "_pat_nullary"), vmapf($, "pat_un")),

    [nmap.pat_un]: ($) =>
      choice(
        $.pat_variant,
        alias($[nmap._pat_variant_nullary], $.pat_variant),
        vmapf($, "_pat_option"),
        seq($.unop, $.lit),
      ),
    // [nmap._pat_variant]: $ => seq(
    //   "#",
    //   field("variant_name", $.name)),
    [nmap._pat_variant_nullary]: ($) =>
      prec.right(PREC.PAT_VARIANT_NULLARY, seq("#", field("variant_name", $.name), vmapf($, "_pat_nullary"))),
    [nmap._pat_option]: ($) =>
      prec.right(PREC.PAT_OPTION, seq(field("unary", "?"), field("option", vmapf($, "_pat_un_raw")))),

    [nmap._pat_nullary]: ($) => choice(vmapf($, "_pat_plain"), vmapf($, "pat_obj")),

    [nmap.pat_obj]: ($) => seq("{", _list(vmapf($, "pat_field"), ";"), "}"),
    [nmap.pat_field]: ($) =>
      seq(
        field("field_name", $.name),
        optional(seq(":", field("field_type_value", $._typ))),
        optional(seq("=", field("value_pattern", vmapf($, "pat")))),
      ),

    [nmap._pat_plain]: ($) =>
      choice(
        $.placeholder,
        $[wrapped_name_node_type],
        // field(name_field, $.name),
        $.lit,
        vmapf($, "pat_tuple"),
      ),

    [nmap.pat_tuple]: ($) => seq("(", _list(vmapf($, "_pat_bin_raw"), ","), ")"),
  };
};

module.exports = grammar({
  name: "motoko",

  supertypes: ($) => [$.dec_nonvar, $.exp_nondec, $.lit, $.exp_nullary, $.typ_nullary],
  conflicts: ($) => [
    // [$.exp_field, $.exp_nullary],
    // [$._exp_obj_compose, $._exp_un_raw],
    // [$._dec_var_typ, $._exp_field_typ],
    // [$._typ_func_short_fish, $._typ_func_fish],
    // [$.dec_var, $.exp_field],
  ],
  word: ($) => $.name,
  extras: ($) => [" ", "\t", "\f", "\r", "\n", $.block_comment, $.line_comment, $.doc_comment],
  inline: ($) => [$.character, $.exp_nonvar, $.exp_nest],
  externals: ($) => [
    $._text_literal_begin,
    $._text_literal_content,
    $._text_literal_end,
    $._block_comment_begin,
    $._block_comment_content,
    $._block_comment_end,
    $._line_comment_begin,
    $._line_comment_content,
    $._doc_comment_begin,
    $._doc_comment_content,
    // ~keep `>`, `<` and `>>` as binary operators need whitespace before them (and `>`/`<`
    // ~keep after). A token that starts with a space makes the generated lexer consume, not
    // ~keep skip, spaces before every token, so names would carry them ("func f" -> " f").
    $._spaced_gt,
    $._spaced_lt,
    $._spaced_shr,
  ],

  rules: {
    prog: ($) => seq(_list($.imp, ";"), _list($._dec, ";")),

    imp: ($) => seq("import", field("module_pattern", $._pat_nullary), optional("="), field("module_path", $.text)),

    dec_field: ($) => seq(optional(field("visibility", $.vis)), optional(field("stability", $.stab)), $._dec),

    _dec: ($) => choice($.dec_var, $.dec_nonvar, $.exp_nondec, $.dec_let_else),

    block: ($) => seq("{", _list($._dec, ";"), "}"),

    dec_var: ($) => seq("var", field("variable_name", $.name), optional($._annotation), "=", field("value", $._exp)),
    // _dec_var_typ: $ => _annotation("var", $._typ),

    dec_nonvar: ($) => choice($.dec_let, $.dec_type, $.dec_obj, $.dec_func, $.dec_class),

    dec_let: ($) => seq("let", field("binding_pattern", $.pat), "=", field("value", $._exp)),

    dec_let_else: ($) =>
      seq(
        "let",
        field("binding_pattern", $.pat),
        "=",
        field("value", $._exp),
        "else",
        field("fallback_value", $.exp_nest),
      ),

    dec_type: ($) =>
      seq("type", field("type_name", $.name), optional($._dec_type_fish), "=", field("type_value", $._typ)),
    _dec_type_fish: ($) => _fish($.typ_bind),

    _annotation: ($) => seq(":", field("type_annotation", $._typ)),

    dec_obj: ($) =>
      seq(
        field("sort", $.obj_sort),
        optional(field("object_name", $.name)),
        optional($._annotation),
        optional("="),
        $._obj_body,
      ),
    _obj_body: ($) => seq("{", _list($.dec_field, ";"), "}"),

    dec_func: ($) =>
      prec(
        1,
        seq(
          optional(field("opt", $.shared_pat_opt)),
          "func",
          optional(field("function_name", $.name)),
          // optional($._dec_func_fish),
          optional($.typ_params_opt),
          field("parameter_pattern", $._pat_plain_fnparam),
          optional($._annotation),
          optional("="),
          field("body", $.exp_nest),
        ),
      ),

    parameter: ($) => field("parameter_name", $.name),

    dec_class: ($) =>
      seq(
        optional(field("opt", $.shared_pat_opt)),
        optional(field("sort", $.obj_sort)),
        "class",
        optional(field("class_name", $.name)),
        optional($.typ_params_opt),
        field("constructor_pattern", $._pat_plain_fnparam),
        optional($._annotation),
        $._class_body,
      ),
    _class_body: ($) => choice(seq("=", optional(field("object_name", $.name)), $._obj_body), $._obj_body),

    object: ($) => seq("{", choice($._exp_obj_fields, $.obj_composite), "}"),
    _exp_obj_fields: ($) => _list1($.exp_field, ";"),

    // prec 1 to resolve _exp_obj_compose and _exp_un_raw conflict
    // we want to prefer object composition when it is possible
    obj_composite: ($) =>
      prec(1, seq($._exp_post_raw, repeat($.obj_combination), choice($.obj_combination, $.obj_extension))),
    obj_combination: ($) => seq("and", $._exp_post_raw),
    obj_extension: ($) => seq("with", $._exp_obj_fields),
    // prec 2 to resolve exp_field and exp_nullary conflict
    // we want to interpret as object when it is possible
    exp_field: ($) =>
      prec(
        2,
        seq(
          optional(token(prec(2, "var"))),
          field("field_name", $.name),
          optional($._annotation),
          optional(seq("=", field("value", $._exp))),
        ),
      ),

    tuple: ($) => seq("(", _list($._exp, ","), ")"),

    exp_nullary: ($) =>
      choice(
        $.object,
        // $.exp_plain,
        $.lit,
        $.tuple,
        $.variable,
        // field("variable_name", $.name),
        $.placeholder,
      ),

    variable: ($) => choice($._named_variable),
    _named_variable: ($) => field("variable_name", $.name),

    _exp_post_raw: ($) => choice($.exp_nullary, $.array_exp, $.call_exp, $.system_actor_class_exp, $.exp_post),

    exp_post: ($) => choice(alias($._indexer, $.member), $.member, $.null_break_exp),

    _indexer: ($) =>
      prec.left(
        PREC.INDEX,
        seq(field("indexed_value", $._exp_post_raw), field("indexer_value", $._exp_member_bracket)),
      ),
    _exp_member_bracket: ($) => seq("[", $._exp, "]"),

    member: ($) => seq(field("indexed_value", $._exp_post_raw), ".", field("key", $._exp_member_dot)),
    _exp_member_dot: ($) => choice($.nat, $.name),

    null_break_exp: ($) => prec.left(seq(field("value", $._exp_post_raw), field("unary", "!"))),

    array_exp: ($) => seq("[", optional("var"), _list($.exp_nonvar, ","), "]"),

    call_exp: ($) =>
      prec.left(
        PREC.CALL,
        seq(
          field("invoked_value", $._exp_post_raw),
          // optional($._call_fish),
          optional($._inst),
          field("argument_value", $.exp_nullary),
        ),
      ),
    // _call_fish: $ => seq("<", _list($._typ, ","), ">"),

    system_actor_class_exp: ($) =>
      seq("(", "system", field("library_value", $._exp_post_raw), ".", field("field_name", $.name), ")"),
    unop: () => choice("-", "+", "^"),
    unassign: () => choice("+=", "-=", "^="),

    _exp_un_raw: ($) => choice($._exp_post_raw, $.exp_variant, $.exp_un),

    exp_un: ($) =>
      choice($._exp_option, $._exp_unassign, $._exp_unop, $.actor, $.not, $.debug_show, $.to_candid, $.from_candid),
    exp_variant: ($) => seq("#", field("variant_name", $.name), optional(field("value", $.exp_nullary))),
    _exp_option: ($) => prec.right(seq(field("unary", "?"), $._exp_un_raw)),
    _exp_unop: ($) => prec.right(seq($.unop, $._exp_un_raw)),
    _exp_unassign: ($) => prec.right(seq($.unassign, $._exp_un_raw)),

    actor: ($) => seq("actor", field("value", $._actor_value)),
    _actor_value: ($) => choice($.lit, $.tuple),

    not: ($) => prec.right(seq("not", $._exp_un_raw)),

    debug_show: ($) => prec.right(seq("debug_show", field("value", $._exp_un_raw))),

    to_candid: ($) =>
      seq(
        "to_candid",
        field("value", $.tuple),
        // "(",
        // _list($._exp, ","),
        // ")"
      ),

    from_candid: ($) => prec.right(seq("from_candid", field("value", $._exp_un_raw))),

    _exp_bin_raw: ($) => choice($.exp_bin, $._exp_un_raw, $.exp_annotated),

    exp_bin: ($) =>
      choice(
        ...[
          ["implies", PREC.IMPLIES],
          ["|>", PREC.PIPE],
          ["or", PREC.OR],
          ["and", PREC.AND],
          ["+", PREC.ADDSUB],
          ["-", PREC.ADDSUB],
          ["*", PREC.MULDIV],
          ["/", PREC.MULDIV],
          ["%", PREC.MOD],
          ["+%", PREC.ADDSUBWRAP],
          ["-%", PREC.ADDSUBWRAP],
          ["*%", PREC.MULWRAP],
          ["|", PREC.BOR],
          ["&", PREC.BAND],
          ["^", PREC.BXOR],
          ["#", PREC.CONCAT],
          ["**", PREC.EXP],
          ["**%", PREC.EXPWRAP],
        ].map(([s, p]) => prec.left(p, _binop($._exp_bin_raw, s))),
        // technically there's no precedence, but we need to resolve the conflict for the parser
        // choose left
        ...[
          ["==", PREC.RELOP],
          ["!=", PREC.RELOP],
          [alias($._spaced_gt, " > "), PREC.RELOP],
          [alias($._spaced_lt, " < "), PREC.RELOP],
          ["<=", PREC.RELOP],
          [">=", PREC.RELOP],
          ["<<", PREC.BSHIFT],
          [alias($._spaced_shr, " >>"), PREC.BSHIFT],
          ["<<>", PREC.BROTATE],
          ["<>>", PREC.BROTATE],
        ].map(([s, p]) => prec(p, prec.left(p, _binop($._exp_bin_raw, s)))),
      ),
    exp_annotated: ($) => prec.left(PREC.COLON, seq($._exp_bin_raw, $._annotation)),

    exp_nondec: ($) =>
      choice(
        $.exp_bin,
        $.exp_un,
        $.exp_nullary,
        $.exp_post,
        $.exp_variant,
        $.exp_annotated,
        $.array_exp,
        $.call_exp,
        $.system_actor_class_exp,
        $.assign_exp,
        $.binassign_exp,
        $.return_exp,
        $.async_exp,
        $.asyncstar_exp,
        $.await_exp,
        $.awaitstar_exp,
        $.assert_exp,
        $.special_assert_exp,
        $.label_exp,
        $.break_exp,
        $.continue_exp,
        $.debug_exp,
        $.if_exp,
        $.if_else_exp,
        $.try_exp,
        $.throw_exp,
        $.switch_exp,
        $.while_exp,
        $.loop_exp,
        $.loop_while_exp,
        $.for_exp,
        $.ignore_exp,
        $.do_exp,
      ),

    assign_exp: ($) =>
      prec.right(PREC.ASSIGN, seq(field("binding", $._exp_bin_raw), field("op", ":="), field("value", $._exp))),

    binassign_exp: ($) =>
      choice(
        ...[
          "+=",
          "-=",
          "*=",
          "/=",
          "%=",
          "**-",
          "+%=",
          "-%=",
          "*%=",
          "**%=",
          "&=",
          "|=",
          "^=",
          "<<=",
          ">>=",
          "<<>=",
          "<>>=",
          "@=",
        ].map((s) =>
          prec.right(PREC.ASSIGN, seq(field("binding", $._exp_bin_raw), field("op", s), field("value", $._exp))),
        ),
      ),

    return_exp: ($) => prec.right(seq("return", optional(field("value", $._exp)))),

    async_exp: ($) => seq("async", $.exp_nest),

    asyncstar_exp: ($) => seq("async*", field("value", $.exp_nest)),

    await_exp: ($) => seq("await", field("value", $.exp_nest)),

    awaitstar_exp: ($) => seq("await*", $.exp_nest),

    assert_exp: ($) => seq("assert", $.exp_nest),

    special_assert_exp: ($) => seq("assert", token.immediate(":"), $._special_assert_sub, $.exp_nest),
    _special_assert_sub: ($) =>
      choice(
        ...[["invariant"], [/[0-9]+/, "async"], ["func"], ["return"], ["system"], ["loop", "invariant"]].map(
          (sl, i) => {
            sl[0] = token.immediate(sl[0]);
            if (sl.length > 1) {
              for (let i = sl.length - 1; i > 0; i--) {
                sl[i] = token.immediate(sl[i]);
                sl.splice(i, 0, token.immediate(":"));
              }
              return seq(...sl);
            }
            return sl[0];
          },
        ),
      ),

    label_exp: ($) => seq("label", field("label_name", $.name), optional($._annotation), $.exp_nest),
    // _label_typ_exp: $ => _annotation("label", $._typ),

    break_exp: ($) => seq("break", field("label_name", $.name), optional(field("value", $.exp_nullary))),

    continue_exp: ($) => seq("continue", field("label_name", $.name)),

    debug_exp: ($) => seq("debug", $.exp_nest),

    if_exp: ($) => prec(PREC.IF, seq("if", field("antecedent", $.exp_nullary), field("consequence", $.exp_nest))),

    if_else_exp: ($) =>
      prec(
        PREC.ELSE,
        seq(
          "if",
          field("antecedent", $.exp_nullary),
          field("consequence", $.exp_nest),
          "else",
          field("fallback", $.exp_nest),
        ),
      ),

    try_exp: ($) => prec(PREC.TRY, seq("try", field("body", $.exp_nest), $._try_alt)),
    _try_alt: ($) => prec.left(choice(seq($.catch_exp, optional($.finally_exp)), $.finally_exp)),

    catch_exp: ($) => prec(PREC.CATCH, seq("catch", field("guard", $._pat_nullary), field("handler", $.exp_nest))),

    finally_exp: ($) => prec(PREC.FINALLY, seq("finally", field("cleanup", $.exp_nest))),

    throw_exp: ($) => seq("throw", $.exp_nest),

    switch_exp: ($) => seq("switch", field("control", $.exp_nullary), "{", _list1($.case_exp, ";"), "}"),

    case_exp: ($) => seq("case", field("candidate", $._pat_nullary), field("consequence", $.exp_nest)),

    while_exp: ($) => seq("while", field("control", $.exp_nullary), field("body", $.exp_nest)),

    loop_exp: ($) => prec(PREC.LOOP, seq("loop", field("body", $.exp_nest))),

    loop_while_exp: ($) =>
      prec(PREC.WHILE, seq("loop", field("loop_body", $.exp_nest), "while", field("while_body", $.exp_nest))),

    for_exp: ($) =>
      seq("for", "(", field("binding_pattern", $.pat), "in", field("iterator", $._exp), ")", field("body", $.exp_nest)),

    ignore_exp: ($) => seq("ignore", field("body", $.exp_nest)),

    do_exp: ($) => seq("do", optional("?"), field("body", $.block)),

    exp_nonvar: ($) => choice($.exp_nondec, $.dec_nonvar),

    _exp: ($) => choice($.exp_nonvar, $.dec_var),

    exp_nest: ($) => choice($.block, $._exp),

    ...make_patternlike_rule_class("variable", ""),
    ...make_patternlike_rule_class("parameter", "fnparam"),

    pat_variant: ($) => seq("#", field("variant_name", $.name)),

    shared_pat_opt: ($) =>
      choice(seq("shared", optional($.query), optional($._pat_plain)), seq($.query, optional($._pat_plain))),

    typ_obj: ($) => seq("{", _list($._typ_field, ";"), "}"),

    _typ_field: ($) => choice($.typ_component, $.typ_member, $.typ_method),
    typ_component: ($) =>
      seq(
        "type",
        field("component_name", $.name),
        optional($._typ_bind_args),
        "=",
        field("component_type_value", $._typ),
      ),
    // _typ_component_fish: $ => _fish($.typ_bind),
    typ_member: ($) => seq(optional("var"), field("field_name", $.name), ":", field("field_type_value", $._typ)),
    typ_method: ($) =>
      seq(
        field("method_name", $.name),
        optional($.typ_params_opt),
        field("param_type_value", $.typ_nullary),
        ":",
        field("return_type_value", $._typ),
      ),
    // _typ_func_short_fish: $ => _fish($.typ_bind),

    typ_variant: ($) => choice($.typ_empty_variant, seq("{", _list1($.typ_tag, ";"), "}")),

    typ_empty_variant: () => seq("{", "#", "}"),

    typ_nullary: ($) => choice($.typ_tuple, $.typ_id, $.typ_array, $.typ_obj, $.typ_variant, $.typ_prim),

    typ_tuple: ($) => seq("(", _list($.typ_item, ","), ")"),

    typ_item: ($) => seq(optional(seq(field("tuple_component_name", $.name), ":")), field("type_value", $._typ)),

    typ_id: ($) => seq(choice(field("type_name", $.name), $._typ_id_member), optional($.typ_args)),
    _typ_id_member: ($) =>
      seq(
        field("module_name", $.name),
        repeat(alias($._typ_id_member_dot_name, $.member)),
        ".",
        field("type_name", $.name),
      ),
    _typ_id_member_dot_name: ($) => seq(".", $.name),

    typ_array: ($) => seq("[", optional("var"), field("type_value", $._typ), "]"),

    typ_prim: ($) => seq("prim", $.text),

    _typ_un_raw: ($) => choice($.typ_nullary, $.typ_un),

    typ_un: ($) => seq(field("unary", "?"), $._typ_un_raw),

    _typ_pre_raw: ($) => choice($._typ_un_raw, $.typ_pre),

    typ_pre: ($) => choice($.typ_async, $.typ_asyncstar, seq($.obj_sort, $.typ_obj)),
    typ_async: ($) => seq("async", field("type_value", $._typ_pre_raw)),
    typ_asyncstar: ($) => seq("async*", field("type_value", $._typ_pre_raw)),

    _typ_nobin_raw: ($) => choice($._typ_pre_raw, $.typ_func),

    //typ_nobin: $ => $.typ_func,

    typ_func: ($) =>
      seq(
        optional($.func_sort_opt),
        optional($.typ_params_opt),
        field("param_type_value", $._typ_un_raw),
        "->",
        field("return_type_value", $._typ_nobin_raw),
      ),
    // _typ_func_fish: $ => prec.dynamic(1, _fish($.typ_bind)),

    query: () => choice("query", seq("composite", "query")),

    func_sort_opt: ($) => choice(seq("shared", optional($.query)), $.query),

    _typ: ($) => choice($._typ_nobin_raw, $.typ_and, $.typ_or),

    typ_and: ($) => prec.left(PREC.TYPEAND, seq(_binop($._typ, "and"))),
    typ_or: ($) => prec.left(PREC.TYPEOR, seq(_binop($._typ, "or"))),

    typ_args: ($) => _fish($._typ),

    _inst: ($) => choice($.typ_args, alias($._system_prefixed_typ_args, $.typ_args)),
    _system_prefixed_typ_args: ($) => seq("<", "system", _rlist($._typ, ","), ">"),

    typ_params_opt: ($) => choice($._typ_bind_args, $._system_prefixed_typ_bind_args),
    _typ_bind_args: ($) => _fish($.typ_bind),
    _system_prefixed_typ_bind_args: ($) => seq("<", "system", _rlist($.typ_bind, ","), ">"),

    typ_tag: ($) => seq("#", field("variant_name", $.name), optional(seq(":", $._typ))),

    typ_bind: ($) =>
      seq(field("type_parameter_name", $.name), optional(seq("<:", field("constraint_type_value", $._typ)))),

    lit: ($) => choice("null", $.bool, $.nat, $.float, $.char, $.text),

    bool: () => choice("true", "false"),

    vis: ($) => choice("private", "public", "system"),
    stab: () => choice("flexible", "stable"),

    obj_sort: () => choice("object", "actor", "module"),

    placeholder: () => "_",

    name: () => NAME,

    // linecomment: () => token(seq("//", /.*/u)),

    nat: () => choice(DECIMAL_NUM, HEX_NUM),
    float: () => choice(...FLOAT),

    // utf8_enc: () => choice(...UTF8_ENC),
    utf8_enc: () => /./u,

    // _utf8: $ => choice(ASCII, $.utf8_enc),
    // _utf8_no_nl: $ => choice(ASCII_NO_NL, $.utf8_enc),

    escape: () => seq("\\", token.immediate(ESCAPE)),
    byte: () => seq("\\", token.immediate(HEX_DIGIT), token.immediate(HEX_DIGIT)),
    utf8code: () => seq("\\u{", token.immediate(HEX_NUM), token.immediate("}")),

    character: ($) => choice(/[^"\\\x00-\x1f\x7f-\xff]/, $.utf8_enc, $.escape, $.byte, $.utf8code),

    char: ($) => seq("'", $.character, token.immediate("'")),

    text: ($) => seq($._text_literal_begin, repeat(choice($.escape, $._text_literal_content)), $._text_literal_end),

    block_comment: ($) => seq($._block_comment_begin, $._block_comment_content, $._block_comment_end),

    line_comment: ($) => seq($._line_comment_begin, $._line_comment_content),

    doc_comment: ($) => seq($._doc_comment_begin, $._doc_comment_content),
  },
});
