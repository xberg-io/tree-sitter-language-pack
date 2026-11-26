;; Emacs Lisp. Based on the tags.scm of github.com/Wilfred/tree-sitter-elisp, which tagged only
;; `defun`, `defsubst` and `defmacro` (macros as functions). The grammar gives `defun`,
;; `defsubst` and `defmacro` their own nodes and `defvar` and `defconst` a `special_form`; every
;; other defining form (`defcustom`, `define-minor-mode`, `cl-defstruct`) is a plain list matched
;; by its head symbol. Earlier patterns win, so the definitions and @ignore claims precede the
;; catch-all call pattern at the end.

(function_definition name: (symbol) @name) @definition.function

(macro_definition name: (symbol) @name) @definition.macro

;; Parameter lists and `let` bindings are not calls.
(function_definition parameters: (list . (symbol) @ignore))

(macro_definition parameters: (list . (symbol) @ignore))

(special_form (list (list . (symbol) @ignore)))

(special_form
  "defvar"
  .
  (symbol) @name) @definition.variable

(special_form
  "defconst"
  .
  (symbol) @name) @definition.constant

(list
  .
  (symbol) @_head
  .
  (symbol) @name
  (#eq? @_head "defcustom")) @definition.variable

;; `define-minor-mode` and `define-derived-mode` define the mode command.
(list
  .
  (symbol) @_head
  .
  (symbol) @name
  (#match? @_head "^define-(minor|derived|globalized-minor)-mode$")) @definition.function

(list
  .
  (symbol) @_head
  .
  [
    (symbol) @name
    (list . (symbol) @name)
  ]
  (#eq? @_head "cl-defstruct")) @definition.class

;; `(provide 'feature)` and `(require 'feature)`
(list
  .
  (symbol) @_head
  .
  (quote (symbol) @name)
  (#eq? @_head "provide")) @definition.module

(list
  .
  (symbol) @_head
  .
  (quote (symbol) @name)
  (#eq? @_head "require")) @reference.module

;; Any other list headed by a symbol is a call, except the defining forms.
(list
  .
  (symbol) @name
  (#not-match? @name "^(def[a-z-]*|define-[a-z-]*|cl-def[a-z-]*)$")) @reference.call
