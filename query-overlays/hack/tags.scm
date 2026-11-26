; Namespaces
(namespace_declaration
  name: (qualified_identifier) @name) @definition.module

; Classes, interfaces, traits, enums and type aliases
(class_declaration
  name: (identifier) @name) @definition.class

(interface_declaration
  name: (identifier) @name) @definition.interface

(trait_declaration
  name: (identifier) @name) @definition.interface

(enum_declaration
  name: (identifier) @name) @definition.enum

(enum_class_declaration
  name: (identifier) @name) @definition.enum

(abstract_enum_class_declaration
  name: (identifier) @name) @definition.enum

(alias_declaration
  (identifier) @name) @definition.type

; Functions and methods
(function_declaration
  name: (identifier) @name) @definition.function

(method_declaration
  name: (identifier) @name) @definition.method

; Constants and properties
(const_declarator
  name: (identifier) @name) @definition.constant

(property_declarator
  name: (variable) @name) @definition.field

; Inheritance
(extends_clause
  (type_specifier
    (qualified_identifier
      (identifier) @name .))) @reference.class

(implements_clause
  (type_specifier
    (qualified_identifier
      (identifier) @name .))) @reference.implementation

; Calls: `f()`, `$obj->m()`, `C::m()`
(call_expression
  function: (qualified_identifier
    (identifier) @name .)) @reference.call

(call_expression
  function: (selection_expression
    (qualified_identifier
      (identifier) @name .) .)) @reference.call

(call_expression
  function: (scoped_identifier
    (identifier) @name .)) @reference.call

; Instantiation
(new_expression
  (qualified_identifier
    (identifier) @name .)) @reference.class
