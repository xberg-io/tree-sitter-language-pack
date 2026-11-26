(constant_definition
  . (identifier) @name) @definition.constant

; `define-data-var` and `define-map` declare contract storage.
(variable_definition
  . (identifier) @name) @definition.variable

(mapping_definition
  . (identifier) @name) @definition.variable

(token_definition
  (_ . (identifier) @name)) @definition.constant

(trait_definition
  . (identifier) @name) @definition.interface

(trait_definition
  (function_signature_for_trait
    . (identifier) @name) @definition.method)

; `define-public`, `define-private` and `define-read-only`.
(function_definition
  (_
    (function_signature . (identifier) @name))) @definition.function

(contract_function_call
  operator: (identifier) @name) @reference.call
