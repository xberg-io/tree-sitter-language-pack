; PHP tags, written from the tree-sitter/tree-sitter-php grammar (php dialect).

; Namespaces: `namespace Geo\Shapes;` names the whole qualified path.
(namespace_definition
  name: (namespace_name) @name) @definition.module

; Types.
(class_declaration
  name: (name) @name) @definition.class

(interface_declaration
  name: (name) @name) @definition.interface

(trait_declaration
  name: (name) @name) @definition.interface

(enum_declaration
  name: (name) @name) @definition.enum

(enum_case
  name: (name) @name) @definition.constant

; Constructors, listed before the method pattern so it claims `__construct`.
(method_declaration
  name: (name) @name
  (#eq? @name "__construct")) @definition.constructor

(method_declaration
  name: (name) @name) @definition.method

(function_definition
  name: (name) @name) @definition.function

; Properties, declared or promoted from constructor parameters. The name drops the `$`: every
; use site (`$this->radius`) spells it bare.
(property_declaration
  (property_element
    name: (variable_name
      (name) @name))) @definition.field

(property_promotion_parameter
  name: (variable_name
    (name) @name)) @definition.field

; Constants: top-level `const LIMIT = 10;`, class constants, and `define('NAME', value)`.
(const_declaration
  (const_element
    (name) @name)) @definition.constant

(function_call_expression
  function: (name) @_define
  arguments: (arguments
    .
    (argument
      (string
        (string_content) @name)))
  (#eq? @_define "define")) @definition.constant

; Inheritance: `class Circle extends Base`.
(class_declaration
  (base_clause
    [
      (name) @name
      (qualified_name
        (name) @name)
    ])) @reference.class

; Interface implementation: `class Circle implements Shape, Other`.
(class_declaration
  (class_interface_clause
    [
      (name) @name
      (qualified_name
        (name) @name)
    ])) @reference.implementation

; Interface extension: `interface Solid extends Shape`.
(interface_declaration
  (base_clause
    [
      (name) @name
      (qualified_name
        (name) @name)
    ])) @reference.interface

; Trait use inside a class body: `use Named;`.
(use_declaration
  [
    (name) @name
    (qualified_name
      (name) @name)
  ]) @reference.interface

; Instantiation: `new Circle(1)`; `new static` / `new self` / `new parent` name no class.
(object_creation_expression
  [
    (name) @name
    (qualified_name
      (name) @name)
  ]
  (#not-match? @name "^(static|self|parent)$")) @reference.class

; Calls: functions, methods (also `?->`) and static methods.
(function_call_expression
  function: [
    (name) @name
    (qualified_name
      (name) @name)
  ]) @reference.call

(member_call_expression
  name: (name) @name) @reference.call

(nullsafe_member_call_expression
  name: (name) @name) @reference.call

(scoped_call_expression
  name: (name) @name) @reference.call
