; The grammar is a flat token stream: `:label` is a `function_definition` wherever it appears, so
; a label that directly follows a `goto` keyword is a jump target, not a definition.

; `goto :label` (also `goto :eof`)
((keyword) @_goto
  .
  (function_definition "function_name" @name) @reference.label
  (#match? @_goto "^@?[Gg][Oo][Tt][Oo]$"))

; `:label` at the start of a line
(function_definition "function_name" @name) @definition.label

; `set NAME="value"` and `set /A NAME=1`
(variable_declaration (identifier) @name) @definition.variable

; `%NAME%`
(variable_reference "variable_name" @name) @reference.variable
