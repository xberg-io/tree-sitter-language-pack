; Fortran (`.f90` and friends). Based on the tags.scm of stadelmanma/tree-sitter-fortran, with
; the forms it left out added: derived-type components and type-bound procedures, module-level
; `parameter` constants and variables, and `use` statements. Programs, modules and submodules are
; modules; functions and subroutines are functions; a derived type is a class. A procedure
; declared in an `interface` block is a function, unless the block is `abstract`, where it
; describes a procedure interface; a named (generic) interface is an interface. Declarations
; inside a procedure body are locals.

(program_statement
  (name) @name) @definition.module

(module_statement
  (name) @name) @definition.module

(submodule_statement
  (module_name) (name) @name) @definition.module

(derived_type_statement
  (type_name) @name) @definition.class

; `real :: x, y` inside a derived type
(derived_type_definition
  (variable_declaration
    declarator: [(identifier) @name
                 (init_declarator left: (identifier) @name)]) @definition.field)

; `procedure :: norm => point_norm` inside a derived type
(procedure_statement
  declarator: (binding
    (binding_name
      (identifier) @name))) @definition.method

; `abstract interface` bodies
(interface
  (interface_statement
    (abstract_specifier))
  [(function
     (function_statement
       (name) @name))
   (subroutine
     (subroutine_statement
       (name) @name))] @definition.interface)

; `interface shape_area`
(interface_statement
  (name) @name) @definition.interface

(function_statement
  (name) @name) @definition.function

(subroutine_statement
  (name) @name) @definition.function

; `real, parameter :: pi = 3.14159` at module level
(module
  (variable_declaration
    attribute: (type_qualifier) @_qualifier
    declarator: [(identifier) @name
                 (init_declarator left: (identifier) @name)]) @definition.constant
  (#match? @_qualifier "^(?i)parameter$"))

; `integer :: shape_count = 0` at module level
(module
  (variable_declaration
    declarator: [(identifier) @name
                 (init_declarator left: (identifier) @name)]) @definition.variable)

; `module procedure circle_area` implements an interface declared in the parent module.
(module_procedure_statement
  (name) @name) @reference.implementation

; `use geometry`
(use_statement
  (module_name) @name) @reference.module

(call_expression
  (identifier) @name) @reference.call

(subroutine_call
  (identifier) @name) @reference.call

; `type, extends(point) :: labeled_point`
(base_type_specifier
  (identifier) @name) @reference.class

; `type(point) :: p` and `class(point) :: self`
(derived_type
  (type_name) @name) @reference.class
