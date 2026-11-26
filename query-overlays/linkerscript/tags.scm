; GNU linker scripts (`.ld`, `.lds`). The symbols a script defines, the memory regions of its
; `MEMORY` command and the output sections of its `SECTIONS` command. Input section patterns such
; as `*(.text*)` select sections of the input files and are not tagged.

; `_stack_size = 0x400;` and `_etext = .;`; `. = ALIGN(4);` moves the location counter and
; defines nothing.
(assignment
  .
  (symbol) @name) @definition.variable

; `PROVIDE(_heap_start = _etext);`
(provide_command
  .
  (symbol) @name) @definition.variable

; `FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 256K`
(memory_command
  name: (symbol) @name) @definition.label

; `.text : { ... } > FLASH`
(output_section
  name: (symbol) @name) @definition.section

; `> FLASH` and `AT > FLASH` place a section in a memory region.
[(region (symbol) @name)
 (lma_region (symbol) @name)] @reference.label
