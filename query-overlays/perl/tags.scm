; Perl tags, written from the tree-sitter-perl/tree-sitter-perl grammar.

; Packages and classes
(package_statement
  name: (package) @name) @definition.module

(class_statement
  name: (package) @name) @definition.class

; Subroutines and methods
(subroutine_declaration_statement
  name: (bareword) @name) @definition.function

(method_declaration_statement
  name: (bareword) @name) @definition.method

; Fields of a `class` block (feature 'class'): `field $price :param = 0;`. Fields keep their
; sigil, which every use spells.
(class_statement
  (block
    (expression_statement
      [
        (variable_declaration
          "field"
          variable: (_) @name)
        (assignment_expression
          left: (variable_declaration
            "field"
            variable: (_) @name))
      ]) @definition.field))

; Constants: `use constant NAME => value;` and `use constant { RED => 1, BLUE => 2 };` (only the
; keys of the hash are barewords).
(use_statement
  module: (package) @_module
  (list_expression
    .
    (autoquoted_bareword) @name)
  (#eq? @_module "constant")) @definition.constant

(use_statement
  module: (package) @_module
  (anonymous_hash_expression
    (list_expression
      (autoquoted_bareword) @name))
  (#eq? @_module "constant")) @definition.constant

; Package variables declared at file scope: `our $VERSION = ...;`, `our ($A, %B);`.
(source_file
  (expression_statement
    [
      (variable_declaration
        "our"
        [
          variable: (_) @name
          variables: (_) @name
        ])
      (assignment_expression
        left: (variable_declaration
          "our"
          [
            variable: (_) @name
            variables: (_) @name
          ]))
    ]) @definition.variable)

; Superclasses: `use parent -norequire, 'Shop::Base';`, `our @ISA = ('Shop::Base');` and
; `class Shop::Item :isa(Shop::Base)`.
(use_statement
  module: (package) @_module
  [
    (string_literal
      content: (string_content) @name)
    (list_expression
      (string_literal
        content: (string_content) @name))
  ]
  (#any-of? @_module "parent" "base")) @reference.class

(assignment_expression
  left: (variable_declaration
    variable: (array
      (varname) @_isa))
  right: (parenthesized_expression
    [
      (string_literal
        content: (string_content) @name)
      (list_expression
        (string_literal
          content: (string_content) @name))
    ])
  (#eq? @_isa "ISA")) @reference.class

(class_statement
  attributes: (attrlist
    (attribute
      name: (attribute_name) @_attribute
      value: (attribute_value) @name))
  (#eq? @_attribute "isa")) @reference.class

; Imported modules
(use_statement
  module: (package) @name) @reference.module

; Calls: `f(...)`, `f ...` and `$obj->m(...)`
(function_call_expression
  function: (function) @name) @reference.call

(ambiguous_function_call_expression
  function: (function) @name) @reference.call

(method_call_expression
  method: (method) @name) @reference.call
