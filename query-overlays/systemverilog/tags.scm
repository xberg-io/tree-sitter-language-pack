(module_declaration
  [
    (module_ansi_header name: (_) @name)
    (module_nonansi_header name: (_) @name)
  ]) @definition.module

(interface_declaration
  [
    (interface_ansi_header name: (_) @name)
    (interface_nonansi_header name: (_) @name)
  ]) @definition.interface

(program_declaration
  [
    (program_ansi_header name: (_) @name)
    (program_nonansi_header name: (_) @name)
  ]) @definition.module

(package_declaration
  name: (_) @name) @definition.module

(class_declaration
  name: (_) @name) @definition.class

(interface_class_declaration
  name: (_) @name) @definition.interface

(checker_declaration
  name: (_) @name) @definition.module

(covergroup_declaration
  name: (_) @name) @definition.class

; `typedef ... word_t;`
(type_declaration
  type_name: (_) @name) @definition.type

; `parameter` and `localparam`, in a module, package or parameter port list.
(parameter_declaration
  (list_of_param_assignments
    (param_assignment . (_) @name))) @definition.constant

(local_parameter_declaration
  (list_of_param_assignments
    (param_assignment . (_) @name))) @definition.constant

; Functions and tasks of a class are methods; the rest are functions.
(class_method
  [
    (function_declaration (function_body_declaration name: (_) @name))
    (task_declaration (task_body_declaration name: (_) @name))
    (function_prototype name: (_) @name)
    (task_prototype name: (_) @name)
  ]) @definition.method

(function_body_declaration
  name: (_) @name) @definition.function

(task_body_declaration
  name: (_) @name) @definition.function

(module_instantiation
  instance_type: (_) @name) @reference.module

; An interface or program instance (`fifo_if bus (...)`) is written like a module instance, and
; the grammar parses all three as module_instantiation: it cannot know what the name refers to,
; so interface_instantiation and program_instantiation are not matched here.

; `f(x)`, `pkg::f(x)` and `obj.method(x)`: the last segment names the called subroutine.
(tf_call
  [
    (simple_identifier) @name
    (hierarchical_identifier (simple_identifier) @name .)
  ]) @reference.call
