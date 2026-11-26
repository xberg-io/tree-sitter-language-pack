; C#, written from tree-sitter-c-sharp (github.com/tree-sitter/tree-sitter-c-sharp), starting from
; its vendored queries/tags.scm. Locals and parameters are not definitions.

; `namespace App.Core { ... }` and `namespace App.Tools;` tag their full qualified name.
[
  (namespace_declaration
    name: [
      (identifier)
      (qualified_name)
    ] @name)
  (file_scoped_namespace_declaration
    name: [
      (identifier)
      (qualified_name)
    ] @name)
] @definition.module

(class_declaration
  name: (identifier) @name) @definition.class

(struct_declaration
  name: (identifier) @name) @definition.class

; `record Size(double Width, double Height);`: a record is a class, and its positional
; parameters are its properties.
(record_declaration
  name: (identifier) @name) @definition.class

(record_declaration
  (parameter_list
    (parameter
      name: (identifier) @name))) @definition.field

(interface_declaration
  name: (identifier) @name) @definition.interface

(enum_declaration
  name: (identifier) @name) @definition.enum

(enum_member_declaration
  name: (identifier) @name) @definition.constant

; `delegate void Handler(...)` declares a function type.
(delegate_declaration
  name: (identifier) @name) @definition.type

; Overloads name one method or constructor: the first declaration in a body tags, later ones are
; claimed.
([
  (declaration_list
    (method_declaration
      name: (identifier) @_first)
    (method_declaration
      name: (identifier) @ignore))
  (declaration_list
    (constructor_declaration
      name: (identifier) @_first)
    (constructor_declaration
      name: (identifier) @ignore))
]
  (#eq? @_first @ignore))

(method_declaration
  name: (identifier) @name) @definition.method

(constructor_declaration
  name: (identifier) @name) @definition.constructor

(property_declaration
  name: (identifier) @name) @definition.field

; `event Handler Changed;` and `event Handler Changed { add ... }`
(event_field_declaration
  (variable_declaration
    (variable_declarator
      name: (identifier) @name))) @definition.method

(event_declaration
  name: (identifier) @name) @definition.method

; `const` fields are constants; other fields are fields.
(field_declaration
  (modifier) @_modifier
  (variable_declaration
    (variable_declarator
      name: (identifier) @name))
  (#eq? @_modifier "const")) @definition.constant

(field_declaration
  (variable_declaration
    (variable_declarator
      name: (identifier) @name))) @definition.field

; `using System.Collections.Generic;`
(using_directive
  [
    (identifier)
    (qualified_name)
  ] @name) @reference.module

; Base classes and implemented interfaces: `class Circle : Shape<int>`, `: IShape`.
[
  (class_declaration
    (base_list
      [
        (identifier) @name
        (generic_name
          (identifier) @name)
        (qualified_name
          name: [
            (identifier) @name
            (generic_name
              (identifier) @name)
          ])
      ]))
  (struct_declaration
    (base_list
      [
        (identifier) @name
        (generic_name
          (identifier) @name)
        (qualified_name
          name: [
            (identifier) @name
            (generic_name
              (identifier) @name)
          ])
      ]))
  (record_declaration
    (base_list
      [
        (identifier) @name
        (generic_name
          (identifier) @name)
        (qualified_name
          name: [
            (identifier) @name
            (generic_name
              (identifier) @name)
          ])
      ]))
  (interface_declaration
    (base_list
      [
        (identifier) @name
        (generic_name
          (identifier) @name)
        (qualified_name
          name: [
            (identifier) @name
            (generic_name
              (identifier) @name)
          ])
      ]))
] @reference.implementation

; `new Circle(2.0)` and `new List<IShape>()`
(object_creation_expression
  type: [
    (identifier) @name
    (generic_name
      (identifier) @name)
  ]) @reference.class

; Calls: `Notify()`, `Console.WriteLine(...)` and `Changed?.Invoke(...)`.
(invocation_expression
  function: [
    (identifier) @name
    (generic_name
      (identifier) @name)
    (member_access_expression
      name: [
        (identifier) @name
        (generic_name
          (identifier) @name)
      ])
    (conditional_access_expression
      (member_binding_expression
        name: (identifier) @name))
  ]) @reference.call
