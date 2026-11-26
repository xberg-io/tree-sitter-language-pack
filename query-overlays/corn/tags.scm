; Corn's only declarations are the inputs of the `let { ... }` block; object keys are data. An
; input is bound once and never reassigned, so it is a constant.

; `let { $name = value } in { ... }`
(assign_block (assignment (input) @name) @definition.constant)

; `key = $name` and `{ ..$name }`
(value (input) @name) @reference.constant

(spread (input) @name) @reference.constant
