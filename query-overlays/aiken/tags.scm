; A record type `pub type Datum { owner: ByteArray }` is a type_struct; a type with several
; constructors is a type_enum whose constructors are variants.
(type_struct
  (type_struct_inner
    . (type_definition . (type_identifier) @name))) @definition.class

(type_enum
  . (type_definition . (type_identifier) @name)) @definition.enum

(type_enum_variant
  . (type_identifier) @name) @definition.constant

(type_alias
  . (type_definition . (type_identifier) @name)) @definition.type

(constant
  . (identifier) @name) @definition.constant

(function
  . (identifier) @name) @definition.function

; A validator groups the handlers (`spend`, `mint`, ...) the ledger calls.
(validator
  . (identifier) @name) @definition.class

(validator_hook
  . (identifier) @name) @definition.method

(test
  . (identifier) @name) @definition.function

(call
  . (identifier) @name) @reference.call

(call
  . (field_access (field_identifier) @name)) @reference.call

(type_definition
  . (type_identifier) @name) @reference.type
