; Procedures, named as written (possibly namespace-qualified)
(procedure
  name: (simple_word) @name) @definition.function

; Namespaces: `namespace eval name { ... }`
(namespace
  (word_list
    .
    (simple_word) @_subcommand
    .
    (simple_word) @name)
  (#eq? @_subcommand "eval")) @definition.module

; Namespace variables: `variable name ?value?` directly in a `namespace eval` body
(namespace
  (word_list
    (braced_word
      (command
        name: (simple_word) @_command
        arguments: (word_list
          .
          (simple_word) @name)
        (#eq? @_command "variable")) @definition.variable)))

; Global variables set at the top level
(source_file
  (set
    (id) @name) @definition.variable)

; Calls: every command is a call, except `return` and `variable`, which are control flow and
; scope declarations
(command
  name: (simple_word) @name
  (#not-any-of? @name "return" "variable")) @reference.call
