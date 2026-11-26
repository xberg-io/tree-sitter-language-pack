; Faust (`.dsp`). Top-level definitions, including `process`, and the definitions of an
; `environment { ... }` bound at top level. Definitions in a `with { ... }` block are local to
; the expression they qualify and are not tagged.

; `voice(f) = os.osc(f) * gain;`
(program
  (function_definition
    name: (identifier) @name) @definition.function)

; `gain = 0.5;` and `process = ...;`
(program
  (definition
    variable: (identifier) @name) @definition.variable)

; `filters = environment { lowpass(c) = ...; };`
(program
  (definition
    (environment
      (function_definition
        name: (identifier) @name) @definition.function)))

(program
  (definition
    (environment
      (definition
        variable: (identifier) @name) @definition.variable)))

; Calls: `voice(freq)` and `fi.lowpass(2, c)`
(function_call
  callee: [(identifier) @name
           (access definition: (identifier) @name)]) @reference.call
