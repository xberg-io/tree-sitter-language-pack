; Based on queries/tags.scm of https://github.com/tact-lang/tree-sitter-tact, without the
; documentation-comment wrappers. `init()` is the contract constructor; `receive`, `bounced` and
; `external` handlers are not tagged: their only name is the keyword, shared by every handler of
; a contract. Fields, storage variables and provided traits are added.

[
  (contract
    name: (identifier) @name)
  (message
    name: (type_identifier) @name)
  (struct
    name: (type_identifier) @name)
] @definition.class

(trait
  name: (identifier) @name) @definition.interface

[
  (native_function
    name: (identifier) @name)
  (asm_function
    name: (identifier) @name)
  (global_function
    name: (identifier) @name)
] @definition.function

(storage_function
  name: (identifier) @name) @definition.method

(init_function
  "init" @name) @definition.constructor

[
  (global_constant
    name: (identifier) @name)
  (storage_constant
    name: (identifier) @name)
] @definition.constant

; Struct and message fields, and contract or trait storage variables
[
  (field
    name: (identifier) @name)
  (storage_variable
    name: (identifier) @name)
] @definition.field

; `contract Counter with Ownable`
(trait_list
  (identifier) @name) @reference.implementation

[
  (method_call_expression
    name: (identifier) @name)
  (static_call_expression
    name: (identifier) @name)
] @reference.call

[
  (instance_expression
    name: (identifier) @name)
  (initOf
    name: (identifier) @name)
  (codeOf
    name: (identifier) @name)
] @reference.class
