; D, written from tree-sitter-d (github.com/gdamore/tree-sitter-d), starting from its vendored
; queries/tags.scm. Locals, parameters and template parameters are not definitions.
; ~keep Constructors are declared with the keyword `this`, which is their name as written; a
; ~keep destructor (`~this`) and a postblit (`this(this)`) do not tag, so an aggregate's `this`
; ~keep names one constructor, and overloads tag once.

; `module geo.shapes;` tags its qualified name.
(module_declaration
  (module_fqn) @name) @definition.module

(struct_declaration
  (struct)
  .
  (identifier) @name) @definition.class

(union_declaration
  (union)
  .
  (identifier) @name) @definition.class

(class_declaration
  (class)
  .
  (identifier) @name) @definition.class

(interface_declaration
  (interface)
  .
  (identifier) @name) @definition.interface

(enum_declaration
  (enum)
  .
  (identifier) @name) @definition.enum

; Members of named and anonymous enums.
(enum_member
  .
  (identifier) @name) @definition.constant

; `enum maxShapes = 64;` is a manifest constant.
(manifest_declarator
  .
  (identifier) @name) @definition.constant

; `template Pair(T)` and `mixin template Counted()`. An eponymous member (`alias Pair = ...`
; inside `template Pair`) is the template itself.
(template_declaration
  (identifier) @_template
  (alias_declaration
    (alias_initializer
      .
      (identifier) @ignore))
  (#eq? @_template @ignore))

(template_declaration
  (template)
  .
  (identifier) @name) @definition.type

(alias_declaration
  (alias_initializer
    .
    (identifier) @name)) @definition.type

; Overloads name one function: the first declaration in a scope tags, later ones are claimed.
([
  (aggregate_body
    (constructor
      (this) @_first)
    (constructor
      (this) @ignore))
  (aggregate_body
    (function_declaration
      (identifier) @_first)
    (function_declaration
      (identifier) @ignore)
    (#eq? @_first @ignore))
  (module_def
    (function_declaration
      (identifier) @_first)
    (function_declaration
      (identifier) @ignore)
    (#eq? @_first @ignore))
  (source_file
    (function_declaration
      (identifier) @_first)
    (function_declaration
      (identifier) @ignore)
    (#eq? @_first @ignore))
])

(aggregate_body
  (constructor
    (this) @name) @definition.constructor)

; Members of structs, unions, classes and interfaces are methods; others are functions.
(aggregate_body
  (function_declaration
    (identifier) @name) @definition.method)

(function_declaration
  (identifier) @name) @definition.function

(aggregate_body
  (variable_declaration
    (declarator
      .
      (identifier) @name)) @definition.field)

; Module-scope variables; `immutable` and `const` ones are constants.
([
  (module_def
    (variable_declaration
      (type
        (type_ctor
          [
            (immutable)
            (const)
          ]))
      (declarator
        .
        (identifier) @name)) @definition.constant)
  (source_file
    (variable_declaration
      (type
        (type_ctor
          [
            (immutable)
            (const)
          ]))
      (declarator
        .
        (identifier) @name)) @definition.constant)
])

([
  (module_def
    (variable_declaration
      (declarator
        .
        (identifier) @name)) @definition.variable)
  (source_file
    (variable_declaration
      (declarator
        .
        (identifier) @name)) @definition.variable)
])

; `class Circle : Shape`
(base_class
  (identifier) @name) @reference.implementation

; `new Circle(2.0)`
(new_expression
  (type
    (identifier) @name .)) @reference.class

; Calls: `add(...)`, `c.draw()` and `to!string(...)`.
(call_expression
  [
    (identifier) @name
    (type
      (identifier) @name .)
    (type
      (template_instance
        (identifier) @name))
  ]) @reference.call
