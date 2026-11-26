; WebAssembly script format (`.wast`), the spec-test superset of the text format. Named modules
; and the named fields of each module: functions, globals, types, memories and tables. Names keep
; their `$` sigil. Parameters and locals are local to their function, and assertions name no
; declarations; neither is tagged. An import names a function defined in another module and is
; not tagged.

; `(module $math ...)`
(module
  identifier: (identifier) @name) @definition.module

; `(func $add (param $a i32) ...)`
(module_field_func
  identifier: (identifier) @name) @definition.function

; `(global $counter (mut i32) ...)`, `(memory $mem 1)` and `(table $fns 2 funcref)`
[(module_field_global identifier: (identifier) @name)
 (module_field_memory identifier: (identifier) @name)
 (module_field_table identifier: (identifier) @name)] @definition.variable

; `(type $binop (func ...))`
(module_field_type
  identifier: (identifier) @name) @definition.type

; `(call $add ...)` and `return_call $add`
(instr_plain
  (op_index) @_op
  (index
    (identifier) @name)
  (#match? @_op "^(return_)?call$")) @reference.call
