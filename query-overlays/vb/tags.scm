; Namespaces
(namespace_block
  name: (namespace_name) @name) @definition.module

; Types
(class_block
  name: (identifier) @name) @definition.class

(structure_block
  name: (identifier) @name) @definition.class

(module_block
  name: (identifier) @name) @definition.module

(interface_block
  name: (identifier) @name) @definition.interface

(enum_block
  name: (identifier) @name) @definition.enum

(delegate_declaration
  name: (identifier) @name) @definition.type

; Members. `Sub New` constructors are not tagged: their name is the `New` keyword, which the
; grammar does not expose as a node.
(method_declaration
  name: (identifier) @name) @definition.method

(property_declaration
  name: (identifier) @name) @definition.field

(event_declaration
  name: (identifier) @name) @definition.method

(const_declaration
  name: (identifier) @name) @definition.constant

(field_declaration
  (variable_declarator
    name: (identifier) @name)) @definition.field

; `Inherits` and `Implements` are not tagged: the grammar accepts them only on the `Class` line
; itself, while VB requires each as its own statement in the class body, so real files never
; produce inherits_clause or implements_clause.

; Calls and instantiation
(invocation
  target: [
    (identifier) @name
    (member_access
      member: (identifier) @name)
  ]) @reference.call

(new_expression
  type: (type
    (namespace_name
      (identifier) @name .))) @reference.class

; Imports
(imports_statement
  namespace: (namespace_name) @name) @reference.module
