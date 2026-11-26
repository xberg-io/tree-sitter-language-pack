; Follows query-overlays/bash/tags.scm: functions are declarations wherever they sit; variables
; are only those declared with `var` at the top level of the script (a `var` inside a function
; is implementation, and `set` reassigns rather than declares).

; `fn name {|args| ... }`
(function_definition . (identifier) @name) @definition.function

; `var prefix = /usr/local`, also `var a b = 1 2`
(source_file
  (variable_declaration (lhs (identifier) @name)) @definition.variable)

; `use str` and `use ./lib/util`
(import [(bareword) (string)] @name) @reference.module
