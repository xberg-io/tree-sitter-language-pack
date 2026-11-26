; Headings are the sections of a Typst document, named by their text. `#let` bindings define
; functions (`#let f(x) = ...`) and variables (`#let x = ...`).

(heading (text) @name) @definition.section

(let pattern: (call item: (ident) @name)) @definition.function

(let pattern: (ident) @name) @definition.variable
