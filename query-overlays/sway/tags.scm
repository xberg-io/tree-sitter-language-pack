(abi_item
  name: (type_identifier) @name) @definition.interface

(trait_item
  name: (type_identifier) @name) @definition.interface

(struct_item
  name: (type_identifier) @name) @definition.class

(enum_item
  name: (type_identifier) @name) @definition.enum

(enum_variant
  name: (identifier) @name) @definition.constant

(const_item
  name: (identifier) @name) @definition.constant

; Contract storage slots.
(storage_content
  (field_declaration
    name: (field_identifier) @name)) @definition.field

; Functions of an abi, trait or impl are methods; free functions are functions.
(function_signature_item
  name: (identifier) @name) @definition.method

(impl_item
  body: (declaration_list
    (function_item name: (identifier) @name) @definition.method))

(function_item
  name: (identifier) @name) @definition.function

(impl_item
  trait: (type_identifier) @name) @reference.implementation

(call_expression
  function: [
    (identifier) @name
    (scoped_identifier name: (identifier) @name)
    (field_expression field: (field_identifier) @name)
  ]) @reference.call
