; CUE fields are data except for definitions (`#Name`), which are schemas at any depth. Regular
; fields and `let` bindings count only at the top level of the file, where they name the
; configuration values a package exports.

; `package deploy`
(package_clause (package_identifier) @name) @definition.module

; `#Service: { ... }`, also nested inside another definition
(field
  (label (identifier) @name)
  (#match? @name "^#")) @definition.type

; `api: #Service & { ... }`
(source_file
  (field (label (identifier) @name)) @definition.field)

; `let _registry = "..."`
(source_file
  (let_clause left: (identifier) @name) @definition.variable)

; `port: #Port` and `#Service & { ... }`
((identifier) @name @reference.type
  (#match? @name "^#"))
