; NQC (`.nqc`), a C dialect for LEGO RCX bricks. Tasks, subroutines, functions, macros and global
; variables. Variables declared inside a task or function body are locals.

; `task main()`
(task_definition
  (identifier) @name) @definition.function

; `void forward(int t) { ... }` and `sub turn_around() { ... }`
(function_definition
  declarator: (function_declarator
                declarator: (identifier) @name)) @definition.function

; `#define SPEED 7`
[(preproc_def name: (identifier) @name)
 (preproc_function_def name: (identifier) @name)] @definition.macro

; `int counter;` at file scope
(translation_unit
  (declaration
    declarator: [(identifier) @name
                 (array_declarator declarator: (identifier) @name)
                 (init_declarator declarator: (identifier) @name)]) @definition.variable)

; `forward(5)` and API calls such as `OnFwd(OUT_A)`
(call_expression
  function: (identifier) @name) @reference.call
