; A Dhall file is one expression, and `let` is its only declaration form. A query cannot tell
; the file's outer `let` chain from one nested in a function body, so every binding is tagged.

; `let mkService = \(name : Text) -> ...`
(let_binding
  label: (label) @name
  (expression (lambda_expression))) @definition.function

; `let Service = { name : Text }` and `let Color = < Red | Green >`
(let_binding
  label: (label) @name
  (expression (primitive_expression [(record_type) (union_type)]))) @definition.type

; `let defaultPort = 8080`
(let_binding label: (label) @name) @definition.variable

; `./Prelude/package.dhall`, `https://prelude.dhall-lang.org/package.dhall`, `env:HOME`
(import [(local_import) (http_import) (env_import)] @name) @reference.module
