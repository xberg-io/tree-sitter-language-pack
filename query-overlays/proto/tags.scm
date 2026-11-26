; Based on queries/tags.scm of https://github.com/coder3101/tree-sitter-proto, extended with
; message fields, oneofs and imports.

(package
  (full_ident) @name) @definition.module

(message
  (message_name
    (identifier) @name)) @definition.class

; proto2 groups declare a nested message type
(group
  (message_name
    (identifier) @name)) @definition.class

(enum
  (enum_name
    (identifier) @name)) @definition.enum

(enum_field
  (identifier) @name) @definition.constant

(service
  (service_name
    (identifier) @name)) @definition.interface

(rpc
  (rpc_name
    (identifier) @name)) @definition.method

; `optional string name = 1;`, `map<string, int32> counts = 2;` and the fields of a oneof
[
  (field
    (identifier) @name)
  (map_field
    (identifier) @name)
  (oneof_field
    (identifier) @name)
] @definition.field

; `oneof contact { ... }` names a field set
(oneof
  (identifier) @name) @definition.field

; `import "google/protobuf/timestamp.proto";`
(import
  path: (string
    (string_content) @name)) @reference.module

; The last segment of a qualified name is the referenced type.
(message_or_enum_type
  (identifier) @name .) @reference.type

(extend
  (full_ident
    (identifier) @name .)) @reference.type
