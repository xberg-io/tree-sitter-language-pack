; Forth (`.fth`, `.4th`). Colon definitions define words; `constant`, `value`, `variable` and
; `create` define a named word for data. Words used inside a definition are calls.

; `: square ( n -- n*n ) dup * ;`
(word_definition
  (start_definition)
  .
  (word) @name) @definition.function

; `10 constant LIMIT`
((builtin) @_defining
  .
  (word) @name @definition.constant
  (#match? @_defining "^(?i)(2?constant|fconstant)$"))

; `variable counter`, `create buffer 64 allot` and `0 value threshold`, whose value `to`
; changes
((builtin) @_defining
  .
  (word) @name @definition.variable
  (#match? @_defining "^(?i)(2?variable|fvariable|create|f?value)$"))

; `dup square *`: `square` inside a definition
(word_definition
  (word) @name @reference.call)
