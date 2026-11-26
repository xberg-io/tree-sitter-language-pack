; Djot. Link reference definitions and footnotes are the named targets of a Djot document, and
; reference links and footnote references point at them.
;
; Headings are not tagged: the grammar ends the heading `content` node after the line break, so
; its text keeps a trailing newline, and it has no child holding the bare text. Explicit ids
; (`{#usage}`) are not tagged either: the `identifier` node keeps the `#`.

; `[install]: https://example.com/install`
(link_reference_definition
  label: (link_label) @name) @definition.label

; `[^speed]: Parsing is incremental.`
(footnote
  label: (reference_label) @name) @definition.label

; `[the pack][install]`
(full_reference_link
  label: (link_label) @name) @reference.label

; `[^speed]`
(footnote_reference
  (reference_label) @name) @reference.label
