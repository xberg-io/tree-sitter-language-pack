; Functions: `function f()`, `function M.f()` and `local function f()`
(function_declaration
  name: [
    (identifier) @name
    (dot_index_expression
      field: (identifier) @name)
  ]) @definition.function

; Methods: `function M:f()`
(function_declaration
  name: (method_index_expression
    method: (identifier) @name)) @definition.method

; Functions assigned to a name or a table field
(assignment_statement
  (variable_list
    .
    name: [
      (identifier) @name
      (dot_index_expression
        field: (identifier) @name)
    ])
  (expression_list
    .
    value: (function_definition))) @definition.function

(table_constructor
  (field
    name: (identifier) @name
    value: (function_definition))) @definition.function

; Type aliases, exported or not
(type_definition
  name: (identifier) @name) @definition.type

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

; `require` is not tagged: a Luau require names a relative path (`require("./Signal")`) or an
; instance path (`require(script.Parent.Signal)`), not a module identifier.

; Calls
(function_call
  name: [
    (identifier) @name
    (dot_index_expression
      field: (identifier) @name)
    (method_index_expression
      method: (identifier) @name)
  ]) @reference.call
