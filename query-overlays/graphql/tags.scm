(object_type_definition
  (name) @name) @definition.class

(input_object_type_definition
  (name) @name) @definition.class

(interface_type_definition
  (name) @name) @definition.interface

(union_type_definition
  (name) @name) @definition.type

(scalar_type_definition
  (name) @name) @definition.type

(enum_type_definition
  (name) @name) @definition.enum

(enum_value_definition
  (enum_value (name) @name)) @definition.constant

(field_definition
  (name) @name) @definition.field

(directive_definition
  (name) @name) @definition.macro

; Named queries, mutations and subscriptions, and fragments.
(operation_definition
  (name) @name) @definition.function

(fragment_definition
  (fragment_name (name) @name)) @definition.function

(named_type
  (name) @name) @reference.type

(fragment_spread
  (fragment_name (name) @name)) @reference.call

(directive
  (name) @name) @reference.macro
