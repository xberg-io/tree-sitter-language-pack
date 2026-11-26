; MLIR (`.mlir`). Symbol-defining operations: named modules and functions with a body. Names keep
; their `@` sigil, as the IR writes and references them. SSA values (`%sum`) and block labels are
; local to a region and are not tagged. A function without a body (`func.func private @log(i32)`)
; declares a symbol defined elsewhere and is not tagged.

; `module @kernels { ... }`
(builtin_dialect
  name: (symbol_ref_id) @name) @definition.module

; `func.func @add(%a: i32) -> i32 { ... }` and `llvm.func @f() { ... }`
[(func_dialect
   name: (symbol_ref_id) @name
   body: (region))
 (llvm_dialect
   name: (symbol_ref_id) @name
   body: (region))] @definition.function

; `func.call @add(%x, %y)` and `call @add(...)`
(func_dialect
  callee: (symbol_ref_id) @name) @reference.call
