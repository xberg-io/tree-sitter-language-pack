; Top-level definition forms are the declarations: `(define (name args) ...)` and a `define` of a
; `lambda` define functions, any other `(define name value)` a variable, `define-syntax` a macro,
; and `define-record-type` a record type. Definitions inside bodies are local.

(program
  (list
    .
    (symbol) @_keyword
    .
    (list
      .
      (symbol) @name)
    (#eq? @_keyword "define")) @definition.function)

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
      (#any-of? @_lambda "lambda" "case-lambda"))
    (#eq? @_keyword "define")) @definition.function)

(program
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#eq? @_keyword "define")) @definition.variable)

(program
  (list
    .
    (symbol) @_keyword
    .
    (symbol) @name
    (#any-of? @_keyword "define-syntax" "define-macro")) @definition.macro)

(program
  (list
    .
    (symbol) @_keyword
    .
    [
      (symbol) @name
      (list . (symbol) @name)
    ]
    (#eq? @_keyword "define-record-type")) @definition.class)
