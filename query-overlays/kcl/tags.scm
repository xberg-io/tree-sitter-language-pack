(schema_stmt
  name: (identifier) @name) @definition.class

(protocol_stmt
  name: (identifier) @name) @definition.interface

(rule_stmt
  name: (identifier) @name) @definition.function

; Schema and protocol attributes.
(schema_stmt
  body: (block
    (assign_stmt
      left: (dotted_name . (identifier) @name)) @definition.field))

(protocol_stmt
  body: (block
    (assign_stmt
      left: (dotted_name . (identifier) @name)) @definition.field))

; Top-level assignments are the module's configuration output.
(module
  (assign_stmt
    left: (dotted_name . (identifier) @name)) @definition.variable)

; `App { ... }` instantiates a schema.
(schema_expr
  operand_name: (identifier) @name) @reference.class

(call_expr
  function: (identifier) @name) @reference.call
