; JavaScript tags: the vendored tags.scm of grammar-orchard/tree-sitter-javascript-orchard,
; plus constructors, class fields, module-level bindings and superclass references.

(
  (comment)* @doc
  .
  (method_definition
    name: (property_identifier) @name) @definition.method
  (#not-eq? @name "constructor")
  (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
  (#select-adjacent! @doc @definition.method)
)

; Constructors: the vendored method pattern above skips `constructor`.
((method_definition
  name: (property_identifier) @name) @definition.constructor
  (#eq? @name "constructor"))

; `#private` methods keep the `#` every use spells.
(method_definition
  name: (private_property_identifier) @name) @definition.method

(
  (comment)* @doc
  .
  [
    (class
      name: (_) @name)
    (class_declaration
      name: (_) @name)
  ] @definition.class
  (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
  (#select-adjacent! @doc @definition.class)
)

(
  (comment)* @doc
  .
  [
    (function_expression
      name: (identifier) @name)
    (function_declaration
      name: (identifier) @name)
    (generator_function
      name: (identifier) @name)
    (generator_function_declaration
      name: (identifier) @name)
  ] @definition.function
  (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
  (#select-adjacent! @doc @definition.function)
)

(
  (comment)* @doc
  .
  (lexical_declaration
    (variable_declarator
      name: (identifier) @name
      value: [(arrow_function) (function_expression)]) @definition.function)
  (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
  (#select-adjacent! @doc @definition.function)
)

(
  (comment)* @doc
  .
  (variable_declaration
    (variable_declarator
      name: (identifier) @name
      value: [(arrow_function) (function_expression)]) @definition.function)
  (#strip! @doc "^[\\s\\*/]+|^[\\s\\*/]$")
  (#select-adjacent! @doc @definition.function)
)

(assignment_expression
  left: [
    (identifier) @name
    (member_expression
      property: (property_identifier) @name)
  ]
  right: [(arrow_function) (function_expression)]
) @definition.function

(pair
  key: (property_identifier) @name
  value: [(arrow_function) (function_expression)]) @definition.function

; Class fields: `radius = 1`, `static count`, `#secret = 2`.
(field_definition
  property: [
    (property_identifier)
    (private_property_identifier)
  ] @name) @definition.field

; Module-level bindings that are not functions (the function patterns above claim those):
; `const` is a constant, `let` and `var` are variables. Exported bindings count anywhere, bare
; ones only at the top level, so locals stay untagged. Destructured names are tagged one by one.
(export_statement
  declaration: (lexical_declaration
    kind: "const"
    (variable_declarator
      name: (identifier) @name) @definition.constant))

(program
  (lexical_declaration
    kind: "const"
    (variable_declarator
      name: [
        (identifier) @name
        (object_pattern
          (shorthand_property_identifier_pattern) @name)
        (array_pattern
          (identifier) @name)
      ]) @definition.constant))

(export_statement
  declaration: [
    (lexical_declaration
      kind: "let"
      (variable_declarator
        name: (identifier) @name) @definition.variable)
    (variable_declaration
      (variable_declarator
        name: (identifier) @name) @definition.variable)
  ])

(program
  [
    (lexical_declaration
      kind: "let"
      (variable_declarator
        name: [
          (identifier) @name
          (object_pattern
            (shorthand_property_identifier_pattern) @name)
          (array_pattern
            (identifier) @name)
        ]) @definition.variable)
    (variable_declaration
      (variable_declarator
        name: [
          (identifier) @name
          (object_pattern
            (shorthand_property_identifier_pattern) @name)
          (array_pattern
            (identifier) @name)
        ]) @definition.variable)
  ])

; Superclasses: `class Circle extends Shape`, `extends geometry.Shape`.
(class_heritage
  [
    (identifier) @name
    (member_expression
      property: (property_identifier) @name)
  ]) @reference.class

(
  (call_expression
    function: (identifier) @name) @reference.call
  (#not-match? @name "^(require)$")
)

(call_expression
  function: (member_expression
    property: (property_identifier) @name)
  arguments: (_) @reference.call)

; `new Greeter()`, `new lib.Greeter()`: the vendored `constructor: (_) @name` named the whole
; member expression.
(new_expression
  constructor: [
    (identifier) @name
    (member_expression
      property: (property_identifier) @name)
  ]) @reference.class

; The vendored `(export_statement value: (assignment_expression ...)) @definition.constant`
; pattern is dropped: it only matches `export default x = value`, which reassigns a binding
; already declared (and tagged) elsewhere, or throws in module code when `x` is undeclared.
