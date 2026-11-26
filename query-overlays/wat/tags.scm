; WebAssembly text format (`.wat`). The named fields of a module: functions, globals, types,
; memories and tables. Names keep their `$` sigil, as the text format writes and references them.
; Parameters and locals are local to their function and are not tagged. An import names a
; function defined in another module and is not tagged.

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

; `call $add` and `return_call $add`
(instr_plain
  (op_index) @_op
  (index
    (identifier) @name)
  (#match? @_op "^(return_)?call$")) @reference.call
