; TypeScript declarations and references. The vendored tags.scm only adds TypeScript-specific
; patterns on top of JavaScript's, which are not bundled with it, so this query covers the whole
; surface and follows the conventions of the JavaScript tags.scm.

; Functions: declarations, overload and ambient signatures, generators and named function
; expressions.
[
  (function_declaration name: (identifier) @name)
  (generator_function_declaration name: (identifier) @name)
  (function_signature name: (identifier) @name)
  (function_expression name: (identifier) @name)
  (generator_function name: (identifier) @name)
] @definition.function

; `const f = () => ...` and `var f = function () {...}`; ahead of the exported-constant pattern
; so a function-valued constant is tagged as a function.
(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    value: [(arrow_function) (function_expression) (generator_function)]) @definition.function)

(variable_declaration
  (variable_declarator
    name: (identifier) @name
    value: [(arrow_function) (function_expression) (generator_function)]) @definition.function)

; `obj.f = () => ...`
(assignment_expression
  left: [
    (identifier) @name
    (member_expression property: (property_identifier) @name)
  ]
  right: [(arrow_function) (function_expression)]) @definition.function

; `{ f: () => ... }`
(pair
  key: (property_identifier) @name
  value: [(arrow_function) (function_expression)]) @definition.function

; Classes, including abstract classes and named class expressions.
[
  (class_declaration name: (type_identifier) @name)
  (abstract_class_declaration name: (type_identifier) @name)
  (class name: (type_identifier) @name)
] @definition.class

; Methods and accessors; the constructor is the class itself.
((method_definition name: (property_identifier) @name) @definition.method
  (#not-eq? @name "constructor"))

[
  (method_signature name: (property_identifier) @name)
  (abstract_method_signature name: (property_identifier) @name)
] @definition.method

(interface_declaration name: (type_identifier) @name) @definition.interface

(type_alias_declaration name: (type_identifier) @name) @definition.type

(enum_declaration name: (identifier) @name) @definition.enum

; `namespace A {}`, `namespace A.B {}`, `module A {}` and `declare module "pkg" {}`; a quoted
; module is named by its unquoted text.
[
  (internal_module name: [(identifier) (nested_identifier)] @name)
  (module name: [(identifier) (nested_identifier)] @name)
  (internal_module name: (string (string_fragment) @name))
  (module name: (string (string_fragment) @name))
] @definition.module

; `export const NAME = value` that is not a function.
(export_statement
  declaration: (lexical_declaration
    kind: "const"
    (variable_declarator name: (identifier) @name) @definition.constant))

; `implements Shape`
(implements_clause (type_identifier) @name) @reference.implementation

; Type annotations: `x: Circle`.
(type_annotation (type_identifier) @name) @reference.type

; `new Circle()`
(new_expression constructor: (identifier) @name) @reference.class

; `f()`; `require()` is a module load, not a call of a project symbol.
((call_expression function: (identifier) @name) @reference.call
  (#not-eq? @name "require"))

; `obj.f()`
(call_expression
  function: (member_expression property: (property_identifier) @name)) @reference.call
