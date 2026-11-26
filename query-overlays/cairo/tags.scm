(const_item
  name: (identifier) @name) @definition.constant

(struct_item
  name: (type_identifier) @name) @definition.class

(enum_item
  name: (type_identifier) @name) @definition.enum

(enum_variant
  name: (identifier) @name) @definition.constant

(trait_item
  name: (type_identifier) @name) @definition.interface

(mod_item
  name: (identifier) @name) @definition.module

; Functions declared or defined in a trait or impl are methods; the rest are functions.
(trait_item
  body: (declaration_list
    (function_signature . (identifier) @name) @definition.method))

(impl_item
  (block
    (function_definition . (identifier) @name) @definition.method))

(function_definition
  . (identifier) @name) @definition.function

; `impl PointShape of Shape<Point>`: Cairo impls are named and implement one trait.
(impl_item
  . (type_identifier) @name) @definition.class

(impl_item
  [
    (type_identifier) @name
    (generic_type type: (type_identifier) @name)
  ] .
  (block)) @reference.implementation

(call_expression
  function: [
    (identifier) @name
    (scoped_identifier name: (identifier) @name)
    (field_expression field: (field_identifier) @name)
  ]) @reference.call
