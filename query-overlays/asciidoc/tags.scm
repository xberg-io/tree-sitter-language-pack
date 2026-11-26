; The document title and section titles of an AsciiDoc document, named by their text, plus block
; ids set with `[#id]`.

(document_title (line) @name) @definition.section

[
  (title1 (line) @name)
  (title2 (line) @name)
  (title3 (line) @name)
  (title4 (line) @name)
  (title5 (line) @name)
] @definition.section

(positional_attr (id) @name) @definition.label
