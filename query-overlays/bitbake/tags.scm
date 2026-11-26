; BitBake recipes, classes and appends (`*.bb`, `*.bbclass`, `*.bbappend`). Only top-level
; statements are declarations; Python inside a task or `def` body is implementation.

; `SRC_URI = "..."`, `PV ?= "1.0"`, `DEPENDS:append = " zlib"`
(recipe
  (variable_assignment . (identifier) @name) @definition.variable)

; `export CC = "gcc"`
(recipe
  (export_statement
    (variable_assignment . (identifier) @name) @definition.variable))

; Shell task `do_compile() { ... }`
(recipe
  (function_definition . (identifier) @name) @definition.function)

; Python task `python do_report() { ... }`; an anonymous `python () { ... }` has no name.
(recipe
  (anonymous_python_function (identifier) @name) @definition.function)

; Python helper `def hello_flags(d):`
(recipe
  (python_function_definition name: (python_identifier) @name) @definition.function)

; `inherit autotools pkgconfig`: each class is a reference.
(inherit_directive (inherit_path) @name) @reference.module

; `include foo.inc` and `require foo.inc`
(include_directive (include_path) @name) @reference.module

(require_directive (include_path) @name) @reference.module
