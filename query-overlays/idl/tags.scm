(module_dcl
  . (identifier) @name) @definition.module

(const_dcl
  (identifier) @name) @definition.constant

(typedef_dcl
  (type_declarator
    (any_declarators
      (any_declarator (_ . (identifier) @name))))) @definition.type

(enum_dcl
  . (identifier) @name) @definition.enum

(enumerator
  . (identifier) @name) @definition.constant

(struct_def
  . (identifier) @name) @definition.class

(union_def
  . (identifier) @name) @definition.class

(except_dcl
  . (identifier) @name) @definition.class

(interface_def
  (interface_header (identifier) @name)) @definition.interface

(op_dcl
  (identifier) @name) @definition.method

; The last segment of a scoped name (`a::b::T`) is the referenced type.
(simple_type_spec
  (scoped_name (identifier) @name .)) @reference.type

(raises_expr
  (scoped_name (identifier) @name .)) @reference.type
