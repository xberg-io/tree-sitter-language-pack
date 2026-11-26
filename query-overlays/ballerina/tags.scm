; ~keep This grammar has no fields; a definition's name is its direct (identifier) child.

; Functions and methods
(function_defn
  (identifier) @name) @definition.function

(method_defn
  (identifier) @name) @definition.method

(remote_method_defn
  (identifier) @name) @definition.method

(method_decl
  (identifier) @name) @definition.method

(remote_method_decl
  (identifier) @name) @definition.method

; Classes, types and enums
(class_defn
  (identifier) @name) @definition.class

(type_defn
  (identifier) @name) @definition.type

(enum_decl
  (identifier) @name) @definition.enum

; Module-level constants and final variables
(const_defn
  (identifier) @name) @definition.constant

(module_defn
  (final_defn
    (typed_binding_pattern
      (binding_pattern
        (identifier) @name)))) @definition.variable

; Class fields
(class_member
  (object_field
    (identifier) @name)) @definition.field

; Imports
(import_decl
  (module_name) @name) @reference.module

; Calls
(function_call_expr
  (function_reference
    (identifier) @name)) @reference.call

(function_call_expr
  (function_reference
    (qualified_identifier
      (identifier) @name))) @reference.call

(method_call_expr
  (identifier) @name
  .
  (arg_list)) @reference.call
