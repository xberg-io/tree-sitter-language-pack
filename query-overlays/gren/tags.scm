; Gren. Based on the tags.scm of github.com/gren-lang/tree-sitter-gren, an Elm fork, which used
; roles outside the tag vocabulary (reference.function, definition.union), tagged type
; annotations and exposed values as references, tagged `let` bindings as definitions and missed
; `type alias`. Kept in step with query-overlays/elm/tags.scm.

; `module Main exposing (..)`; a dotted module name is one name.
(module_declaration
  name: (upper_case_qid) @name) @definition.module

(import_clause
  moduleName: (upper_case_qid) @name) @reference.module

; Bindings of a `let` block are local.
(let_in_expr
  (value_declaration
    (function_declaration_left (lower_case_identifier) @ignore)))

(value_declaration
  (function_declaration_left (lower_case_identifier) @name)) @definition.function

(port_annotation
  name: (lower_case_identifier) @name) @definition.function

; A custom type is an enum of its variants.
(type_declaration
  name: (upper_case_identifier) @name) @definition.enum

(union_variant
  name: (upper_case_identifier) @name) @definition.constant

(type_alias_declaration
  name: (upper_case_identifier) @name) @definition.type

; Calls name the function by its last part: `String.fromInt` calls `fromInt`.
(function_call_expr
  target: (value_expr
    name: (value_qid (lower_case_identifier) @name .))) @reference.call

; Type uses, without the core types.
(type_ref
  (upper_case_qid (upper_case_identifier) @name .)
  (#not-match? @name "^(Int|Float|String|Bool|Char|Never|Array|Maybe|Result)$")) @reference.type

; Types imported by name; a module's own exposing list is an export, not a use.
(import_clause
  exposing: (exposing_list
    (exposed_type (upper_case_identifier) @name) @reference.type))

; Constructor uses, without the core constructors.
(value_expr
  name: (upper_case_qid (upper_case_identifier) @name .)
  (#not-match? @name "^(True|False|Just|Nothing|Ok|Err)$")) @reference.constant
