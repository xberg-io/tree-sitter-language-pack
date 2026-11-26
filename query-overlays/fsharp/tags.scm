; F# (`.fs`). Based on queries/tags.scm of https://github.com/ionide/tree-sitter-fsharp, with
; types tagged by what they define, properties as fields, and type extensions (which add members
; to a type defined elsewhere) not tagged as a second definition of that type. Bindings inside a
; function body are local.

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

[
  (file
    (declaration_expression
      (function_or_value_defn
        (function_declaration_left
          . (_) @name))))
  (named_module
    (declaration_expression
      (function_or_value_defn
        (function_declaration_left
          . (_) @name))))
  (module_defn
    (declaration_expression
      (function_or_value_defn
        (function_declaration_left
          . (_) @name))))
  (namespace
    (declaration_expression
      (function_or_value_defn
        (function_declaration_left
          . (_) @name))))
] @definition.function

[
  (file
    (declaration_expression
      (function_or_value_defn
        (value_declaration_left
          . (identifier_pattern
              (long_identifier_or_op
                (_) @name))))))
  (named_module
    (declaration_expression
      (function_or_value_defn
        (value_declaration_left
          . (identifier_pattern
              (long_identifier_or_op
                (_) @name))))))
  (module_defn
    (declaration_expression
      (function_or_value_defn
        (value_declaration_left
          . (identifier_pattern
              (long_identifier_or_op
                (_) @name))))))
  (namespace
    (declaration_expression
      (function_or_value_defn
        (value_declaration_left
          . (identifier_pattern
              (long_identifier_or_op
                (_) @name))))))
] @definition.constant

; `member this.Radius = radius`: a member without arguments is a property.
(member_defn
  (method_or_prop_defn
    name: (property_or_ident
      method: (identifier) @name)
    !args)) @definition.field

(member_defn
  (method_or_prop_defn
    name: (property_or_ident
      method: (identifier) @name))) @definition.method

; Static members (`static member Unit() = ...`): the name holds only the identifier.
(member_defn
  (method_or_prop_defn
    name: (property_or_ident
      . (identifier) @name .))) @definition.method

; `abstract member Area: unit -> float`
(member_defn
  (member_signature
    (identifier) @name)) @definition.method
