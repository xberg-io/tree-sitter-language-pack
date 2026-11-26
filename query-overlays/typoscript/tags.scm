; TypoScript declares named objects by assigning a content-object type to a top-level path
; (`lib.footer = TEXT`); `{ ... }` blocks and plain value assignments only configure a path,
; and a numbered entry inside a block (`10 = TEXT`) is positional. `@import` paths are not
; tagged: the grammar keeps them together with their quotes.

; `lib.footer = TEXT` and `page = PAGE`
(typoscript
  (assignment_line (identifier) @name (cobject)) @definition.variable)

; `10 < lib.contentElement` copies an object.
(copy_line (copy_identifier) @name) @reference.variable

; `20 =< lib.footer` references an object.
(reference_line (reference_identifier) @name) @reference.variable
