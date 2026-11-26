; A TOML document's named blocks are its tables and array-of-tables headers; keys are data.

; `[package]` and `[target.'cfg(unix)'.dependencies]`
(table [(bare_key) (dotted_key) (quoted_key)] @name) @definition.module

; `[[bin]]`: the array is tagged at its first element; a following element of the same array
; is claimed untagged.
((table_array_element [(bare_key) (dotted_key) (quoted_key)] @_first)
  .
  (table_array_element [(bare_key) (dotted_key) (quoted_key)] @ignore)
  (#eq? @_first @ignore))

(table_array_element [(bare_key) (dotted_key) (quoted_key)] @name) @definition.module
