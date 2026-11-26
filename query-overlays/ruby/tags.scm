; Ruby tags, written from the tree-sitter/tree-sitter-ruby grammar.

; Classes: `class Shape`, `class Geometry::Circle`.
(class
  name: [
    (constant) @name
    (scope_resolution
      name: (constant) @name)
  ]) @definition.class

; Modules: `module Geometry`, `module Geometry::Solids`.
(module
  name: [
    (constant) @name
    (scope_resolution
      name: (constant) @name)
  ]) @definition.module

; Methods: `def area`, `def self.unit`, and `def build` inside `class << self`.
(method
  name: (identifier) @name) @definition.method

(singleton_method
  name: (identifier) @name) @definition.method

; Classes built at runtime: `Point = Struct.new(:x, :y)`, `Pair = Data.define(:a, :b)`,
; `Error = Class.new(StandardError)`. Listed before the constant pattern so it claims the name.
(assignment
  left: (constant) @name
  right: (call
    receiver: (constant) @_receiver
    method: (identifier) @_method)
  (#match? @_receiver "^(Struct|Class|Data)$")
  (#match? @_method "^(new|define)$")) @definition.class

; Constants: `VERSION = "1.0"`, `Geometry::MAX = 3`.
(assignment
  left: [
    (constant) @name
    (scope_resolution
      name: (constant) @name)
  ]) @definition.constant

; Global variables keep their `$` sigil, which every use spells.
(program
  (assignment
    left: (global_variable) @name) @definition.variable)

; Attribute macros declare fields. Only the string form (`attr_reader "id"`) has a clean name
; node: a symbol argument (`:id`) is a single simple_symbol token that includes the colon.
(call
  method: (identifier) @_macro
  arguments: (argument_list
    (string
      (string_content) @name))
  (#match? @_macro "^attr_(accessor|reader|writer)$")) @definition.field

; Superclasses: `class Circle < Shape`, `class Circle < Geometry::Shape`.
(superclass
  [
    (constant) @name
    (scope_resolution
      name: (constant) @name)
  ]) @reference.class

; Mixins: `include Comparable`, `extend Forwardable`, `prepend Logging`.
(call
  method: (identifier) @_mixin
  arguments: (argument_list
    [
      (constant) @name
      (scope_resolution
        name: (constant) @name)
    ])
  (#match? @_mixin "^(include|extend|prepend)$")) @reference.module

; Method calls.
(call
  method: (identifier) @name) @reference.call
