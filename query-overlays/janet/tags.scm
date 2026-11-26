; Top-level forms are the declarations: `defn`/`varfn` define functions and `defmacro` macros
; (each with a `-` private variant), and `def`/`var` bind values. Bindings inside function
; bodies (`let`, nested `def`) are local.

(extra_defs
  [
    "defmacro"
    "defmacro-"
  ]
  name: (symbol) @name) @definition.macro

(extra_defs
  name: (symbol) @name) @definition.function

(source_file
  [
    (def name: (symbol) @name)
    (var name: (symbol) @name)
  ] @definition.variable)
