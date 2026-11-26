; Section titles. Substitution definitions (`.. |name| replace:: ...`) are not tagged: the
; grammar's `substitution` node is a leaf that keeps the surrounding `|` delimiters, so there is no
; node holding the bare name. Hyperlink targets are not tagged either: the grammar's target name
; keeps the leading `_` and trailing `:` of `.. _name:`, so the tag would not carry the name a
; reference uses.

(section (title) @name) @definition.section
