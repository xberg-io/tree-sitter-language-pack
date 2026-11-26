(namespace_decl
  namespace_ident: (full_ident) @name) @definition.module

(enum_decl
  enum_name: (identifier) @name) @definition.enum

(enum_val_decl
  enum_key: (identifier) @name) @definition.constant

(union_decl
  union_name: (identifier) @name) @definition.class

; `table` and `struct`.
(type_decl
  table_or_struct_name: (identifier) @name) @definition.class

(rpc_decl
  rpc_name: (identifier) @name) @definition.interface

(rpc_method
  rpc_method_name: (identifier) @name) @definition.method

; The last segment of a qualified name is the referenced type.
(type
  full_ident: (full_ident (identifier) @name .)) @reference.type

(union_decl
  field_without_type: (full_ident (identifier) @name .)) @reference.type

(rpc_method
  [
    rpc_parameter: (identifier) @name
    rpc_return_type: (identifier) @name
  ]) @reference.type

(root_decl
  root_type_ident: (identifier) @name) @reference.type
