; A TOML document's named blocks are its tables and array-of-tables headers; keys are data.

; `[package]` and `[target.'cfg(unix)'.dependencies]`
(table [(bare_key) (dotted_key) (quoted_key)] @name) @definition.module

; `[[bin]]`, one tag per element
(table_array_element [(bare_key) (dotted_key) (quoted_key)] @name) @definition.module
