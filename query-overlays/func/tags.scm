(const_var_declarations
  name: (identifier) @name) @definition.constant

(global_var_declarations
  name: (identifier) @name) @definition.variable

(function_definition
  name: (function_name) @name) @definition.function

(function_application
  function: (identifier) @name) @reference.call

; `.method()` and `~method()` calls.
(method_call
  method_name: (identifier) @name) @reference.call
