; `let` statements are the declarations: a `let` bound to a function body (`(params) { ... }`)
; defines a function, any other `let` a named value. Calls are references.

(let_statement
  (identifier) @name
  (func)) @definition.function

(let_statement
  .
  (let_keyword)
  .
  (identifier) @name) @definition.variable

(function_call
  .
  (identifier) @name) @reference.call
