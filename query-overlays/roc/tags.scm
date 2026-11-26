; Roc. Based on the tags.scm of github.com/faldor20/tree-sitter-roc, which tagged only functions
; and calls. Type aliases, nominal types, tag unions and top-level values are added. Top-level
; values are constants: Roc evaluates them at compile time. Type annotations (`add : I64 -> I64`)
; restate a declaration and are not tagged.

(import_expr
  (import_ident (module) @name)) @reference.module

; `Color : [Red, Green]` is an enum of its tags; other aliases are types.
(alias_type_def
  (apply_type (concrete_type) @name)
  body: (tags_type)) @definition.enum

(alias_type_def
  (apply_type (concrete_type) @name)) @definition.type

(tags_type (tag_type) @name) @definition.constant

; `Counter := U64`
(nominal_type_def
  .
  (apply_type (concrete_type) @name)) @definition.type

(file
  (value_declaration
    (decl_left (identifier_pattern (identifier) @name))
    body: (expr_body (anon_fun_expr))) @definition.function)

(file
  (value_declaration
    (decl_left (identifier_pattern (identifier) @name))) @definition.constant)

; Calls: `add(1, 2)`, `Stdout.line!(text)` and `ops.run(value)`.
(function_call_pnc_expr
  caller: (variable_expr (identifier) @name)) @reference.call

(function_call_pnc_expr
  caller: (field_access_expr (identifier) @name .)) @reference.call

; Tag uses, without the builtin `Ok` and `Err`.
((tag_expr (tag (identifier) @name))
  (#not-match? @name "^(Ok|Err)$")) @reference.constant

; Type uses, without the builtin types.
((concrete_type) @name
  (#not-match? @name "^(Str|Bool|List|Dict|Set|Result|Box|Num|Int|Frac|Dec|[IU](8|16|32|64|128)|F(32|64))$")) @reference.type
