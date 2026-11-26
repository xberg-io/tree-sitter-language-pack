; Functions: `function f()`, `local function f()` and `function T.f()`
(function_statement
  name: [
    (identifier) @name
    (table_function_name
      entry: (identifier) @name)
  ]) @definition.function

; Methods: `function T:m()`
(function_statement
  name: (table_function_name
    method: (identifier) @name)) @definition.method

; Records, interfaces, enums and type aliases, at any scope
(record_declaration
  name: (identifier) @name) @definition.class

(interface_declaration
  name: (identifier) @name) @definition.interface

(enum_declaration
  name: (identifier) @name) @definition.enum

(type_declaration
  name: (identifier) @name) @definition.type

(typedef
  name: (identifier) @name) @definition.type

; Record and interface fields
[
  (record_body
    (field
      key: (identifier) @name) @definition.field)
  (interface_body
    (field
      key: (identifier) @name) @definition.field)
]

; Macro expressions
(macroexp_declaration
  name: (identifier) @name) @definition.macro

; Module-level variables; `<const>` and `<total>` ones are constants
(program
  (var_declaration
    declarators: (var_declarators
      (var
        name: (identifier) @name
        attribute: (attribute) @_attribute
        (#any-of? @_attribute "const" "total")))) @definition.constant)

(program
  (var_declaration
    declarators: (var_declarators
      (var
        name: (identifier) @name))) @definition.variable)

; Calls: `f()`, `t.f()` and `obj:m()`
(function_call
  called_object: [
    (identifier) @name
    (index
      key: (identifier) @name)
    (method_index
      key: (identifier) @name)
  ]) @reference.call
