; CMake command names are case-insensitive, so the predicates match with `(?i)`. Variables count
; only when set at the top level of the file; a `set` inside a function or macro is local to it.

; `function(name args...)`
(function_def
  (function_command
    (argument_list . (argument (unquoted_argument) @name)))) @definition.function

; `macro(name args...)`
(macro_def
  (macro_command
    (argument_list . (argument (unquoted_argument) @name)))) @definition.macro

; `project(name ...)`
(source_file
  (normal_command
    (identifier) @_cmd
    (argument_list . (argument (unquoted_argument) @name))) @definition.module
  (#match? @_cmd "(?i)^project$"))

; `add_library(name ...)`, `add_executable(name ...)` and `add_custom_target(name ...)` declare
; build targets.
(normal_command
  (identifier) @_cmd
  (argument_list . (argument (unquoted_argument) @name))
  (#match? @_cmd "(?i)^(add_library|add_executable|add_custom_target)$")) @definition.class

; `set(NAME ...)` and `option(NAME "doc" OFF)`
(source_file
  (normal_command
    (identifier) @_cmd
    (argument_list . (argument (unquoted_argument) @name))) @definition.variable
  (#match? @_cmd "(?i)^(set|option)$"))

; Every command invocation, built-in or user-defined: `greeter_install(app)`
(normal_command (identifier) @name) @reference.call
