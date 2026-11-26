; `package example:http@0.1.0;` is named by the package, after the namespace.
(package_decl
  (decl_head (id) . (id) @name)) @definition.module

(interface_item
  name: (id) @name) @definition.interface

; A world is the import/export contract of a component.
(world_item
  name: (id) @name) @definition.module

(type_item
  alias: (id) @name) @definition.type

(record_item
  name: (id) @name) @definition.class

(variant_items
  name: (id) @name) @definition.type

(variant_case
  name: (id) @name) @definition.constant

(enum_items
  name: (id) @name) @definition.enum

(enum_case) @name @definition.constant

(flags_items
  name: (id) @name) @definition.enum

(flags_field) @name @definition.constant

(resource_item
  name: (id) @name) @definition.class

; Resource methods (`read: func(...)`) and static functions (`from-string: static func(...)`);
; constructors have no name.
(resource_method
  (func_item name: (id) @name)) @definition.method

(resource_method
  name: (id) @name) @definition.method

(func_item
  name: (id) @name) @definition.function

; `export handler: func(...)` and `import name: func(...)` in a world.
(export_item
  name: (id) @name
  (extern_type (func_type))) @definition.function

(import_item
  name: (id) @name
  (extern_type (func_type))) @definition.function

(ty
  (id) @name) @reference.type

; `import types;` and `export types;` name an interface.
(use_path
  (id) @name .) @reference.interface
