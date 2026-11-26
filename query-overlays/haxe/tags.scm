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

; `public function new(...)` is the constructor; other functions of a class or interface are
; methods, and functions at module level are functions.
(class_declaration
  body: (block
    (function_declaration
      name: "new" @name) @definition.constructor))

(class_declaration
  body: (block
    (function_declaration
      name: (identifier) @name) @definition.method))

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

; `interface Scalable extends Shape`
(interface_declaration
  interface_name: (type_name) @name) @reference.interface

; Calls and instantiation
; `super(radius)` calls the parent constructor, which is not a named call target.
((call_expression
  object: (identifier) @name) @reference.call
  (#not-eq? @name "super"))

(call_expression
  object: (member_expression
    member: (identifier) @name)) @reference.call

(call_expression
  constructor: (type_name) @name) @reference.class
