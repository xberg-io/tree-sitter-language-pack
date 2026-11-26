; Each entity becomes a Haskell record type with one field per column.
(entity_definition
  name: (type_name (type) @name)) @definition.class

(field_definition
  name: (variable) @name) @definition.field

; `UniqueEmail email` generates the `UniqueEmail` constructor.
(unique_constraint
  (constructor) @name) @definition.constant

(field_definition
  type: (type_name (type) @name)) @reference.type
