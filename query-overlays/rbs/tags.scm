; A qualified name (`Foo::Bar`) is named by its last segment.
(module_decl
  name: (module_name (constant) @name .)) @definition.module

(class_decl
  name: (class_name (constant) @name .)) @definition.class

(interface_decl
  name: (interface_name (interface) @name .)) @definition.interface

(const_decl
  (const_name (constant) @name .)) @definition.constant

(type_alias_decl
  (alias_name (identifier) @name .)) @definition.type

(global_decl
  (global_name) @name) @definition.variable

(method_member
  name: (method_name) @name) @definition.method

(alias_member
  name: (method_name) @name) @definition.method

; `attr_reader`, `attr_writer` and `attr_accessor`, and instance variables.
(attribute_member
  name: (method_name) @name) @definition.field

(ivar_member
  name: (ivar_name) @name) @definition.field

(superclass
  name: (class_name (constant) @name .)) @reference.class

(include_member
  name: (class_name (constant) @name .)) @reference.module

(class_type
  (class_name (constant) @name .)) @reference.type
