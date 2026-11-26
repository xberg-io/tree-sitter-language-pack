; Gleam. Based on the tags.scm of github.com/gleam-lang/tree-sitter-gleam, which used roles
; outside the tag vocabulary (reference.function, definition.constructor) and relied on the
; `#is-not? local` property, which no query engine here evaluates, so every record access
; `user.name` tagged `user` as a module. The grammar cannot tell `module.function` from
; `record.field`, so a qualified name is tagged only where it is called.
; The pre-1.0 `external fn` form is not tagged: `@external` functions parse as `function`.

; `import gleam/io`
(import module: (module) @name) @reference.module

(constant name: (identifier) @name) @definition.constant

(function name: (identifier) @name) @definition.function

; A custom type with one constructor is a record (class) whose constructor shares its name; a
; type with several constructors is an enum of them.
(type_definition
  (type_name name: (type_identifier) @name)
  (data_constructors . (data_constructor) .)) @definition.class

(data_constructors
  .
  (data_constructor name: (constructor_name) @ignore)
  .)

(type_definition
  (type_name name: (type_identifier) @name)) @definition.enum

(data_constructor name: (constructor_name) @name) @definition.constant

(data_constructor_argument label: (label) @name) @definition.field

(type_alias
  (type_name name: (type_identifier) @name)) @definition.type

; `pub type Handle` declares a type with no constructors.
(external_type
  (type_name name: (type_identifier) @name)) @definition.type

; Type uses, without the prelude types.
((type_identifier) @name
  (#not-match? @name "^(Int|Float|String|Bool|Nil|List|Result|BitArray|UtfCodepoint)$")) @reference.type

; Constructor uses, without the prelude constructors.
((constructor_name) @name
  (#not-match? @name "^(True|False|Nil|Ok|Error)$")) @reference.constant

; Calls: `area(x)`, `io.println(x)` and the pipe targets `x |> f` and `x |> io.println`.
(function_call function: (identifier) @name) @reference.call

(function_call function: (field_access field: (label) @name)) @reference.call

(binary_expression
  operator: "|>"
  right: [
    (identifier) @name
    (field_access field: (label) @name)
    (function_call function: (identifier) @name)
    (function_call function: (field_access field: (label) @name))
  ]) @reference.call
