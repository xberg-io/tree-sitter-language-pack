; A git config file's named blocks are its sections. Keys are fixed by git (`url`, `merge`),
; except under `[alias]`, where each key defines a git subcommand.

; `[remote "origin"]` and `[branch "main"]` are named by their subsection.
(section (section_header (section_name) (subsection_name) @name)) @definition.module

; `[core]`
(section (section_header (section_name) @name .)) @definition.module

; `[alias]` `st = status --short`
(section
  (section_header (section_name) @_section .)
  (variable (name) @name) @definition.function
  (#match? @_section "(?i)^alias$"))
