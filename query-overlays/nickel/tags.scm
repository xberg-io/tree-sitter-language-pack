; A Nickel file is one expression, and `let` is its only declaration form; record fields are
; configuration data. A query cannot tell the file's outer `let` chain from one nested in a
; function body, so every named binding is tagged. Destructuring patterns name no single value.

; `let make_service = fun name port => ...`
(let_binding
  pat: (pattern pat: (ident) @name)
  t1: (term (uni_term (fun_expr)))) @definition.function

; `let Service = { ... }` and `let Port = std.contract.from_predicate ...`
(let_binding pat: (pattern pat: (ident) @name)) @definition.variable
