; The grammar's lexer folds the whitespace after some keywords (`module`, `actor`, `type`, `var`,
; `let`, `import`) into the following name token, so those names can carry leading spaces.
(dec_obj
  sort: (obj_sort) @_sort
  object_name: (name) @name
  (#eq? @_sort "module")) @definition.module

; Actors and objects.
(dec_obj
  object_name: (name) @name) @definition.class

(dec_class
  class_name: (name) @name) @definition.class

(dec_type
  type_name: (name) @name) @definition.type

; Functions of an actor, object or class are methods; module functions are functions.
(dec_obj
  sort: (obj_sort) @_sort
  (dec_field
    (dec_func function_name: (name) @name) @definition.method)
  (#not-eq? @_sort "module"))

(dec_class
  (dec_field
    (dec_func function_name: (name) @name) @definition.method))

(dec_func
  function_name: (name) @name) @definition.function

(dec_field
  (dec_var variable_name: (name) @name) @definition.variable)

(dec_field
  (dec_let
    binding_pattern: (pat (variable variable_name: (name) @name))) @definition.variable)

(call_exp
  invoked_value: [
    (variable variable_name: (name) @name)
    (exp_post (member key: (name) @name))
  ]) @reference.call
