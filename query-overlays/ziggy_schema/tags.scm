(struct
  name: (identifier) @name) @definition.class

(struct_field
  key: (identifier) @name) @definition.field

(union
  name: (identifier) @name) @definition.type

(union_field
  key: (identifier) @name) @definition.constant

; Types named by `$ = Config` and by field types (`[]Route`, `?Mode`).
(expr
  (identifier) @name) @reference.type
