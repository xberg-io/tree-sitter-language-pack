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

; Event handlers have no name of their own; they are identified by the device (and channel or
; level) they handle, as written in the brackets
(button_event_declarator
  devchan: (button_event_devchan_reference) @name) @definition.event

(channel_event_declarator
  devchan: (channel_event_devchan_reference) @name) @definition.event

(data_event_declarator
  device: (data_event_device_reference) @name) @definition.event

(level_event_declarator
  devlev: (level_event_devlev_reference) @name) @definition.event

(timeline_event_declarator
  id: (timeline_event_id_reference) @name) @definition.event

(push_event_declarator
  devchan: (push_event_devchan_reference) @name) @definition.event

(release_event_declarator
  devchan: (release_event_devchan_reference) @name) @definition.event

(custom_event_declarator
  reference: (custom_event_reference) @name) @definition.event

; Calls
(call_expression
  function: (identifier) @name) @reference.call

(call_statement
  call: (string_literal
    (string_content) @name)) @reference.call
