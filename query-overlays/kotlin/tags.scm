; Kotlin, written from tree-sitter-kotlin (github.com/fwcd/tree-sitter-kotlin), starting from its
; vendored queries/tags.scm. Locals, parameters and type parameters are not definitions.
; ~keep A secondary constructor (`constructor(diameter: Int) : this(...)`) has no name node, so it
; ~keep does not tag; the primary constructor is the class itself.

(package_header
  (identifier) @name) @definition.module

; `interface Shape`, `enum class Color` and every other class form (data, sealed, value, open).
(class_declaration
  "interface"
  (type_identifier) @name) @definition.interface

(class_declaration
  (type_identifier) @name
  (enum_class_body)) @definition.enum

(class_declaration
  (type_identifier) @name) @definition.class

; `object Registry` and `companion object Factory`; an unnamed companion has no name to tag.
(object_declaration
  (type_identifier) @name) @definition.class

(companion_object
  (type_identifier) @name) @definition.class

(enum_entry
  (simple_identifier) @name) @definition.constant

(type_alias
  (type_identifier) @name) @definition.type

; `class Point(val x: Double)`: a `val` or `var` constructor parameter is a property.
(class_parameter
  (binding_pattern_kind)
  (simple_identifier) @name) @definition.field

; Properties: `const val` is a constant, a top-level `val`/`var` a variable, a member one a field.
; Properties declared in function bodies are locals.
(property_declaration
  (modifiers
    (property_modifier) @_modifier)
  (variable_declaration
    (simple_identifier) @name)
  (#eq? @_modifier "const")) @definition.constant

(source_file
  (property_declaration
    (variable_declaration
      (simple_identifier) @name)) @definition.variable)

(class_body
  (property_declaration
    (variable_declaration
      (simple_identifier) @name)) @definition.field)

; Overloads name one function: the first declaration in a scope tags, later ones are claimed.
([
  (source_file
    (function_declaration
      (simple_identifier) @_first)
    (function_declaration
      (simple_identifier) @ignore))
  (class_body
    (function_declaration
      (simple_identifier) @_first)
    (function_declaration
      (simple_identifier) @ignore))
  (enum_class_body
    (function_declaration
      (simple_identifier) @_first)
    (function_declaration
      (simple_identifier) @ignore))
]
  (#eq? @_first @ignore))

; Member functions are methods; top-level and extension functions (`fun Shape.describe()`) are
; functions.
([
  (class_body
    (function_declaration
      (simple_identifier) @name) @definition.method)
  (enum_class_body
    (function_declaration
      (simple_identifier) @name) @definition.method)
])

(function_declaration
  (simple_identifier) @name) @definition.function

; Supertypes: `: Shape, Comparable<Circle>` and `: Result()`; a qualified supertype tags its last
; segment.
([
  (class_declaration
    (delegation_specifier
      [
        (user_type
          (type_identifier) @name .)
        (user_type
          (type_identifier) @name
          .
          (type_arguments) .)
        (constructor_invocation
          (user_type
            (type_identifier) @name .))
        (constructor_invocation
          (user_type
            (type_identifier) @name
            .
            (type_arguments) .))
      ]))
  (object_declaration
    (delegation_specifier
      [
        (user_type
          (type_identifier) @name .)
        (user_type
          (type_identifier) @name
          .
          (type_arguments) .)
        (constructor_invocation
          (user_type
            (type_identifier) @name .))
        (constructor_invocation
          (user_type
            (type_identifier) @name
            .
            (type_arguments) .))
      ]))
] @reference.implementation)

; Calls: `println(...)`, `Circle(UNIT)` and `shapes.add(shape)`.
(call_expression
  [
    (simple_identifier) @name
    (navigation_expression
      (navigation_suffix
        (simple_identifier) @name))
  ]) @reference.call
