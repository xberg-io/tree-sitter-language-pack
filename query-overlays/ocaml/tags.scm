; OCaml (implementation files). Based on the tags.scm of github.com/tree-sitter/tree-sitter-ocaml,
; which tagged every infix, prefix and `let*` operator as a call, used roles outside the tag
; vocabulary (enum_variant, operator, field references) and tagged local `let ... in` functions.
; Kept in step with query-overlays/ocaml_interface/tags.scm, which shares the grammar.

; Modules
(module_definition
  (module_binding (module_name) @name)) @definition.module

(module_path (module_name) @name) @reference.module

(extended_module_path (module_name) @name) @reference.module

(module_type_definition (module_type_name) @name) @definition.interface

(module_type_path (module_type_name) @name) @reference.interface

; Classes, methods and instance variables
(class_definition
  (class_binding (class_name) @name)) @definition.class

(class_path (class_name) @name) @reference.class

; `class type name = object ... end` is a class signature.
(class_type_definition
  (class_type_binding (class_type_name) @name)) @definition.interface

(class_type_path (class_type_name) @name) @reference.interface

(method_definition (method_name) @name) @definition.method

(method_specification (method_name) @name) @definition.method

(method_invocation (method_name) @name) @reference.call

(instance_variable_definition (instance_variable_name) @name) @definition.field

; Types: a record is a class, a variant an enum, anything else (abstract types, aliases) a type.
(type_binding
  name: (type_constructor) @name
  body: (record_declaration)) @definition.class

(type_binding
  name: (type_constructor) @name
  body: (variant_declaration)) @definition.enum

(type_binding
  name: [
    (type_constructor) @name
    (type_constructor_path (type_constructor) @name)
  ]) @definition.type

; Type uses, without the predefined types.
(type_constructor_path
  (type_constructor) @name
  (#not-match? @name "^(int|float|string|bool|char|unit|bytes|exn|list|array|option|result|ref|int32|int64|nativeint|lazy_t|format)$")) @reference.type

(exception_definition
  (constructor_declaration (constructor_name) @name)) @definition.class

(constructor_declaration (constructor_name) @name) @definition.constant

(constructor_path (constructor_name) @name) @reference.constant

(field_declaration (field_name) @name) @definition.field

; Values. Bindings local to an expression (`let ... in`) are not definitions.
(let_expression
  (value_definition
    (let_binding pattern: (value_name) @ignore)))

(value_definition
  [
    (let_binding pattern: (value_name) @name (parameter))
    (let_binding
      pattern: (value_name) @name
      body: [(fun_expression) (function_expression)])
  ]) @definition.function

; `let ( +: ) a b = ...` defines the operator `+:`.
(value_definition
  [
    (let_binding pattern: (parenthesized_operator (_) @name) (parameter))
    (let_binding
      pattern: (parenthesized_operator (_) @name)
      body: [(fun_expression) (function_expression)])
  ]) @definition.function

(value_definition
  (let_binding pattern: (value_name) @name)) @definition.variable

(external (value_name) @name) @definition.function

; `val name : a -> b` in a signature.
(value_specification
  (value_name) @name
  type: (function_type)) @definition.function

(value_specification
  (value_name) @name) @definition.variable

; Calls: `f x`, `x |> f` and `f @@ x`. Operators are not tagged.
(application_expression
  function: (value_path (value_name) @name)) @reference.call

(infix_expression
  left: (value_path (value_name) @name)
  operator: (concat_operator) @_operator
  (#eq? @_operator "@@")) @reference.call

(infix_expression
  operator: (rel_operator) @_operator
  right: (value_path (value_name) @name)
  (#eq? @_operator "|>")) @reference.call
