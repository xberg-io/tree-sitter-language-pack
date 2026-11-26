; Uxntal (`.tal`), the assembly language of the Uxn virtual machine. Labels (`@name`) mark
; routines, data and device ports; `%NAME { ... }` defines a macro. Sublabels (`&loop`) are local to
; the preceding label and are not tagged.

; `%EMIT { .Console/write DEO }`
(macro
  .
  (identifier) @name) @definition.macro

; `@print-str` and `@System`
(label
  (identifier) @name) @definition.label

; A bare word invokes a macro or calls a label: `print-str` and `EMIT`.
[(subroutine (identifier) @name)
 (memory_execution (identifier) @name)
 (brackets (identifier) @name)
 (macro (identifier) @name)] @reference.call
