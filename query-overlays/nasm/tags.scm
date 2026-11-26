; NASM (`.asm`, `.nasm`). Global labels, `equ` constants, structures and single-line and
; multi-line macros. Local labels (`.loop:`) belong to the preceding global label, and the fields
; of a `struc` are local to it; neither is tagged.

; `LEN equ 13`
(source_line
  (label
    name: (word) @name)
  (instruction
    (pseudo_instruction_equ_command))) @definition.constant

; `_start:` and `msg: db "Hello", 10`
(source_line
  (label
    name: (word) @name
    (#not-match? @name "^\\."))) @definition.label

; `struc point ... endstruc`
(struc_declaration
  name: (word) @name) @definition.struct

; `%define SYS_EXIT 60` and `%macro exit 1 ... %endmacro`
[(preproc_def name: (word) @name)
 (preproc_multiline_macro name: (word) @name)] @definition.macro

; `call print`
(actual_instruction
  instruction: (word) @_instruction
  operands: (operands
              .
              (operand
                (word) @name))
  (#match? @_instruction "^(?i)call$")) @reference.call
