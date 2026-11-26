; Functions
(function_definition
  name: (identifier) @name) @definition.function

; Preprocessor macros: `@define name %( ... %)`
(preprocessor_macro_definition
  name: (identifier) @name) @definition.macro

; Probe aliases: `probe name = probe.point { ... }` (and `+=`) name a new probe point whose
; handler runs like a function body.
(probe_point_alias_prologue
  alias: (probe_point) @name) @definition.function

(probe_point_alias_epilogue
  alias: (probe_point) @name) @definition.function

; Global variables and arrays
(variable_declaration
  [
    (identifier) @name
    (init_declarator
      name: (identifier) @name)
    (array_declarator
      name: (identifier) @name)
  ]) @definition.variable

; Calls
(call_expression
  function: (identifier) @name) @reference.call
