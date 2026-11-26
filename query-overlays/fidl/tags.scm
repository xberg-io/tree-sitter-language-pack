(library
  (compound_identifier) @name) @definition.module

(const_declaration
  . (identifier) @name) @definition.constant

(type_alias_declaration
  . (identifier) @name) @definition.type

; `type X = enum {...}` and `bits` declare enumerations; `struct`, `table` and `union` declare
; record types.
(layout_declaration
  (identifier) @name
  (inline_layout (layout_kind) @_kind)
  (#any-of? @_kind "enum" "bits")) @definition.enum

(layout_declaration
  (identifier) @name) @definition.class

(value_layout_member
  . (identifier) @name) @definition.constant

(protocol_declaration
  (identifier) @name) @definition.interface

(protocol_method
  (identifier) @name) @definition.method

(protocol_event
  (identifier) @name) @definition.method

(service_declaration
  (identifier) @name) @definition.interface

; The last segment of a qualified name is the referenced type.
(layout
  (compound_identifier (identifier) @name .)) @reference.type
