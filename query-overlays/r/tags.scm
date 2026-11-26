; R tags: the vendored tags.scm of r-lib/tree-sitter-r, plus right assignment, S4 and R6 classes,
; top-level constants and package loads. A quoted target (`"f" <- function...`) is named by its
; string content, not the quoted literal.

; Functions: `f <- function(x)`, `f = function(x)`, `f <<- \(x)`.
(binary_operator
  lhs: [
    (identifier) @name
    (string
      content: (string_content) @name)
  ]
  operator: [
    "<-"
    "<<-"
    "="
  ]
  rhs: (function_definition)) @definition.function

; Right assignment: `(function(x) x + 1) -> add_one`. Without the parentheses R folds the
; assignment into the function body, so there is no unparenthesized form to match.
(binary_operator
  lhs: (parenthesized_expression
    (function_definition))
  operator: [
    "->"
    "->>"
  ]
  rhs: (identifier) @name) @definition.function

; Classes: `setClass("Person", ...)`, `setRefClass("Account", ...)`, `R6Class("Account", ...)`.
; Assigned to a generator (`Person <- setClass("Person", ...)`), the class is tagged once, at
; the assigned name, and the class name string is claimed without a tag.
(binary_operator
  lhs: (identifier) @name
  operator: [
    "<-"
    "<<-"
    "="
  ]
  rhs: (call
    function: (identifier) @_fn)
  (#match? @_fn "^(setClass|setRefClass|R6Class)$")) @definition.class

(binary_operator
  rhs: (call
    function: (identifier) @_fn
    arguments: (arguments
      .
      (argument
        value: (string
          content: (string_content) @ignore))))
  (#match? @_fn "^(setClass|setRefClass|R6Class)$"))

(call
  function: (identifier) @_fn
  arguments: (arguments
    .
    (argument
      value: (string
        content: (string_content) @name)))
  (#match? @_fn "^(setClass|setRefClass|R6Class)$")) @definition.class

; S4 generics: `setGeneric("greet", ...)`.
(call
  function: (identifier) @_fn
  arguments: (arguments
    .
    (argument
      value: (string
        content: (string_content) @name)))
  (#eq? @_fn "setGeneric")) @definition.function

; S4 methods: `setMethod("greet", "Person", function(obj) ...)` defines `greet` for a class.
(call
  function: (identifier) @_fn
  arguments: (arguments
    .
    (argument
      value: (string
        content: (string_content) @name)))
  (#eq? @_fn "setMethod")) @definition.method

; R6 members: functions in the `public`, `private` or `active` lists are methods, the other
; entries fields. The method pattern comes first so it claims function-valued entries.
(call
  function: (identifier) @_fn
  arguments: (arguments
    (argument
      name: (identifier) @_section
      value: (call
        function: (identifier) @_list
        arguments: (arguments
          (argument
            name: (identifier) @name
            value: (function_definition)) @definition.method))))
  (#eq? @_fn "R6Class")
  (#match? @_section "^(public|private|active)$")
  (#eq? @_list "list"))

(call
  function: (identifier) @_fn
  arguments: (arguments
    (argument
      name: (identifier) @_section
      value: (call
        function: (identifier) @_list
        arguments: (arguments
          (argument
            name: (identifier) @name) @definition.field))))
  (#eq? @_fn "R6Class")
  (#match? @_section "^(public|private|active)$")
  (#eq? @_list "list"))

; Top-level bindings that are not functions (the patterns above claim those): UPPER_CASE names
; are constants, the rest variables. Assignments inside functions are locals and not tagged.
(program
  (binary_operator
    lhs: (identifier) @name
    operator: [
      "<-"
      "<<-"
      "="
    ]) @definition.constant
  (#match? @name "^[A-Z][A-Z0-9_.]*$"))

(program
  (binary_operator
    lhs: (identifier) @name
    operator: [
      "<-"
      "<<-"
      "="
    ]) @definition.variable)

; Package loads: `library(methods)`, `require(R6)`.
(call
  function: (identifier) @_fn
  arguments: (arguments
    .
    (argument
      value: (identifier) @name))
  (#match? @_fn "^(library|require|requireNamespace)$")) @reference.module

; Calls: `f(x)`, `pkg::f(x)`.
(call
  function: (identifier) @name) @reference.call

(call
  function: (namespace_operator
    rhs: (identifier) @name)) @reference.call
