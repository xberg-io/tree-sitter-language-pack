; F# signature files (`.fsi`). Based on queries/tags.scm of the fsharp_signature grammar in
; https://github.com/ionide/tree-sitter-fsharp, with types tagged by what they define and
; signatures split by their type: a `val` or `member` whose type takes arguments is a function or
; method, one without arguments a value or property. A `new: ... -> T` constructor signature has
; no name node of its own and is not tagged.

; `module Geometry` heading a file, or `namespace Geometry`; a file has one or the other.
[(named_module
   name: (_) @name)
 (namespace
   name: (_) @name)] @definition.module

; `module Shapes = ...` nested in a namespace or module
(module_defn
  (identifier) @name) @definition.module

; `type IShape = interface ... end`. An interface written without `interface ... end` parses as
; a class, and `type Distance = float` as a one-case union: the grammar cannot tell them apart.
(type_definition
  (interface_type_defn
    (type_name
      type_name: (_) @name))) @definition.interface

(type_definition
  (enum_type_defn
    (type_name
      type_name: (_) @name))) @definition.enum

; `type Pair = float * float` and `type Callback = delegate of int -> unit`
(type_definition
  [(type_abbrev_defn
     (type_name
       type_name: (_) @name))
   (delegate_type_defn
     (type_name
       type_name: (_) @name))]) @definition.type

; Records, unions, classes and abstract type declarations
(type_definition
  [(record_type_defn
     (type_name
       type_name: (_) @name))
   (union_type_defn
     (type_name
       type_name: (_) @name))
   (anon_type_defn
     (type_name
       type_name: (_) @name))
   (type_declaration
     (type_name
       type_name: (_) @name))]) @definition.class

(exception_definition
  exception_name: (_) @name) @definition.class

; `val area: Shape -> float`
(value_definition
  (value_declaration_left
    . (identifier_pattern
        (long_identifier_or_op
          (_) @name)))
  (curried_spec
    (arguments_spec))) @definition.function

; `val origin: Point`
(value_definition
  (value_declaration_left
    . (identifier_pattern
        (long_identifier_or_op
          (_) @name)))) @definition.constant

; `member Scale: factor: float -> Ring` and `abstract member Area: unit -> float`
(member_defn
  (member_signature
    (identifier) @name
    (curried_spec
      (arguments_spec)))) @definition.method

; `member Radius: float`
(member_defn
  (member_signature
    (identifier) @name)) @definition.field
