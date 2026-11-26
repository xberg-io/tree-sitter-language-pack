; Top-level `def`, `defn` and `defop` forms are the declarations. `defop` defines an operative,
; which receives its arguments unevaluated, like a macro. Forms inside bodies (`let` bindings,
; nested `def`s) are local.

(source
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#eq? @_keyword "defn")) @definition.function)

(source
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#eq? @_keyword "defop")) @definition.macro)

(source
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#eq? @_keyword "def")) @definition.variable)
