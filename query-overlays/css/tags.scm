; The named declarations of a stylesheet: custom properties and `@keyframes` animations, plus
; `var()` uses of custom properties. Selectors match elements rather than declare anything, so
; rule sets are not tagged.

; `--brand: #336699;`
((declaration (property_name) @name) @definition.variable
  (#match? @name "^--"))

; `@keyframes fade-in { ... }`: an animation name is a constant other rules refer to.
(keyframes_statement (keyframes_name) @name) @definition.constant

; `var(--brand)` and `var(--gap, 4px)`
((call_expression
  (function_name) @_function
  (arguments . (plain_value) @name)) @reference.variable
  (#eq? @_function "var")
  (#match? @name "^--"))
