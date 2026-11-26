; Headings are the named sections of a Markdown document, named by their text. Link reference
; definitions are not tagged: the label node keeps its brackets, so the tag would not carry the
; name a reference uses.

(atx_heading heading_content: (inline) @name) @definition.section

(setext_heading heading_content: (paragraph (inline) @name)) @definition.section
