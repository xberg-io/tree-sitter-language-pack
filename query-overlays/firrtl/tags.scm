; FIRRTL (`.fir`). The modules and external modules of a circuit. The circuit itself is not
; tagged: its name is the name of its main module, which is. Ports, wires, registers and nodes are
; signals local to their module and are not tagged.

; `module Half :` and `extmodule BlackBox :`
(module
  .
  (identifier) @name) @definition.module

; `inst half of Half` instantiates the module `Half`.
(inst
  (identifier)
  .
  (identifier) @name) @reference.module
