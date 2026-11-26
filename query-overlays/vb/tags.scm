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

; Members
(method_declaration
  name: (identifier) @name) @definition.method

(property_declaration
  name: (identifier) @name) @definition.property

(event_declaration
  name: (identifier) @name) @definition.event

(const_declaration
  name: (identifier) @name) @definition.constant

(field_declaration
  (variable_declarator
    name: (identifier) @name)) @definition.field

; Inheritance: `Inherits Base` and `Implements IFoo, IBar`, by the last name of the type
(inherits_clause
  (type
    (namespace_name
      (identifier) @name .))) @reference.class

(implements_clause
  (type
    (namespace_name
      (identifier) @name .))) @reference.implementation

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
