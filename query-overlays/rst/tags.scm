; Section titles and substitution definitions (`.. |name| replace:: ...`, named as `|name|` is
; referenced). Hyperlink targets are not tagged: the grammar's target name keeps the leading `_`
; and trailing `:` of `.. _name:`, so the tag would not carry the name a reference uses.

(section (title) @name) @definition.section

(substitution_definition name: (substitution) @name) @definition.constant
