; Macros
(macro_definition
  name: (identifier) @name) @definition.macro

; Declared maps: `let @name = hash(...);`
(map_declaration
  (map_variable) @name) @definition.variable

; Calls to builtins and macros
(call_expression
  function: (identifier) @name) @reference.call
