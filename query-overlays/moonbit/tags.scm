; Based on queries/tags.scm of https://github.com/moonbitlang/tree-sitter-moonbit, reworked:
; methods (`fn Point::norm`) are named by the method identifier instead of the whole
; `Point::norm` path, enum constructors, struct fields, constants and top-level `let`s are
; tagged, type uses are type references, and `.method()` calls are not tagged because the
; grammar's `dot_lowercase_identifier` keeps the leading `.`.

; `fn Point::norm(self : Point) -> Int { ... }`
(function_definition
  (function_identifier
    (type_name)
    (lowercase_identifier) @name)) @definition.method

; `fn add(a : Int, b : Int) -> Int { ... }`
(function_definition
  (function_identifier
    (lowercase_identifier) @name)) @definition.function

; `fn new(x : Int) -> Point` declared in a struct body
(struct_constructor_declaration
  (lowercase_identifier) @name) @definition.constructor

(struct_definition
  (identifier) @name) @definition.class

(enum_definition
  (identifier) @name) @definition.enum

(enum_constructor
  (uppercase_identifier) @name) @definition.constant

(struct_field_declaration
  (lowercase_identifier) @name) @definition.field

; `type UserId = Int`, `suberror ParseError`, `extenum Event { ... }`
[
  (type_definition
    (identifier) @name)
  (error_type_definition
    (identifier) @name)
  (extenum_definition
    (identifier) @name)
] @definition.type

; `typealias Int as Score`
(type_alias_definition
  (type_alias_targets
    "as"
    (identifier) @name)) @definition.type

(trait_definition
  (identifier) @name) @definition.interface

(trait_method_declaration
  (function_identifier
    (lowercase_identifier) @name)) @definition.method

; `impl Shape for Point with area(self) { ... }`
(impl_definition
  (function_identifier
    (lowercase_identifier) @name)) @definition.method

(const_definition
  (uppercase_identifier) @name) @definition.constant

(structure
  (value_definition
    (lowercase_identifier) @name) @definition.variable)

(impl_definition
  (type_name
    (qualified_type_identifier) @name)) @reference.implementation

; `Self` stands for the implementing type, not a named one
((qualified_type_identifier) @name @reference.type
  (#not-eq? @name "Self"))

(apply_expression
  (qualified_identifier) @name) @reference.call

(method_expression
  (lowercase_identifier) @name) @reference.call
