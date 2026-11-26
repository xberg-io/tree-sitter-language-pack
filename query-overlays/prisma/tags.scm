; `datasource db {}` and `generator client {}`.
(key_value_block
  . (identifier) @name) @definition.module

(model_block
  . (identifier) @name) @definition.class

(model_field
  field_name: (identifier) @name) @definition.field

(enum_block
  . (identifier) @name) @definition.enum

(enum_block
  (identifier) @name @definition.constant)

(non_null_type
  (identifier) @name) @reference.type

(apply_function
  name: (identifier) @name) @reference.call
