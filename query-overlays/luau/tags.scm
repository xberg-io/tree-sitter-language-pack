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

; Module-level locals
(chunk
  (variable_declaration
    (assignment_statement
      (variable_list
        name: (identifier) @name))) @definition.variable)

; Calls
(function_call
  name: [
    (identifier) @name
    (dot_index_expression
      field: (identifier) @name)
    (method_index_expression
      method: (identifier) @name)
  ]) @reference.call
