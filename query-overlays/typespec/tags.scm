(namespace_statement
  name: (identifier_or_member_expression) @name) @definition.module

(model_statement
  name: (identifier) @name) @definition.class

(model_body
  (model_property
    name: (identifier) @name) @definition.field)

(enum_statement
  name: (identifier) @name) @definition.enum

(enum_member
  name: (identifier) @name) @definition.constant

(union_statement
  name: (identifier) @name) @definition.type

(scalar_statement
  name: (identifier) @name) @definition.type

(alias_statement
  name: (identifier) @name) @definition.type

(interface_statement
  name: (identifier) @name) @definition.interface

(interface_member
  name: (identifier) @name) @definition.method

(operation_statement
  name: (identifier) @name) @definition.function

; Declared types, by their last segment; built-in types (`int32`, `string`) are not declared.
(reference_expression
  (identifier_or_member_expression
    [
      (identifier (plain_identifier) @name)
      (member_expression member: (identifier (plain_identifier) @name))
    ])) @reference.type
