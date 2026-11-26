; Functions: DEFINE_FUNCTION bodies and library function declarations
(function_definition
  name: (identifier) @name) @definition.function

(function_declaration
  name: (identifier) @name) @definition.function

; DEFINE_CALL subroutines are named by a string: `DEFINE_CALL 'Power On'`
(call_definition
  name: (string_literal
    (string_content) @name)) @definition.function

; Structures
(struct_specifier
  name: (type_identifier) @name) @definition.class

; Module instances: `DEFINE_MODULE 'ModuleName' instance(...)`
(module_definition
  instance_name: (identifier) @name) @definition.module

(module_definition
  module_name: (string_literal
    (string_content) @name)) @reference.module

; Devices: `dvTP = 10001:1:0` in DEFINE_DEVICE
(source_file
  (expression_statement
    (assignment_expression
      left: (identifier) @name
      right: (device_literal))) @definition.constant)

; Top-level declarations. ~keep The DEFINE_CONSTANT / DEFINE_VARIABLE section keyword is a
; ~keep preceding sibling, not a parent, so only an explicit CONSTANT qualifier marks a constant.
(source_file
  (declaration
    (type_qualifier
      (constant_keyword))
    declarator: [
      (identifier) @name
      (init_declarator
        declarator: [
          (identifier) @name
          (array_declarator
            declarator: (identifier) @name)
        ])
      (array_declarator
        declarator: (identifier) @name)
    ]) @definition.constant)

(source_file
  (declaration
    declarator: [
      (identifier) @name
      (init_declarator
        declarator: [
          (identifier) @name
          (array_declarator
            declarator: (identifier) @name)
        ])
      (array_declarator
        declarator: (identifier) @name)
    ]) @definition.variable)

; Event handlers (BUTTON_EVENT[dvTP, 1] and the like) are not tagged: they have no name of their
; own, only the bracketed device/channel expression they handle, which is neither an identifier
; nor unique per handler.

; Calls
(call_expression
  function: (identifier) @name) @reference.call

(call_statement
  call: (string_literal
    (string_content) @name)) @reference.call
