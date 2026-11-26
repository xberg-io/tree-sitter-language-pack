; LLVM IR (`.ll`). Module-level definitions: functions, global variables, aliases and named
; types. Names keep their sigil (`@add`, `%struct.Point`), as the IR writes and references them.
; Virtual registers and basic-block labels inside a function body are local and are not tagged.
; `declare` names a function defined in another module and is not tagged.

; `define i32 @add(i32 %a, i32 %b) { ... }`
(fn_define
  (function_header
    name: (global_var) @name)) @definition.function

; `@.str = private constant [4 x i8] c"..."`
(global_global
  .
  (global_var) @name
  "constant") @definition.constant

; `@counter = global i32 0`
(global_global
  .
  (global_var) @name) @definition.variable

; `@resolved = ifunc ptr (), ptr @resolver`
(ifunc
  .
  (global_var) @name) @definition.function

; `@sum = alias i32 (i32, i32), ptr @add`: an alias whose value type is a function type names a
; function; `@total = alias i32, ptr @counter` names a variable.
(alias
  .
  (global_var) @name
  .
  (type (argument_list))) @definition.function

(alias
  .
  (global_var) @name) @definition.variable

; `%struct.Point = type { i32, i32 }`
(global_type
  .
  (local_var) @name) @definition.type

; `call i32 @add(i32 1, i32 2)` and `invoke void @may_throw()`
[(instruction_call
   callee: (value (var (global_var) @name)))
 (instruction_invoke
   callee: (value (var (global_var) @name)))] @reference.call
