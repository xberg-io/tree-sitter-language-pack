; Declaration kinds follow query-overlays/hcl/tags.scm: resources are classes, inputs and
; variables are variables, outputs are constants.
(parameter_declaration
  . (identifier) @name) @definition.variable

(variable_declaration
  . (identifier) @name) @definition.variable

(type_declaration
  . (identifier) @name) @definition.type

(user_defined_function
  name: (identifier) @name) @definition.function

(resource_declaration
  . (identifier) @name) @definition.class

(module_declaration
  . (identifier) @name) @definition.module

(output_declaration
  . (identifier) @name) @definition.constant

(call_expression
  function: (identifier) @name) @reference.call
