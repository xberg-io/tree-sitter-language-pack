; Dart tags, written from the UserNobody14/tree-sitter-dart grammar.

; Types.
(class_definition
  name: (identifier) @name) @definition.class

; A mixin is Dart's trait: `mixin Walker on Animal`. Its name has no field.
(mixin_declaration
  (identifier) @name) @definition.interface

(enum_declaration
  name: (identifier) @name) @definition.enum

(enum_constant
  name: (identifier) @name) @definition.constant

; Type aliases: `typedef Callback = void Function(int);`, `typedef Mapper<T> = ...`, and the
; legacy `typedef int Compare(a, b);`, where the alias name directly precedes the parameters.
(type_alias
  .
  (type_identifier) @name
  .
  (type_parameters)?
  .
  "=") @definition.type

(type_alias
  (type_identifier) @name
  .
  (formal_parameter_list)) @definition.type

; Constructors: the last identifier before the parameters is the constructor's own name, so
; `Point(...)` tags `Point` and `Point.polar(...)` tags `polar`.
(constructor_signature
  name: (identifier) @name
  .
  parameters: (formal_parameter_list)) @definition.constructor

(constant_constructor_signature
  name: (identifier) @name
  .
  (formal_parameter_list)) @definition.constructor

(factory_constructor_signature
  (identifier) @name
  .
  (formal_parameter_list)) @definition.constructor

(redirecting_factory_constructor_signature
  (identifier) @name
  .
  (formal_parameter_list)) @definition.constructor

; Methods, getters and setters in class, mixin, enum and extension bodies.
; ~keep: method_signature has no fields of its own — it wraps a function/getter/setter
; signature, and only the wrapped node carries `name`. `method_signature name:` is an
; impossible pattern that tree-sitter rejects at compile time.
(method_signature
  [
    (function_signature
      name: (identifier) @name)
    (getter_signature
      name: (identifier) @name)
    (setter_signature
      name: (identifier) @name)
  ]) @definition.method

; Abstract methods and getters: a bodiless signature declared in a class body.
(class_body
  (declaration
    [
      (function_signature
        name: (identifier) @name)
      (getter_signature
        name: (identifier) @name)
      (setter_signature
        name: (identifier) @name)
    ]) @definition.method)

; Top-level functions, getters and setters.
(function_signature
  name: (identifier) @name) @definition.function

(program
  [
    (getter_signature
      name: (identifier) @name)
    (setter_signature
      name: (identifier) @name)
  ] @definition.function)

; Class members: `static const origin = 0;` is a constant, other fields are fields.
(class_body
  (declaration
    (const_builtin)
    (static_final_declaration_list
      (static_final_declaration
        (identifier) @name))) @definition.constant)

(class_body
  (declaration
    [
      (static_final_declaration_list
        (static_final_declaration
          (identifier) @name))
      (initialized_identifier_list
        (initialized_identifier
          (identifier) @name))
    ]) @definition.field)

; Top-level declarations: `const` (optionally typed) is a constant; `final`, `var` and typed
; declarations are variables. The const patterns come first, so they claim the name.
(program
  (const_builtin)
  .
  (static_final_declaration_list
    (static_final_declaration
      (identifier) @name) @definition.constant))

(program
  (const_builtin)
  .
  (type_identifier)
  .
  (type_arguments)?
  .
  (static_final_declaration_list
    (static_final_declaration
      (identifier) @name) @definition.constant))

(program
  [
    (static_final_declaration_list
      (static_final_declaration
        (identifier) @name) @definition.variable)
    (initialized_identifier_list
      (initialized_identifier
        (identifier) @name) @definition.variable)
  ])

; Supertypes: `extends Base`, `with Walker`, `implements Shape`, `extension on String`.
(superclass
  .
  (type_identifier) @name) @reference.class

(mixins
  (type_identifier) @name) @reference.interface

(class_definition
  interfaces: (interfaces
    (type_identifier) @name)) @reference.implementation

(extension_declaration
  class: (type_identifier) @name) @reference.class

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
