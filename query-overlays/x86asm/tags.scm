; x86 assembly (Intel syntax, also objdump disassembly listings). Global labels and the symbol
; labels of a disassembly (`0000000000001139 <main>:`). Local labels (`.loop:`) belong to the
; preceding global label and are not tagged.

; `_start:` and `print:`
(label
  (label_name
    (identifier) @name
    (#not-match? @name "^\\."))) @definition.label

; `0000000000001139 <main>:` in `objdump -d` output
(objdump_section_label
  (identifier) @name) @definition.label

; `call print`
(ins
  (mnemonic) @_mnemonic
  (operand_args
    .
    (operand
      (operand_ident) @name))
  (#match? @_mnemonic "^(?i)call$")) @reference.call
