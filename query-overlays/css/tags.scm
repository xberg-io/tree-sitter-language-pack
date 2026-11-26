; The named declarations of a stylesheet: custom properties and `@keyframes` animations.
; Selectors match elements rather than declare anything, so rule sets are not tagged.

; `--brand: #336699;`
((declaration (property_name) @name) @definition.variable
  (#match? @name "^--"))

; `@keyframes fade-in { ... }`
(keyframes_statement (keyframes_name) @name) @definition.keyframes
