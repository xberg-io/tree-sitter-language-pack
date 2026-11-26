; Class definitions
(class_definition name: (identifier) @name) @definition.class

; Method declarations inside class bodies
; ~keep: method_signature has no fields of its own — it wraps a function/getter/setter
; signature, and only the wrapped node carries `name`. `method_signature name:` is an
; impossible pattern that tree-sitter rejects at compile time.
(method_signature
  (function_signature name: (identifier) @name)) @definition.method
(method_signature
  (getter_signature name: (identifier) @name)) @definition.method
(method_signature
  (setter_signature name: (identifier) @name)) @definition.method
; Abstract methods: a bodiless signature declared in a class body
(class_body
  (declaration
    (function_signature name: (identifier) @name)) @definition.method)
(function_signature name: (identifier) @name) @definition.function

; Class superclass: `class Foo extends Bar`
(class_definition
  superclass: (superclass (type_identifier) @name)) @reference.implementation

; Class interfaces: `class Foo implements Bar, Baz`
(class_definition
  interfaces: (interfaces (type_identifier) @name)) @reference.implementation

; Calls: a callee followed directly by an argument list, `f(x)`, `obj.m(x)`, `obj?.m(x)`
; or a cascade `..m(x)`. Fields, parameters and annotations have no argument part.
((identifier) @name @reference.call
  .
  (selector
    (argument_part)))

((selector
  [
    (unconditional_assignable_selector
      (identifier) @name)
    (conditional_assignable_selector
      (identifier) @name)
  ]) @reference.call
  .
  (selector
    (argument_part)))

(cascade_section
  (cascade_selector
    (identifier) @name)
  .
  (argument_part)) @reference.call
