; Based on queries/tags.scm of https://github.com/nilshelmig/tree-sitter-sourcepawn, without the
; documentation-comment variants (each re-tagged a node the plain patterns tag), with enums as
; enums, methodmap constructors as constructors and function type declarations as types.

; Types
(methodmap
  name: (identifier) @name) @definition.class

(enum_struct
  name: (identifier) @name) @definition.class

(enum
  name: (identifier) @name) @definition.enum

(enum_entry
  name: (identifier) @name) @definition.constant

; Function types: `typedef`, `typeset`, and the legacy `functag` and `funcenum`
[
  (typeset
    name: (identifier) @name)
  (typedef
    name: (identifier) @name)
  (functag
    name: (identifier) @name)
  (funcenum
    name: (identifier) @name)
] @definition.type

; Methods
[
  (enum_struct_method
    name: (identifier) @name)
  (methodmap_method
    name: (identifier) @name)
  (methodmap_method_destructor
    name: (identifier) @name)
] @definition.method

(methodmap_method_constructor
  name: (identifier) @name) @definition.constructor

(global_variable_declaration
  (variable_declaration
    name: (identifier) @name) @definition.variable)

; Functions, and `native` / `forward` declarations of ones defined elsewhere
[
  (function_definition
    name: (identifier) @name)
  (function_declaration
    name: (identifier) @name)
] @definition.function

; Macros: `#define MAX 64` and `#define SQUARE(%1) ((%1) * (%1))`
[
  (preproc_define
    name: (identifier) @name)
  (preproc_macro
    name: (identifier) @name)
] @definition.macro

; References
(call_expression
  function: (identifier) @name) @reference.call

(call_expression
  function: (field_access
    field: (identifier) @name)) @reference.call

(new_expression
  class: (identifier) @name) @reference.class
