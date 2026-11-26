; Starlark (`BUILD`, `BUILD.bazel`, `WORKSPACE`, `MODULE.bazel`, `*.bzl`, `*.star`). Only top-level
; statements are declarations; what a macro or function does inside its body is not.

; `def macro(name, srcs = []):`
(module
  (function_definition
    name: (identifier) @name) @definition.function)

; `DEPS = [...]` and `_private = 1`
(module
  (expression_statement
    (assignment
      left: (identifier) @name) @definition.variable))

; A rule invocation is a build target named by its `name` argument: `py_library(name = "lib")`
; is the tag `lib`. A target assigned to a variable (`x = my_rule(name = "t")`) is a target too.
(module
  (expression_statement
    (call
      function: (identifier)
      arguments: (argument_list
        (keyword_argument
          name: (identifier) @_arg
          value: (string (string_content) @name)))) @definition.class
    (#eq? @_arg "name")))

(module
  (expression_statement
    (assignment
      right: (call
        function: (identifier)
        arguments: (argument_list
          (keyword_argument
            name: (identifier) @_arg
            value: (string (string_content) @name)))) @definition.class)
    (#eq? @_arg "name")))

; `load("//tools:defs.bzl", "my_rule", helper = "helper_fn")` references the loaded module label.
(module
  (expression_statement
    (call
      function: (identifier) @_fn
      arguments: (argument_list . (string (string_content) @name))) @reference.module
    (#eq? @_fn "load")))
