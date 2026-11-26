; Assembly (`.s`, `.asm`). Global labels name the code and data they mark; `const NAME value`
; names a constant. Dot-prefixed labels such as `.loop:` are local to the enclosing global label
; and are not tagged.

; `_start:` and `label name`
(label
  (ident) @name) @definition.label

(label
  name: (word) @name) @definition.label

; `const STACK_SIZE 4096`
(const
  name: (word) @name) @definition.constant

; `call clear`: a call names the label it transfers to.
(instruction
  kind: (word) @_kind
  (ident) @name
  (#match? @_kind "^[cC][aA][lL][lL]$")) @reference.call
