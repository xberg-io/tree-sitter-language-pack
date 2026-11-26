; Ada. Library units (packages and subprograms) and the declarations a package spec or body makes
; at package level. Declarations inside a subprogram body are locals and are not tagged.

; `package Shapes is` and `package body Shapes is`
(package_declaration
  name: (_) @name) @definition.module

(package_body
  name: (_) @name) @definition.module

; Library-level subprograms: `procedure Main is ... end Main;`
(compilation_unit
  [(subprogram_body
     [(function_specification name: (_) @name)
      (procedure_specification name: (_) @name)])
   (subprogram_declaration
     [(function_specification name: (_) @name)
      (procedure_specification name: (_) @name)])
   (generic_subprogram_declaration
     [(function_specification name: (_) @name)
      (procedure_specification name: (_) @name)])] @definition.function)

; Subprograms declared in a package spec
(package_declaration
  [(subprogram_declaration
     [(function_specification name: (_) @name)
      (procedure_specification name: (_) @name)])
   (expression_function_declaration
     (function_specification name: (_) @name))
   (null_procedure_declaration
     (procedure_specification name: (_) @name))
   (generic_subprogram_declaration
     [(function_specification name: (_) @name)
      (procedure_specification name: (_) @name)])] @definition.function)

; Subprograms declared or defined in a package body
(package_body
  (non_empty_declarative_part
    [(subprogram_body
       [(function_specification name: (_) @name)
        (procedure_specification name: (_) @name)])
     (subprogram_declaration
       [(function_specification name: (_) @name)
        (procedure_specification name: (_) @name)])
     (expression_function_declaration
       (function_specification name: (_) @name))
     (null_procedure_declaration
       (procedure_specification name: (_) @name))] @definition.function))

; Types, by what they define. `type Point is record ... end record;` is a struct, an enumeration
; type an enum, an interface type an interface; any other type or subtype is a type.
(package_declaration
  (full_type_declaration
    . (identifier) @name
    (record_type_definition)) @definition.struct)

(package_body
  (non_empty_declarative_part
    (full_type_declaration
      . (identifier) @name
      (record_type_definition)) @definition.struct))

(package_declaration
  (full_type_declaration
    . (identifier) @name
    (enumeration_type_definition)) @definition.enum)

(package_body
  (non_empty_declarative_part
    (full_type_declaration
      . (identifier) @name
      (enumeration_type_definition)) @definition.enum))

(package_declaration
  (full_type_declaration
    . (identifier) @name
    (interface_type_definition)) @definition.interface)

(package_body
  (non_empty_declarative_part
    (full_type_declaration
      . (identifier) @name
      (interface_type_definition)) @definition.interface))

(package_declaration
  [(full_type_declaration . (identifier) @name)
   (private_type_declaration . (identifier) @name)
   (private_extension_declaration . (identifier) @name)
   (subtype_declaration . (identifier) @name)
   (task_type_declaration . (identifier) @name)
   (protected_type_declaration . (identifier) @name)] @definition.type)

(package_body
  (non_empty_declarative_part
    [(full_type_declaration . (identifier) @name)
     (subtype_declaration . (identifier) @name)
     (task_type_declaration . (identifier) @name)
     (protected_type_declaration . (identifier) @name)] @definition.type))

; Package-level objects: `Max : constant Integer := 8;` and named numbers are constants, other
; objects are variables.
(package_declaration
  [(object_declaration
     name: (identifier) @name
     "constant")
   (number_declaration
     . (identifier) @name)] @definition.constant)

(package_body
  (non_empty_declarative_part
    [(object_declaration
       name: (identifier) @name
       "constant")
     (number_declaration
       . (identifier) @name)] @definition.constant))

(package_declaration
  (object_declaration
    name: (identifier) @name) @definition.variable)

(package_body
  (non_empty_declarative_part
    (object_declaration
      name: (identifier) @name) @definition.variable))

; Procedure calls: `Report (P);` and `Ada.Text_IO.Put_Line (S);`. Function calls are left out:
; the grammar cannot tell `F (X)` from an array index or a type conversion.
(procedure_call_statement
  name: [(identifier) @name
         (selected_component selector_name: (identifier) @name)]) @reference.call
