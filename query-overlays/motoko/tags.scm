; Motoko (`.mo`). Modules, actors, objects, classes, functions, types, and the `let`/`var`
; fields of a program, module, actor, object or class body. Bindings inside function bodies are
; local. Functions in an actor, object or class body are methods; in a module, functions.

(dec_obj
  sort: (obj_sort) @_sort
  object_name: (name) @name
  (#eq? @_sort "module")) @definition.module

(dec_obj
  sort: (obj_sort) @_sort
  object_name: (name) @name
  (#not-eq? @_sort "module")) @definition.class

(dec_class
  class_name: (name) @name) @definition.class

(dec_obj
  sort: (obj_sort) @_sort
  (dec_field
    (dec_func
      function_name: (name) @name) @definition.method)
  (#not-eq? @_sort "module"))

(dec_class
  (dec_field
    (dec_func
      function_name: (name) @name) @definition.method))

(dec_func
  function_name: (name) @name) @definition.function

(dec_type
  type_name: (name) @name) @definition.type

; `let origin = ...` and `let origin : Point = ...`
[
  (prog
    (dec_let
      binding_pattern: (pat
        [
          (variable variable_name: (name) @name)
          (pat_bin (variable variable_name: (name) @name))
        ])) @definition.constant)
  (dec_field
    (dec_let
      binding_pattern: (pat
        [
          (variable variable_name: (name) @name)
          (pat_bin (variable variable_name: (name) @name))
        ])) @definition.constant)
]

[
  (prog (dec_var variable_name: (name) @name) @definition.variable)
  (dec_field (dec_var variable_name: (name) @name) @definition.variable)
]

; `greet(name)` and `Debug.print(name)`
(call_exp
  invoked_value: (variable
    variable_name: (name) @name)) @reference.call

(call_exp
  invoked_value: (exp_post
    (member
      key: (name) @name)
    .)) @reference.call

(typ_id
  type_name: (name) @name) @reference.type
