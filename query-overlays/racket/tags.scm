; Replaces the vendored tags.scm, whose call pattern has no @name capture.
;
; Top-level definition forms are the declarations: `(define (name args) ...)` and a `define` of a
; `lambda` define functions, any other `(define name value)` a variable, `define-syntax` and
; `define-syntax-rule` macros, and `struct` a structure type. Definitions inside bodies are local.

(program
  (list
    .
    (symbol) @_keyword
    .
    (list
      .
      (symbol) @name)
    (#any-of? @_keyword "define" "define/contract")) @definition.function)

(program
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    .
    (list
      .
      (symbol) @_lambda
      (#any-of? @_lambda "lambda" "λ" "case-lambda"))
    (#any-of? @_keyword "define" "define/contract")) @definition.function)

(program
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#any-of? @_keyword "define" "define/contract")) @definition.variable)

(program
  (list
    .
    (symbol) @_keyword
    .
    [
      (symbol) @name
      (list . (symbol) @name)
    ]
    (#any-of? @_keyword "define-syntax" "define-syntax-rule")) @definition.macro)

(program
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#eq? @_keyword "struct")) @definition.struct)

; `(module name lang ...)` and `(module* name lang ...)`
(program
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#any-of? @_keyword "module" "module*")) @definition.module)
