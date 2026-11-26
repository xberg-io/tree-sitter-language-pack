; A tree-sitter corpus file is a list of named test cases; `tree-sitter test --include <name>`
; runs one by name, so each case is a function.

; `===\nFunction definition\n===`
(test (header (name) @name)) @definition.function
