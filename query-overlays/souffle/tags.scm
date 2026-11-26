; Declarations: relations (`.decl`), types (`.type`, including the constructors of an algebraic
; type), user-defined functors (`.functor`) and components (`.comp`). Rules and facts populate
; relations declared elsewhere.

(relation_decl
  head: (ident) @name) @definition.function

(type_decl
  [
    (subtype left: (ident) @name)
    (type_synonym left: (ident) @name)
    (type_union left: (ident) @name)
    (type_record left: (ident) @name)
    (adt left: (ident) @name)
    (legacy_bare_type_decl name: (ident) @name)
  ]) @definition.type

; The constructors of `.type T = A {...} | B {...}` are the identifiers after `T`, which the
; pattern above claims as the type. Queries cannot use the grammar's `constructor` field: it is
; nested inside the `branch` field.
(adt
  (ident) @name) @definition.constructor

(functor_decl
  name: (ident) @name) @definition.function

(component_decl
  type: (component_type name: (ident) @name)) @definition.class
