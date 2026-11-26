(protocol_declaration
  name: (identifier) @name) @definition.module

(record_declaration
  name: (identifier) @name) @definition.class

(error_declaration
  name: (identifier) @name) @definition.class

(enum_declaration
  name: (identifier) @name) @definition.enum

(enumeral
  (identifier) @name) @definition.constant

; `fixed Hash(16);`
(fixed_declaration
  (call_expression . (identifier) @name)) @definition.type

(rpc_message_declaration
  name: (identifier) @name) @definition.method

(logical_type
  (identifier) @name) @reference.type

(throw_statement
  (identifier) @name) @reference.type
