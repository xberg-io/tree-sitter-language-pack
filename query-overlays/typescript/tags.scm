; TypeScript declarations and references. The vendored tags.scm only adds TypeScript-specific
; patterns on top of JavaScript's, which are not bundled with it, so this query covers the whole
; surface and follows the conventions of the JavaScript tags.scm.

; Overloads: a run of signatures (and the implementation that follows them) is one function,
; tagged at its first signature. Each later member of the run is claimed without a tag; these
; patterns come first so they win over the definition patterns below.
((function_signature name: (identifier) @_first)
  .
  [
    (function_signature name: (identifier) @ignore)
    (function_declaration name: (identifier) @ignore)
  ]
  (#eq? @_first @ignore))

((export_statement declaration: (function_signature name: (identifier) @_first))
  .
  (export_statement
    declaration: [
      (function_signature name: (identifier) @ignore)
      (function_declaration name: (identifier) @ignore)
    ])
  (#eq? @_first @ignore))

((method_signature name: (property_identifier) @_first)
  .
  [
    (method_signature name: (property_identifier) @ignore)
    (method_definition name: (property_identifier) @ignore)
  ]
  (#eq? @_first @ignore))

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

; Constructors, ahead of the method patterns so they claim `constructor`.
(([
  (method_definition name: (property_identifier) @name)
  (method_signature name: (property_identifier) @name)
] @definition.constructor)
  (#eq? @name "constructor"))

; Methods and accessors; `#private` names keep the `#` every use spells.
(method_definition
  name: [(property_identifier) (private_property_identifier)] @name) @definition.method

[
  (method_signature name: (property_identifier) @name)
  (abstract_method_signature name: (property_identifier) @name)
] @definition.method

(interface_declaration name: (type_identifier) @name) @definition.interface

(type_alias_declaration name: (type_identifier) @name) @definition.type

(enum_declaration name: (identifier) @name) @definition.enum

; Enum members: `Red` and `Green = 2`.
[
  (enum_body name: (property_identifier) @name)
  (enum_assignment name: (property_identifier) @name)
] @definition.constant

; Class fields and interface properties (not members of inline object types).
(public_field_definition
  name: [(property_identifier) (private_property_identifier)] @name) @definition.field

(interface_body
  (property_signature name: (property_identifier) @name) @definition.field)

; `namespace A {}`, `namespace A.B {}`, `module A {}` and `declare module "pkg" {}`; a quoted
; module is named by its unquoted text.
[
  (internal_module name: [(identifier) (nested_identifier)] @name)
  (module name: [(identifier) (nested_identifier)] @name)
  (internal_module name: (string (string_fragment) @name))
  (module name: (string (string_fragment) @name))
] @definition.module

; Module-level bindings that are not functions: `const` is a constant, `let` and `var` are
; variables. Exported bindings count anywhere (also inside a namespace); a bare one only at the
; top level, so locals stay untagged. Destructured names are tagged one by one.
(export_statement
  declaration: (lexical_declaration
    kind: "const"
    (variable_declarator name: (identifier) @name) @definition.constant))

(program
  (lexical_declaration
    kind: "const"
    (variable_declarator
      name: [
        (identifier) @name
        (object_pattern (shorthand_property_identifier_pattern) @name)
        (array_pattern (identifier) @name)
      ]) @definition.constant))

(export_statement
  declaration: [
    (lexical_declaration
      kind: "let"
      (variable_declarator name: (identifier) @name) @definition.variable)
    (variable_declaration
      (variable_declarator name: (identifier) @name) @definition.variable)
  ])

(program
  [
    (lexical_declaration
      kind: "let"
      (variable_declarator
        name: [
          (identifier) @name
          (object_pattern (shorthand_property_identifier_pattern) @name)
          (array_pattern (identifier) @name)
        ]) @definition.variable)
    (variable_declaration
      (variable_declarator
        name: [
          (identifier) @name
          (object_pattern (shorthand_property_identifier_pattern) @name)
          (array_pattern (identifier) @name)
        ]) @definition.variable)
  ])

; `extends Base`, `extends Base<T>`, `extends ns.Base`.
(extends_clause
  value: [
    (identifier) @name
    (member_expression property: (property_identifier) @name)
  ]) @reference.class

; `implements Shape`, `implements Comparable<T>`, `implements ns.Shape`.
(implements_clause
  [
    (type_identifier) @name
    (generic_type name: (type_identifier) @name)
    (nested_type_identifier name: (type_identifier) @name)
  ]) @reference.implementation

; `interface Solid extends Shape, Comparable<T>`.
(extends_type_clause
  type: [
    (type_identifier) @name
    (generic_type name: (type_identifier) @name)
    (nested_type_identifier name: (type_identifier) @name)
  ]) @reference.interface

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
