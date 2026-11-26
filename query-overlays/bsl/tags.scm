; Procedures and functions
(procedure_definition
  name: (identifier) @name) @definition.function

(function_definition
  name: (identifier) @name) @definition.function

; Module variables (`Var Name Export;` before the methods); locals use var_statement
(var_definition
  variable: (variable_spec
    name: (identifier) @name)) @definition.variable

; Calls, including `Object.Method()`
(method_call
  name: (identifier) @name) @reference.call
