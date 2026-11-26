; Motorola 68000 assembly. Global labels, symbols assigned with `equ`, `set` or `=`, structure
; offsets defined with `rs`, and macros. Local labels (`.loop:` or `1$:`) belong to the
; enclosing global label and are not tagged.

; `start:` and `buffer: ds.b 256`
(label
  name: (symbol) @name
  (#not-match? @name "^\\.|\\$$")) @definition.label

; `BUFSIZE equ 256`
(symbol_definition
  name: (symbol) @name) @definition.constant

; `pos_x rs.w 1`
(offset_definition
  name: (symbol) @name) @definition.constant

; `PUSHALL macro ... endm`
(macro_definition
  name: (symbol) @name) @definition.macro

; `PUSHALL` and `PRINT msg`
(macro_call
  name: (symbol) @name) @reference.call

; `bsr clear` and `jsr (init)`
(instruction
  mnemonic: (instruction_mnemonic) @_mnemonic
  operands: (operand_list
              .
              (symbol) @name)
  (#match? @_mnemonic "^(?i)(bsr|jsr)$")) @reference.call
