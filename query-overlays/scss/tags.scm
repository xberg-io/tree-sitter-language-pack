; Top-level declarations of a stylesheet: variables (`$name: value`), mixins, functions and
; placeholder selectors (`%name { ... }`).

; `$name: value;`
(stylesheet
  (declaration (property_name) @name) @definition.variable
  (#match? @name "^\\$"))

; `@mixin name($args) { ... }`
(stylesheet
  (mixin_statement name: (identifier) @name) @definition.macro)

; `@function name($args) { ... }`
(stylesheet
  (function_statement name: (identifier) @name) @definition.function)

; `%name { ... }`, one tag per placeholder of the selector list.
(stylesheet
  (rule_set
    (selectors (placeholder) @name)) @definition.class)
