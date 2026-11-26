; Methods: `_method exemplar.name(...)`; the exemplar is a class reference
(method
  name: (identifier) @name) @definition.method

(method
  exemplarname: (identifier) @name) @reference.class

; Globals at the top level of a file: `_global [_constant] name << value`. A procedure value
; makes the global a function.
(fragment
  "_global"
  .
  (identifier) @name
  .
  (procedure) @definition.function)

(fragment
  "_global"
  .
  "_constant"
  .
  (identifier) @name @definition.constant)

(fragment
  "_global"
  .
  (identifier) @name @definition.variable)

; Calls: message sends `receiver.message()` and procedure invocations `proc()`
(call
  message: (identifier) @name) @reference.call

(invoke
  receiver: (variable) @name) @reference.call
