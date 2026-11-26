; Top-level declarations: types with their record fields, abilities with their operations, and
; terms. A term is tagged at its definition, not at its optional type signature. Terms defined
; inside a term's block are local. The grammar does not group a type's constructors with their
; arguments, so constructors are not tagged.

(unison
  (type_declaration
    (type_constructor
      (type_name) @name)) @definition.type)

(record_field
  (field_name) @name) @definition.field

(unison
  (ability_declaration
    (ability_name) @name) @definition.interface)

(ability_declaration
  (constructor
    (constructor_name) @name) @definition.method)

(unison
  (term_declaration
    (term_definition
      name: (_) @name)) @definition.function)
