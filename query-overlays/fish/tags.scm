; Follows query-overlays/bash/tags.scm: functions are declarations wherever they sit; variables
; are only those set at the top level of the script (a `set` inside a function is
; implementation). The variable is the first `set` argument after up to two flags; flags that
; erase, query, list names or show (`-e`, `-q`, `-n`, `-S`) make the command a lookup.

; `function name --argument-names a b`
(function_definition name: (word) @name) @definition.function

; `set NAME value`
(program
  (command
    name: (word) @_cmd
    .
    argument: (word) @name) @definition.variable
  (#eq? @_cmd "set")
  (#not-match? @name "^-"))

; `set -gx NAME value`
(program
  (command
    name: (word) @_cmd
    .
    argument: (word) @_flag
    .
    argument: (word) @name) @definition.variable
  (#eq? @_cmd "set")
  (#match? @_flag "^-")
  (#not-match? @_flag "^-[a-zA-Z]*[eqnS]|^--(erase|query|names|show)")
  (#not-match? @name "^-"))

; `set -U -x NAME value`
(program
  (command
    name: (word) @_cmd
    .
    argument: (word) @_flag1
    .
    argument: (word) @_flag2
    .
    argument: (word) @name) @definition.variable
  (#eq? @_cmd "set")
  (#match? @_flag1 "^-")
  (#match? @_flag2 "^-")
  (#not-match? @_flag1 "^-[a-zA-Z]*[eqnS]|^--(erase|query|names|show)")
  (#not-match? @_flag2 "^-[a-zA-Z]*[eqnS]|^--(erase|query|names|show)")
  (#not-match? @name "^-"))
