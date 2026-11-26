; Help tags (`*name*`) are the named targets of a Vim help file and `|name|` links reference them.
; Headings are its sections.

(tag text: (word) @name) @definition.label

(taglink text: (word) @name) @reference.label

[
  (h1 (heading) @name)
  (h2 (heading) @name)
  (h3 (heading) @name)
] @definition.section
