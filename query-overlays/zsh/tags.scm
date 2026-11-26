; Follows query-overlays/bash/tags.scm: functions are declarations wherever they sit; variables
; are only those assigned at the top level of the script (a `local` or an assignment inside a
; function is implementation).

; `name() { ... }` and `function name { ... }`
(function_definition name: (word) @name) @definition.function

; `NAME=value`
(program
  (variable_assignment name: (variable_name) @name) @definition.variable)

; `A=1 B=2` on one line is a `variable_assignments` run.
(program
  (variable_assignments
    (variable_assignment name: (variable_name) @name) @definition.variable))

; `export NAME=value`, `typeset -g NAME=value`, `readonly NAME=value`
(program
  (declaration_command
    (variable_assignment name: (variable_name) @name) @definition.variable))
