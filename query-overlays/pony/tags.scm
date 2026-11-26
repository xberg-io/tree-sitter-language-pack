; Types. ~keep A definition's only direct (identifier) child is its name.
(class_definition
  (identifier) @name) @definition.class

(actor_definition
  (identifier) @name) @definition.class

(primitive_definition
  (identifier) @name) @definition.class

(struct_definition
  (identifier) @name) @definition.class

(trait_definition
  (identifier) @name) @definition.interface

(interface_definition
  (identifier) @name) @definition.interface

(type_alias
  (identifier) @name) @definition.type

; Constructors, methods and behaviours
(constructor
  (identifier) @name) @definition.constructor

(method
  (identifier) @name) @definition.method

(behavior
  (identifier) @name) @definition.method

; Fields
(field
  name: (identifier) @name) @definition.field

; Provided traits and interfaces: `is Trait` or `is (A & B)`
[
  (class_definition
    [
      (base_type
        name: (identifier) @name)
      (intersection_type
        (base_type
          name: (identifier) @name))
    ])
  (actor_definition
    [
      (base_type
        name: (identifier) @name)
      (intersection_type
        (base_type
          name: (identifier) @name))
    ])
  (primitive_definition
    [
      (base_type
        name: (identifier) @name)
      (intersection_type
        (base_type
          name: (identifier) @name))
    ])
  (struct_definition
    [
      (base_type
        name: (identifier) @name)
      (intersection_type
        (base_type
          name: (identifier) @name))
    ])
  (trait_definition
    [
      (base_type
        name: (identifier) @name)
      (intersection_type
        (base_type
          name: (identifier) @name))
    ])
  (interface_definition
    [
      (base_type
        name: (identifier) @name)
      (intersection_type
        (base_type
          name: (identifier) @name))
    ])
] @reference.implementation

; Packages
(use_statement
  (string
    (string_content) @name)) @reference.module

; Calls: `f()`, `f[T]()`, `obj.m()` and `obj.m[T]()`
(call_expression
  callee: [
    (identifier) @name
    (generic_expression
      .
      (identifier) @name)
    (member_expression
      (identifier) @name .)
    (member_expression
      (generic_expression
        .
        (identifier) @name) .)
  ]) @reference.call
