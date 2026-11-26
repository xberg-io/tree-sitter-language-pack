; Top-level `def*` forms are the declarations: the symbol after the defining form is the name.
; Forms inside function bodies (`let` bindings, local `fn` names) are local.

; `(ns example.core ...)`
(source
  (list_lit
    .
    (sym_lit name: (sym_name) @_keyword)
    .
    (sym_lit name: (sym_name) @name)
    (#eq? @_keyword "ns")) @definition.module)

; `defn`, `defn-` and `defmulti`
(source
  (list_lit
    .
    (sym_lit name: (sym_name) @_keyword)
    .
    (sym_lit name: (sym_name) @name)
    (#any-of? @_keyword "defn" "defn-" "defmulti")) @definition.function)

; `defmacro`
(source
  (list_lit
    .
    (sym_lit name: (sym_name) @_keyword)
    .
    (sym_lit name: (sym_name) @name)
    (#eq? @_keyword "defmacro")) @definition.macro)

; `defprotocol` and `definterface`, then the method signatures they declare.
(source
  (list_lit
    .
    (sym_lit name: (sym_name) @_keyword)
    .
    (sym_lit name: (sym_name) @name)
    (#any-of? @_keyword "defprotocol" "definterface")) @definition.interface)

(source
  (list_lit
    .
    (sym_lit name: (sym_name) @_keyword)
    (list_lit
      .
      (sym_lit name: (sym_name) @name)) @definition.method
    (#any-of? @_keyword "defprotocol" "definterface")))

; `defrecord` and `deftype`
(source
  (list_lit
    .
    (sym_lit name: (sym_name) @_keyword)
    .
    (sym_lit name: (sym_name) @name)
    (#any-of? @_keyword "defrecord" "deftype")) @definition.class)

; `def` and `defonce`
(source
  (list_lit
    .
    (sym_lit name: (sym_name) @_keyword)
    .
    (sym_lit name: (sym_name) @name)
    (#any-of? @_keyword "def" "defonce")) @definition.variable)
