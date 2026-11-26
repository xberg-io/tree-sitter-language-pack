; Top-level declarations of a Less stylesheet: variables, mixins and `@keyframes` animations,
; plus mixin calls. Variables declared inside a rule set are local to it and not tagged.

; `@primary: #336699;`
(stylesheet
  (variable_def name: (variable) @name) @definition.variable)

; `.bordered(@width) { ... }` and `#ns { .rounded() { ... } }`
(mixin_def
  name: [
    (class_selector name: (class_name) @name)
    (id_selector name: (id_name) @name)
  ]) @definition.macro

; `@keyframes pulse { ... }`
(keyframes_statement name: (keyframes_name) @name) @definition.keyframes

; `.bordered(4px);`
(mixin
  name: [
    (class_selector name: (class_name) @name)
    (id_selector name: (id_name) @name)
  ]) @reference.call
