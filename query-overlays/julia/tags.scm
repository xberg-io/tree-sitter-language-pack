; Modules
(module_definition
  name: (identifier) @name) @definition.module

; Functions: the callee of the signature, unwrapped from a return type annotation and a
; `where` clause; `Base.show(...)` names `show`
(function_definition
  (signature
    [
      (call_expression
        .
        [
          (identifier) @name
          (field_expression
            (identifier) @name .)
        ])
      (typed_expression
        .
        (call_expression
          .
          [
            (identifier) @name
            (field_expression
              (identifier) @name .)
          ]))
      (where_expression
        .
        [
          (call_expression
            .
            [
              (identifier) @name
              (field_expression
                (identifier) @name .)
            ])
          (typed_expression
            .
            (call_expression
              .
              [
                (identifier) @name
                (field_expression
                  (identifier) @name .)
              ]))
        ])
    ])) @definition.function

; Short-form functions: `f(x) = ...`
(assignment
  .
  (call_expression
    .
    (identifier) @name)
  .
  (operator)) @definition.function

; Macros
(macro_definition
  (signature
    (call_expression
      .
      (identifier) @name))) @definition.macro

; Types: structs, abstract and primitive types, named with or without type parameters and
; a supertype
(struct_definition
  (type_head
    [
      (identifier) @name
      (parametrized_type_expression
        .
        (identifier) @name)
      (binary_expression
        .
        [
          (identifier) @name
          (parametrized_type_expression
            .
            (identifier) @name)
        ])
    ])) @definition.class

(abstract_definition
  (type_head
    [
      (identifier) @name
      (parametrized_type_expression
        .
        (identifier) @name)
      (binary_expression
        .
        [
          (identifier) @name
          (parametrized_type_expression
            .
            (identifier) @name)
        ])
    ])) @definition.type

(primitive_definition
  (type_head
    [
      (identifier) @name
      (binary_expression
        .
        (identifier) @name)
    ])) @definition.type

; Supertypes: `<: Parent`
(type_head
  (binary_expression
    (operator)
    .
    [
      (identifier) @name
      (parametrized_type_expression
        .
        (identifier) @name)
    ])) @reference.class

; Constants
(const_statement
  (assignment
    .
    (identifier) @name)) @definition.constant

; Calls and macro calls
(call_expression
  .
  [
    (identifier) @name
    (field_expression
      (identifier) @name .)
  ]) @reference.call

(macrocall_expression
  (macro_identifier
    (identifier) @name)) @reference.call
