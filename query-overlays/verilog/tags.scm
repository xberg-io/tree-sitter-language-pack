(module_declaration
  (module_header (simple_identifier) @name)) @definition.module

(interface_declaration
  [
    (interface_ansi_header (interface_identifier) @name)
    (interface_nonansi_header (interface_identifier) @name)
  ]) @definition.interface

(package_declaration
  (package_identifier) @name) @definition.module

(class_declaration
  (class_identifier) @name) @definition.class

(text_macro_definition
  (text_macro_name (text_macro_identifier) @name)) @definition.macro

; `parameter` and `localparam`, in a module body or a parameter port list.
(parameter_declaration
  (list_of_param_assignments
    (param_assignment (parameter_identifier) @name))) @definition.constant

(local_parameter_declaration
  (list_of_param_assignments
    (param_assignment (parameter_identifier) @name))) @definition.constant

; Functions and tasks of a class are methods; the rest are functions.
(class_method
  [
    (function_declaration (function_body_declaration (function_identifier) @name))
    (task_declaration (task_body_declaration (task_identifier) @name))
  ]) @definition.method

(function_body_declaration
  (function_identifier) @name) @definition.function

(task_body_declaration
  (task_identifier) @name) @definition.function

(module_instantiation
  . (simple_identifier) @name) @reference.module

(tf_call
  . (simple_identifier) @name) @reference.call

(simple_text_macro_usage
  (text_macro_identifier) @name) @reference.macro
