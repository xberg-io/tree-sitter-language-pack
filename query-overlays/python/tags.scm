; Python tags, written from the grammar-orchard/tree-sitter-python-orchard grammar, which this
; pack pins. That fork parses a top-level `X = 1` as `module > assignment` (no
; expression_statement wrapper), so the vendored constant pattern never matched.

; Methods: a function directly in a class body, plain or decorated. Listed before the
; function pattern so the earlier pattern claims the name.
(class_definition
  body: (block
    (function_definition
      name: (identifier) @name) @definition.method))

(class_definition
  body: (block
    (decorated_definition
      definition: (function_definition
        name: (identifier) @name) @definition.method)))

; Classes, plain, decorated or nested.
(class_definition
  name: (identifier) @name) @definition.class

; Functions, sync or async (the `async` keyword is anonymous), plain or decorated.
(function_definition
  name: (identifier) @name) @definition.function

; Class-level attributes: `count: int = 0`, `count = 0`.
(class_definition
  body: (block
    (assignment
      left: (identifier) @name) @definition.field))

; Type aliases: `Vector: TypeAlias = list[float]` and `type Point = tuple[int, int]` (generic
; `type Pair[T] = ...` wraps the name in a generic_type).
(module
  (assignment
    left: (identifier) @name
    type: (type
      (identifier) @_annotation)) @definition.type
  (#eq? @_annotation "TypeAlias"))

(type_alias_statement
  left: (type
    [
      (identifier) @name
      (generic_type
        type: (identifier) @name)
    ])) @definition.type

; Module-level assignments: UPPER_CASE names are constants, the rest variables. Tuple targets
; (`a, b = 1, 2`) tag each name.
(module
  (assignment
    left: (identifier) @name) @definition.constant
  (#match? @name "^_?[A-Z][A-Z0-9_]*$"))

(module
  (assignment
    left: (identifier) @name) @definition.variable)

(module
  (assignment
    left: [
      (pattern_list
        (identifier) @name)
      (tuple_pattern
        (identifier) @name)
    ]) @definition.variable)

; Base classes: `class Greeter(Base)`, `class Greeter(models.Model)`.
(class_definition
  superclasses: (argument_list
    [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
    ]) @reference.class)

; Imported modules: `import os.path`, `from typing import TypeAlias`.
(import_statement
  name: [
    (dotted_name) @name
    (aliased_import
      name: (dotted_name) @name)
  ]) @reference.module

(import_from_statement
  module_name: (dotted_name) @name) @reference.module

; Calls: `format_name(x)`, `name.title()`.
(call
  function: [
    (identifier) @name
    (attribute
      attribute: (identifier) @name)
  ]) @reference.call
