; Packages: the innermost package name
(package_statement
  name: (package_name) @name .) @definition.module

; Types
(class_declaration
  name: (identifier) @name) @definition.class

(interface_declaration
  name: (identifier) @name) @definition.interface

(enum_declaration
  name: (identifier) @name) @definition.enum

(enum_abstract_declaration
  name: (identifier) @name) @definition.enum

(typedef_declaration
  name: (identifier) @name) @definition.type

; Methods of classes and interfaces, including the `new` constructor; module-level functions
(class_declaration
  body: (block
    (function_declaration
      name: [
        (identifier)
        "new"
      ] @name) @definition.method))

(interface_declaration
  body: (block
    (function_declaration
      name: (identifier) @name) @definition.method))

(module
  (function_declaration
    name: (identifier) @name) @definition.function)

; Class fields and module-level variables, not locals
(class_declaration
  body: (block
    (variable_declaration
      name: (identifier) @name) @definition.field))

(module
  (variable_declaration
    name: (identifier) @name) @definition.variable)

; Inheritance
(class_declaration
  super_class_name: (type_name) @name) @reference.class

(class_declaration
  interface_name: (type_name) @name) @reference.implementation

(interface_declaration
  interface_name: (type_name) @name) @reference.implementation

; Calls and instantiation
(call_expression
  object: (identifier) @name) @reference.call

(call_expression
  object: (member_expression
    member: (identifier) @name)) @reference.call

(call_expression
  constructor: (type_name) @name) @reference.class
