; Packages
(package_declaration
  name: [
    (identifier)
    (scoped_data_type)
  ] @name) @definition.module

; Classes and interfaces
(class_declaration
  name: (identifier) @name) @definition.class

(interface_declaration
  name: (identifier) @name) @definition.interface

; Methods: functions declared in a class body or an interface
(class_declaration
  body: (statement_block
    (function_declaration
      name: (identifier) @name) @definition.method))

(method_declaration
  name: (identifier) @name) @definition.method

; Functions
(function_declaration
  name: (identifier) @name) @definition.function

; Fields and constants of a class; variables and constants of a package or script, not locals
(class_declaration
  body: (statement_block
    (constant_declaration
      name: (identifier) @name) @definition.constant))

(class_declaration
  body: (statement_block
    (variable_declaration
      name: (identifier) @name) @definition.field))

(package_declaration
  body: (statement_block
    [
      (constant_declaration
        name: (identifier) @name) @definition.constant
      (variable_declaration
        name: (identifier) @name) @definition.variable
    ]))

(program
  [
    (constant_declaration
      name: (identifier) @name) @definition.constant
    (variable_declaration
      name: (identifier) @name) @definition.variable
  ])

(namespace_declaration
  name: (identifier) @name) @definition.module

; Inheritance
(class_declaration
  superclass: (identifier) @name) @reference.class

(class_declaration
  interfaces: (identifier) @name) @reference.implementation

(interface_declaration
  supertype: (identifier) @name) @reference.interface

; Calls
(call_expression
  function: (identifier) @name) @reference.call

(call_expression
  function: (member_expression
    property: (identifier) @name)) @reference.call

(new_expression
  (identifier) @name) @reference.class
