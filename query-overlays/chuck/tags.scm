; ChucK (`.ck`). Classes, functions, methods and the variables a script declares at top level.
; Variables declared inside a function or class body are not tagged.

; `class Voice extends Object { ... }`: the name follows `class`, the base class `extends`.
(class_definition
  "class"
  .
  [(class_identifier) (variable_identifier)] @name) @definition.class

; `fun void play(float freq)` inside a class body is a method.
(class_definition
  (block
    (function_definition
      name: (_) @name) @definition.method))

; `fun float double(float x)` at top level
(source_file
  (function_definition
    name: (_) @name) @definition.function)

; Top-level `SinOsc s => dac;`, `440.0 => float base;` and `Voice v;`
(source_file
  (statement
    [(variable_declaration
       (variable_identifier) @name)
     (chuck_operation
       (variable_declaration
         (variable_identifier) @name))]) @definition.variable)

; Calls: `double(base)` and `v.play(x)`
(function_call
  .
  [(variable_identifier) @name
   (member_identifier
     (variable_identifier) @name .)]) @reference.call
