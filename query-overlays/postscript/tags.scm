; PostScript (`.ps`, `.eps`). `/name {...} def` (optionally `bind def`) defines a procedure and
; `/name value def` a variable; the name is the literal without its slash. Every executed name
; is a call: PostScript does not distinguish operators, procedures and variable lookups.

(_
  (literal
    name: (name) @name) @definition.function
  .
  (procedure)
  .
  (operator) @_def
  (#eq? @_def "def"))

(_
  (literal
    name: (name) @name) @definition.function
  .
  (procedure)
  .
  (operator) @_bind
  .
  (operator) @_def
  (#eq? @_bind "bind")
  (#eq? @_def "def"))

(_
  (literal
    name: (name) @name) @definition.variable
  .
  [
    (numeric)
    (string)
    (array)
    (dictionary)
    (literal)
  ]
  .
  (operator) @_def
  (#eq? @_def "def"))

(operator) @name @reference.call
