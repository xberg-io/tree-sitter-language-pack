; Lua tags: the vendored tags.scm of MunifTanjim/tree-sitter-lua, plus module-level locals,
; constants, module table fields and required modules.

(function_declaration
  name: [
    (identifier) @name
    (dot_index_expression
      field: (identifier) @name)
  ]) @definition.function

(function_declaration
  name: (method_index_expression
    method: (identifier) @name)) @definition.method

(assignment_statement
  (variable_list .
    name: [
      (identifier) @name
      (dot_index_expression
        field: (identifier) @name)
    ])
  (expression_list .
    value: (function_definition))) @definition.function

(table_constructor
  (field
    name: (identifier) @name
    value: (function_definition))) @definition.function

; `local LIMIT <const> = 10`: a const attribute makes a constant whatever the case.
(chunk
  (variable_declaration
    (assignment_statement
      (variable_list
        name: (identifier) @name
        .
        attribute: (attribute
          (identifier) @_attribute)))) @definition.constant
  (#eq? @_attribute "const"))
; Module-level locals that are not functions (the patterns above claim those): UPPER_CASE
; names of two or more characters are constants (a one-letter `M` is the module table), the
; rest variables. Locals inside functions are not tagged.
(chunk
  (variable_declaration
    [
      (assignment_statement
        (variable_list
          name: (identifier) @name))
      (variable_list
        name: (identifier) @name)
    ]) @definition.constant
  (#match? @name "^[A-Z][A-Z0-9_]+$"))

(chunk
  (variable_declaration
    [
      (assignment_statement
        (variable_list
          name: (identifier) @name))
      (variable_list
        name: (identifier) @name)
    ]) @definition.variable)

; Module-level globals and module table fields: `count = 0`, `M.VERSION = "1.0"`.
(chunk
  (assignment_statement
    (variable_list
      name: (identifier) @name)) @definition.variable)

(chunk
  (assignment_statement
    (variable_list
      name: (dot_index_expression
        field: (identifier) @name))) @definition.field)

; Loaded modules: `require("json")`, `require "socket.http"`. Path-like strings name no module.
(function_call
  name: (identifier) @_require
  arguments: (arguments
    (string
      content: (string_content) @name))
  (#eq? @_require "require")
  (#match? @name "^[A-Za-z_][A-Za-z0-9_.]*$")) @reference.module

(function_call
  name: [
    (identifier) @name
    (dot_index_expression
      field: (identifier) @name)
    (method_index_expression
      method: (identifier) @name)
  ]) @reference.call
