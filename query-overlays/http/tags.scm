; HTTP request files (`*.http`, REST Client and JetBrains HTTP Client). A request named with
; `# @name` can be referenced from later requests, so it is a function; the free-text title
; after `###` is a label, not an identifier, and is not tagged.

; `# @name listUsers`
(section
  (comment
    name: (identifier) @_directive
    value: (value) @name) @definition.function
  (#eq? @_directive "name"))

; `@host = https://api.example.com`
(variable_declaration name: (identifier) @name) @definition.variable

; `{{host}}`
(variable name: (identifier) @name) @reference.variable
