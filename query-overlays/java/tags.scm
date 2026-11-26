; Java, written from tree-sitter-java-orchard (github.com/grammar-orchard/tree-sitter-java-orchard),
; starting from its vendored queries/tags.scm. Locals, parameters and type parameters are not
; definitions.

; `package com.example.geo;` tags its qualified name.
(package_declaration
  [
    (scoped_identifier)
    (identifier)
  ] @name) @definition.module

(class_declaration
  name: (identifier) @name) @definition.class

; A record is a class; its components are its fields.
(record_declaration
  name: (identifier) @name) @definition.class

(record_declaration
  parameters: (formal_parameters
    (formal_parameter
      name: (identifier) @name))) @definition.field

(interface_declaration
  name: (identifier) @name) @definition.interface

; `@interface Audited { String owner(); }`: an annotation type and its elements.
(annotation_type_declaration
  name: (identifier) @name) @definition.interface

(annotation_type_element_declaration
  name: (identifier) @name) @definition.method

(enum_declaration
  name: (identifier) @name) @definition.enum

(enum_constant
  name: (identifier) @name) @definition.constant

; Overloads name one method: the first declaration in a body tags, later ones are claimed.
([
  (class_body
    (method_declaration
      name: (identifier) @_first)
    (method_declaration
      name: (identifier) @ignore))
  (interface_body
    (method_declaration
      name: (identifier) @_first)
    (method_declaration
      name: (identifier) @ignore))
  (enum_body_declarations
    (method_declaration
      name: (identifier) @_first)
    (method_declaration
      name: (identifier) @ignore))
  (class_body
    (constructor_declaration
      name: (identifier) @_first)
    (constructor_declaration
      name: (identifier) @ignore))
  (enum_body_declarations
    (constructor_declaration
      name: (identifier) @_first)
    (constructor_declaration
      name: (identifier) @ignore))
]
  (#eq? @_first @ignore))

(method_declaration
  name: (identifier) @name) @definition.method

(constructor_declaration
  name: (identifier) @name) @definition.constructor

; `Point { ... }` in a record.
(compact_constructor_declaration
  name: (identifier) @name) @definition.constructor

; Interface fields and `static final` fields are constants; other fields are fields.
(constant_declaration
  declarator: (variable_declarator
    name: (identifier) @name)) @definition.constant

(field_declaration
  (modifiers
    (modifier) @_first
    (modifier) @_second)
  declarator: (variable_declarator
    name: (identifier) @name)
  (#any-of? @_first "static" "final")
  (#any-of? @_second "static" "final")) @definition.constant

(field_declaration
  declarator: (variable_declarator
    name: (identifier) @name)) @definition.field

; `class Circle extends Base<Circle>`, `implements Shape` and `interface Shape extends
; Comparable<Shape>`.
(superclass
  [
    (type_identifier) @name
    (generic_type
      (type_identifier) @name)
  ]) @reference.implementation

(super_interfaces
  (type_list
    [
      (type_identifier) @name
      (generic_type
        (type_identifier) @name)
    ])) @reference.implementation

(extends_interfaces
  (type_list
    [
      (type_identifier) @name
      (generic_type
        (type_identifier) @name)
    ])) @reference.implementation

; `new ArrayList<>()` and `new IllegalArgumentException("x")`.
(object_creation_expression
  type: [
    (type_identifier) @name
    (generic_type
      (type_identifier) @name)
  ]) @reference.class

(method_invocation
  name: (identifier) @name) @reference.call
