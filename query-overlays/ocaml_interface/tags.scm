; OCaml interfaces (.mli). Based on the tags.scm of github.com/tree-sitter/tree-sitter-ocaml,
; which used roles outside the tag vocabulary (enum_variant, field references) and never tagged
; `val` specifications. An interface holds no expressions, so only declarations and the
; module and type paths they mention are tagged. Kept in step with query-overlays/ocaml/tags.scm:
; the interface grammar shares its node types.

; Modules
(module_definition
  (module_binding (module_name) @name)) @definition.module

(module_path (module_name) @name) @reference.module

(extended_module_path (module_name) @name) @reference.module

(module_type_definition (module_type_name) @name) @definition.interface

(module_type_path (module_type_name) @name) @reference.interface

; Classes and methods; `class type name = object ... end` is a class signature.
(class_definition
  (class_binding (class_name) @name)) @definition.class

(class_type_definition
  (class_type_binding (class_type_name) @name)) @definition.interface

(class_type_path (class_type_name) @name) @reference.interface

(method_specification (method_name) @name) @definition.method

; Types: a record is a class, a variant an enum, anything else (abstract types, aliases) a type.
(type_binding
  name: (type_constructor) @name
  body: (record_declaration)) @definition.class

(type_binding
  name: (type_constructor) @name
  body: (variant_declaration)) @definition.enum

(type_binding
  name: [
    (type_constructor) @name
    (type_constructor_path (type_constructor) @name)
  ]) @definition.type

; Type uses, without the predefined types.
(type_constructor_path
  (type_constructor) @name
  (#not-match? @name "^(int|float|string|bool|char|unit|bytes|exn|list|array|option|result|ref|int32|int64|nativeint|lazy_t|format)$")) @reference.type

(exception_definition
  (constructor_declaration (constructor_name) @name)) @definition.class

(constructor_declaration (constructor_name) @name) @definition.constant

(field_declaration (field_name) @name) @definition.field

; `val name : a -> b`
(value_specification
  (value_name) @name
  type: (function_type)) @definition.function

(value_specification
  (value_name) @name) @definition.variable

(external (value_name) @name) @definition.function
