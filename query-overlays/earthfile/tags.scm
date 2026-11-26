; Earthfile targets and functions (a target whose body starts with `FUNCTION`, or `COMMAND` in
; older files) are the recipes other targets invoke, so both are functions. Only the base
; target's `ARG`s are file-wide variables; an `ARG` inside a target is that target's parameter.

; `build:`
(target name: (identifier) @name) @definition.function

; `ARG --global APP=server` in the base target
(source_file
  base_target: (block (arg_command name: (variable) @name) @definition.variable))

; `FROM +deps`, `BUILD +build`, `COPY +build/server ./`, also `./lib+target`
(target_ref name: (identifier) @name) @reference.call

; `DO +GO_BUILD`
(function_ref name: (identifier) @name) @reference.call

; `IMPORT ./lib AS lib`
(import_command alias: (identifier) @name) @definition.module
