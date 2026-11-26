; Swift tags, written from the alex-pinkus/tree-sitter-swift grammar. One class_declaration node
; covers class, struct, actor, enum and extension; the anonymous keyword child tells them apart.

; Types.
(class_declaration
  "enum"
  name: (type_identifier) @name) @definition.enum

(class_declaration
  [
    "class"
    "struct"
    "actor"
  ]
  name: (type_identifier) @name) @definition.class

; An extension declares no new type; it names the type it extends.
(class_declaration
  "extension"
  name: (user_type
    (type_identifier) @name)) @reference.class

(protocol_declaration
  name: (type_identifier) @name) @definition.interface

(typealias_declaration
  name: (type_identifier) @name) @definition.type

(associatedtype_declaration
  name: (type_identifier) @name) @definition.type

(enum_entry
  name: (simple_identifier) @name) @definition.constant

; Initializers.
(init_declaration
  "init" @name) @definition.constructor

; Methods in type and protocol bodies. A subscript has no name of its own, so it is tagged by
; its `subscript` keyword (not by a parameter name).
(class_body
  [
    (function_declaration
      name: (simple_identifier) @name)
    (subscript_declaration
      "subscript" @name)
    (deinit_declaration
      "deinit" @name)
  ] @definition.method)

(enum_class_body
  [
    (function_declaration
      name: (simple_identifier) @name)
    (subscript_declaration
      "subscript" @name)
  ] @definition.method)

(protocol_body
  [
    (protocol_function_declaration
      name: (simple_identifier) @name)
    (subscript_declaration
      "subscript" @name)
  ] @definition.method)

; Free functions.
(function_declaration
  name: (simple_identifier) @name) @definition.function

; Stored and computed properties of types and protocols.
(class_body
  (property_declaration
    name: (pattern
      bound_identifier: (simple_identifier) @name)) @definition.field)

(enum_class_body
  (property_declaration
    name: (pattern
      bound_identifier: (simple_identifier) @name)) @definition.field)

(protocol_property_declaration
  name: (pattern
    bound_identifier: (simple_identifier) @name)) @definition.field

; Top-level bindings: `let` is a constant, `var` a variable. Locals live under `statements`
; and are not tagged.
(source_file
  (property_declaration
    (value_binding_pattern
      "let")
    name: (pattern
      bound_identifier: (simple_identifier) @name)) @definition.constant)

(source_file
  (property_declaration
    (value_binding_pattern
      "var")
    name: (pattern
      bound_identifier: (simple_identifier) @name)) @definition.variable)

; Inheritance and protocol conformance: `: SuperType`. The grammar does not tell a superclass
; from a protocol, so both are implementation references.
(class_declaration
  (inheritance_specifier
    inherits_from: (user_type
      (type_identifier) @name)) @reference.implementation)

(protocol_declaration
  (inheritance_specifier
    inherits_from: (user_type
      (type_identifier) @name)) @reference.implementation)

; Imports.
(import_declaration
  (identifier) @name) @reference.module

; Calls: `area()`, `shape.area()`.
(call_expression
  .
  [
    (simple_identifier) @name
    (navigation_expression
      suffix: (navigation_suffix
        suffix: (simple_identifier) @name))
  ]) @reference.call
