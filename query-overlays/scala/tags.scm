; Scala 2 and 3, written from tree-sitter-scala (github.com/tree-sitter/tree-sitter-scala), starting
; from its vendored queries/tags.scm. Locals, parameters and type parameters are not definitions.

; `package com.example.geo` tags its qualified name.
(package_clause
  name: (package_identifier) @name) @definition.module

(trait_definition
  name: (identifier) @name) @definition.interface

(enum_definition
  name: (identifier) @name) @definition.enum

; `case Red extends Color("#f00")` and `case North, South`
[
  (simple_enum_case
    name: (identifier) @name)
  (full_enum_case
    name: (identifier) @name)
] @definition.constant

(class_definition
  name: (identifier) @name) @definition.class

; A singleton or companion `object` is a module (scalac's `ModuleDef`): tagging it as a class would
; define a companion's name twice, once for `class Circle` and once for `object Circle`.
(object_definition
  name: (identifier) @name) @definition.module

; `type Points = Seq[Point]` and `opaque type Meters = Double`
(type_definition
  name: (type_identifier) @name) @definition.type

; `given pointOrdering: Ordering[Point]`; an anonymous given has no name to tag.
(given_definition
  name: (identifier) @name) @definition.variable

; Constructor parameters that are members: `val`/`var` ones, and every parameter of a case class.
(class_parameter
  [
    "val"
    "var"
  ]
  name: (identifier) @name) @definition.field

(class_definition
  "case"
  class_parameters: (class_parameters
    (class_parameter
      name: (identifier) @name) @definition.field))

; Overloads name one method: the first `def` in a scope tags, later ones are claimed.
([
  (compilation_unit
    (function_definition
      name: (identifier) @_first)
    (function_definition
      name: (identifier) @ignore))
  (template_body
    (function_definition
      name: (identifier) @_first)
    (function_definition
      name: (identifier) @ignore))
]
  (#eq? @_first @ignore))

; Members of classes, traits, objects and givens are methods, abstract ones included; top-level,
; extension and local `def`s are functions.
([
  (template_body
    [
      (function_definition
        name: (identifier) @name)
      (function_declaration
        name: (identifier) @name)
    ] @definition.method)
  (with_template_body
    [
      (function_definition
        name: (identifier) @name)
      (function_declaration
        name: (identifier) @name)
    ] @definition.method)
])

(function_definition
  name: (identifier) @name) @definition.function

; Top-level `val`/`var` are variables, member ones fields; those in blocks are locals.
(compilation_unit
  [
    (val_definition
      pattern: (identifier) @name)
    (var_definition
      pattern: (identifier) @name)
  ] @definition.variable)

(template_body
  [
    (val_definition
      pattern: (identifier) @name)
    (var_definition
      pattern: (identifier) @name)
    (val_declaration
      name: (identifier) @name)
    (var_declaration
      name: (identifier) @name)
  ] @definition.field)

; Supertypes: `extends Base("circle") with Serializable` and `extends Ordered[Shape]`.
(extends_clause
  [
    (type_identifier) @name
    (generic_type
      type: (type_identifier) @name)
  ]) @reference.implementation

; `new Circle(1.0)`
(instance_expression
  [
    (type_identifier) @name
    (generic_type
      type: (type_identifier) @name)
  ]) @reference.class

; Calls: `sqrt(...)` and `Circle.unit()`.
(call_expression
  function: [
    (identifier) @name
    (field_expression
      field: (identifier) @name)
  ]) @reference.call
