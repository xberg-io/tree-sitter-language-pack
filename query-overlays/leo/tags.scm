(program_declaration
  (program_id name: (program_name_literal) @name)) @definition.module

(constant_declaration
  . (identifier) @name) @definition.constant

(struct_declaration
  name: (identifier) @name) @definition.class

(record_declaration
  . (identifier) @name) @definition.class

; `mapping balances: address => u64;` is on-chain storage.
(mapping_declaration
  . (identifier) @name) @definition.variable

; Transitions are the program's entry points; functions and inlines are helpers.
(transition_declaration
  name: (identifier) @name) @definition.function

(function_declaration
  name: (identifier) @name) @definition.function

(inline_declaration
  name: (identifier) @name) @definition.function

(free_function_call
  . (identifier) @name) @reference.call

(struct_expression
  . (identifier) @name) @reference.class
