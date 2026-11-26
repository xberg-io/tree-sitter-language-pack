; Common Lisp. Based on the tags.scm of github.com/theHamsta/tree-sitter-commonlisp,
; which tagged only `defun` and treated every other defining form (defpackage, defvar,
; defstruct, defclass slots) as a call. The grammar gives `defun`, `defmacro`, `defgeneric` and
; `defmethod` a `defun_header`; every other defining form is a plain `list_lit` matched by its
; head symbol. Earlier patterns win, so the definitions and @ignore claims precede the catch-all
; call pattern at the end.

; `(defun name (args) ...)`; a `(setf name)` function name is a list and is not tagged.
(defun_header
  keyword: (defun_keyword) @_keyword
  function_name: (sym_lit) @name
  (#match? @_keyword "(?i)^defun$")) @definition.function

(defun_header
  keyword: (defun_keyword) @_keyword
  function_name: (sym_lit) @name
  (#match? @_keyword "(?i)^defmacro$")) @definition.macro

; A generic function is declared once by `defgeneric`; each `defmethod` is a separate method
; object specialising it.
(defun_header
  keyword: (defun_keyword) @_keyword
  function_name: (sym_lit) @name
  (#match? @_keyword "(?i)^defgeneric$")) @definition.function

(defun_header
  keyword: (defun_keyword) @_keyword
  function_name: (sym_lit) @name
  (#match? @_keyword "(?i)^defmethod$")) @definition.method

; Parameters are not calls: `(defun f (a b))`, `(defmethod m ((s square)))`,
; `(defmacro m ((var x) &body body))`.
(defun_header
  lambda_list: (list_lit . [(sym_lit) (package_lit)] @ignore))

(defun_header
  lambda_list: (list_lit (list_lit . [(sym_lit) (package_lit)] @ignore)))

; `(defpackage :name ...)`, also written `#:name` or `name`.
(list_lit
  .
  (sym_lit) @_head
  .
  [
    (kwd_lit (kwd_symbol) @name)
    (sym_lit) @name
  ]
  (#match? @_head "(?i)^(cl:)?defpackage$")) @definition.module

(list_lit
  .
  (sym_lit) @_head
  .
  (sym_lit) @name
  (#match? @_head "(?i)^(cl:)?(defvar|defparameter)$")) @definition.variable

(list_lit
  .
  (sym_lit) @_head
  .
  (sym_lit) @name
  (#match? @_head "(?i)^(cl:)?defconstant$")) @definition.constant

; `(defstruct name ...)` and `(defstruct (name options...) ...)`; `(:include parent)` names the
; parent structure.
(list_lit
  .
  (sym_lit) @_head
  .
  [
    (sym_lit) @name
    (list_lit . (sym_lit) @name)
  ]
  (#match? @_head "(?i)^(cl:)?defstruct$")) @definition.class

(list_lit
  .
  (sym_lit) @_head
  .
  (list_lit
    (list_lit
      .
      (kwd_lit) @_option
      .
      (sym_lit) @name
      (#match? @_option "(?i)^:include$")))
  (#match? @_head "(?i)^(cl:)?defstruct$")) @reference.class

; Structure slots, `(x 0)`, are fields, not calls.
(list_lit
  .
  (sym_lit) @_head
  (list_lit . (sym_lit) @ignore)
  (#match? @_head "(?i)^(cl:)?defstruct$"))

; `(defclass name (superclasses...) (slots...))` and `define-condition`.
(list_lit
  .
  (sym_lit) @_head
  .
  (sym_lit) @name
  (#match? @_head "(?i)^(cl:)?(defclass|define-condition)$")) @definition.class

(list_lit
  .
  (sym_lit) @_head
  .
  (sym_lit)
  .
  (list_lit (sym_lit) @name)
  (#match? @_head "(?i)^(cl:)?(defclass|define-condition)$")) @reference.class

; Slot specifiers, `(name :initarg :name)`, are fields, not calls.
(list_lit
  .
  (sym_lit) @_head
  .
  (sym_lit)
  .
  (list_lit)
  .
  (list_lit (list_lit . (sym_lit) @ignore))
  (#match? @_head "(?i)^(cl:)?(defclass|define-condition)$"))

; Bindings of `let`, `flet` and friends are not calls: `(let ((var value)) ...)`.
(list_lit
  .
  (sym_lit) @_head
  .
  (list_lit (list_lit . [(sym_lit) (package_lit)] @ignore))
  (#match? @_head "(?i)^(cl:)?(let|let\\*|flet|labels|macrolet|symbol-macrolet)$"))

; `(make-instance 'name ...)` names a class.
(list_lit
  .
  (sym_lit) @_head
  .
  (quoting_lit (sym_lit) @name)
  (#match? @_head "(?i)^(cl:)?make-instance$")) @reference.class

; Any other list headed by a symbol is a call, except defining forms, special operators and the
; standard binding and control macros, which are syntax rather than calls.
(list_lit
  .
  [(sym_lit) (package_lit)] @name
  (#not-match? @name
    "(?i)^(cl:)?(def[a-z-]*|define-[a-z-]*|in-package|declare|declaim|quote|function|lambda|let|let\\*|flet|labels|macrolet|symbol-macrolet|if|progn|prog1|when|unless|cond|case|setq|setf|block|return-from|loop|dolist|dotimes|multiple-value-bind|destructuring-bind)$")) @reference.call
