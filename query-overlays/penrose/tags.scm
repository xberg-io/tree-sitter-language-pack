; `.domain` files declare types, predicates, functions and constructors.
(type_decl
  names: (identifier_list (identifier) @name)) @definition.type

(subtype_decl
  subtype: (identifier) @name) @definition.type

(predicate_decl
  name: (identifier) @name) @definition.function

(function_decl
  name: (identifier) @name) @definition.function

(constructor_decl
  name: (identifier) @name) @definition.function

(value_decl
  name: (_) @name) @definition.variable

; `.substance` files declare objects (`Set A, B`) and bind constructed ones (`Let C := ...`).
(substance_type_decl
  names: (identifier_list (identifier) @name)) @definition.variable

(let_stmt
  target: (_) @name) @definition.variable

(subtype_decl
  supertype: (identifier) @name) @reference.type

(parameter
  type: (identifier) @name) @reference.type

(_
  result_type: (identifier) @name) @reference.type

(substance_type_decl
  type: (identifier) @name) @reference.type

(predicate_stmt
  name: (identifier) @name) @reference.call

(call_expression
  function: (identifier) @name) @reference.call
