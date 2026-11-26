; Declarations are the type signatures, which Idris requires for every top-level function; the
; defining clauses (`name x = ...`) are not tagged again. Signatures inside a `data` body declare
; its constructors and those inside an `interface` body its methods. Helpers in `where` and
; `let` blocks are local. An operator signature such as `(::) : ...` does not put the operator
; in the `name` field, so it is matched as the operator just before the colon.

(module
  name: (_) @name) @definition.module

(namespace
  name: (_) @name) @definition.module

(data
  name: (data_name) @name) @definition.type

(data_body
  [
    (signature name: (_) @name)
    (signature (operator) @name . (colon))
  ] @definition.constructor)

(record
  name: (record_name) @name) @definition.struct

(record_body
  (constructor
    name: (_) @name) @definition.constructor)

(record_body
  (record_field
    .
    (loname) @name) @definition.field)

(interface
  (interface_head
    name: (interface_name) @name)) @definition.interface

(interface_body
  [
    (signature name: (_) @name)
    (signature (operator) @name . (colon))
  ] @definition.method)

; `implementation Show Point where ...`: an implementation of the interface
(implementation
  (implementation_head
    subject: (interface_name) @name)) @reference.implementation

; Top-level signatures, also inside `namespace`, `mutual` and `parameters` blocks
(idris
  [
    (signature name: (_) @name)
    (signature (operator) @name . (colon))
  ] @definition.function)

(namespace_body
  [
    (signature name: (_) @name)
    (signature (operator) @name . (colon))
  ] @definition.function)

(mutual_body
  [
    (signature name: (_) @name)
    (signature (operator) @name . (colon))
  ] @definition.function)

(parameters_body
  [
    (signature name: (_) @name)
    (signature (operator) @name . (colon))
  ] @definition.function)
