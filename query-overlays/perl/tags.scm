; Packages and classes
(package_statement
  name: (package) @name) @definition.module

(class_statement
  name: (package) @name) @definition.class

; Subroutines and methods
(subroutine_declaration_statement
  name: (bareword) @name) @definition.function

(method_declaration_statement
  name: (bareword) @name) @definition.method

; Constants: `use constant NAME => value;`
(use_statement
  module: (package) @_module
  (list_expression
    .
    (autoquoted_bareword) @name)
  (#eq? @_module "constant")) @definition.constant

; Package variables declared at file scope: `our $VERSION = ...;`
(source_file
  (expression_statement
    (assignment_expression
      left: (variable_declaration
        "our"
        variable: (_) @name))) @definition.variable)

; Imported modules
(use_statement
  module: (package) @name) @reference.module

; Calls: `f(...)`, `f ...` and `$obj->m(...)`
(function_call_expression
  function: (function) @name) @reference.call

(ambiguous_function_call_expression
  function: (function) @name) @reference.call

(method_call_expression
  method: (method) @name) @reference.call
