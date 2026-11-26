; Module-level units are the declarations: modules, operator and function definitions,
; `CONSTANT` and `VARIABLE` declarations, and named theorems. `LET` definitions and quantifier
; bindings are local.

(module
  name: (identifier) @name) @definition.module

(module
  (operator_definition
    name: (_) @name) @definition.function)

(module
  (function_definition
    name: (identifier) @name) @definition.function)

(module
  (constant_declaration
    (identifier) @name) @definition.constant)

(module
  (variable_declaration
    (identifier) @name) @definition.variable)

(module
  (theorem
    name: (identifier) @name) @definition.function)

; `M == INSTANCE Other WITH ...`
(module
  (module_definition
    name: (identifier) @name) @definition.module)
