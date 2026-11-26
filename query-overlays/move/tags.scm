; The vendored grammar covers modules, `use` and functions only: structs, constants and specs
; do not parse, so they have no patterns here.
(module
  (identifier) @name) @definition.module

(function_item
  name: (identifier) @name) @definition.function

(call_expression
  function: [
    (identifier) @name
    (scoped_identifier name: (identifier) @name)
  ]) @reference.call
