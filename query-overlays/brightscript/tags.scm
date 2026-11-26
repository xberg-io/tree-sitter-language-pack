; Functions and subs
(function_statement
  name: (identifier) @name) @definition.function

(sub_statement
  name: (identifier) @name) @definition.function

; Calls: the callee is the last identifier of the called expression, so `obj.Method()` names
; `Method`
(function_call
  function: (prefix_exp
    (identifier) @name .)) @reference.call
