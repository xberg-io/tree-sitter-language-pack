; `object "Token" {}` is not tagged: its name is a string literal whose node text keeps the
; quotes, and the grammar exposes no node for the bare name.
(functionDefinition
  name: (identifier) @name) @definition.function

(functionCall
  name: (identifier) @name) @reference.call
