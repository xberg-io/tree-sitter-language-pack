; AsciiDoc. The attributes a document header defines (`:name: value`) and the block ids set with
; `[#id]`, which cross references (`<<id>>`) target.
;
; Section titles and the document title are not tagged: the grammar's `line` node ends after the
; line break, so its text keeps a trailing newline, and it has no child holding the bare title.
; `[[id]]` anchors are not tagged either: the block grammar reads them as paragraph text.

; `:product-name: Language Pack`, without the built-in attributes that configure rendering.
(document_attr
  (attr_name) @name
  (#not-match? @name "^(toc|toclevels|toc-title|sectnums|sectanchors|sectlinks|numbered|icons|imagesdir|iconsdir|source-highlighter|doctype|experimental|idprefix|idseparator|nofooter|showtitle|stem|xrefstyle)$")) @definition.variable

; `[#config]`
(positional_attr (id) @name) @definition.label
