; Top-level forms are the declarations: named `fn` and `lambda` forms, `macro` forms, and
; `local`, `var` and `global` bindings. Bindings inside function bodies and `let` are local, and
; anonymous functions have no name.

(program
  [
    (fn name: (_) @name)
    (lambda name: (_) @name)
  ] @definition.function)

(program
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#eq? @_keyword "macro")) @definition.macro)

(program
  [
    (local (binding (symbol) @name))
    (var (binding (symbol) @name))
    (global (binding (symbol) @name))
  ] @definition.variable)
