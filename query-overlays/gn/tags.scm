; GN build files. The grammar parses a call's `{ ... }` body as the next sibling statement, not a
; child of the call, so patterns that need the body anchor on that sibling.

; `template("name") { ... }`
(call_expression
  function: (identifier) @_fn
  .
  (primary_expression (string (string_content) @name))
  (#eq? @_fn "template")) @definition.function

; `config("warnings") { ... }` and `toolchain("clang") { ... }` are named settings blocks.
(call_expression
  function: (identifier) @_fn
  .
  (primary_expression (string (string_content) @name))
  (#any-of? @_fn "config" "toolchain" "pool")) @definition.module

; A target is a statement-level call whose first argument is a literal name:
; `static_library("greet") { ... }` and a template invocation `greeter_test("unit") { ... }`.
; A template body names its target `target_name`, an identifier, so it declares no tag.
(source_file
  (primary_expression
    (call_expression
      function: (identifier) @_fn
      .
      (primary_expression (string (string_content) @name))) @definition.class)
  (#not-any-of? @_fn "template" "import" "print" "assert" "set_defaults" "set_default_toolchain"
    "declare_args" "forward_variables_from" "not_needed" "config" "toolchain" "pool"))

(block
  (primary_expression
    (call_expression
      function: (identifier) @_fn
      .
      (primary_expression (string (string_content) @name))) @definition.class)
  (#not-any-of? @_fn "template" "import" "print" "assert" "set_defaults" "set_default_toolchain"
    "declare_args" "forward_variables_from" "not_needed" "config" "toolchain" "pool"))

; `greeter_sources = [ ... ]` at the top level
(source_file
  (assignment_statement . (identifier) @name) @definition.variable)

; Build arguments inside `declare_args() { ... }`
(source_file
  (primary_expression
    (call_expression function: (identifier) @_fn))
  .
  (primary_expression
    (block (assignment_statement . (identifier) @name) @definition.variable))
  (#eq? @_fn "declare_args"))

; `import("//build/config/compiler.gni")`
(import_statement
  (primary_expression (string (string_content) @name))) @reference.module

; Every call, built-in function, target type or template: `executable(target_name)`
(call_expression function: (identifier) @name) @reference.call
