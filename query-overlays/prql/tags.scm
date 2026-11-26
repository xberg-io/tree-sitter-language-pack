; Top-level `let` declarations: `let name params -> body` defines a function and
; `let name = (pipeline)` a named relation.

(program
  (function_definition
    name: (identifier) @name) @definition.function)

(program
  (variable
    name: (identifier) @name) @definition.variable)
