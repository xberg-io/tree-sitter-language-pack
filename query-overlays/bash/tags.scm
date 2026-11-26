; Functions are declarations wherever they sit; variables are only those assigned at the top
; level of the script (a `local` or an assignment inside a function is implementation).

; `name() { ... }` and `function name { ... }`
(function_definition name: (word) @name) @definition.function

; `readonly NAME=value` and `declare -r NAME=value` cannot be reassigned.
(program
  (declaration_command
    "readonly"
    (variable_assignment name: (variable_name) @name) @definition.constant))

; `NAME=value`
(program
  (variable_assignment name: (variable_name) @name) @definition.variable)

; `A=1 B=2` on one line is a `variable_assignments` run.
(program
  (variable_assignments
    (variable_assignment name: (variable_name) @name) @definition.variable))

; `export NAME=value` and `declare -x NAME=value`
(program
  (declaration_command
    (variable_assignment name: (variable_name) @name) @definition.variable))

; `build --release` and `echo "$PREFIX"`: a command runs a function, builtin or program.
(command
  name: (command_name (word) @name)) @reference.call
