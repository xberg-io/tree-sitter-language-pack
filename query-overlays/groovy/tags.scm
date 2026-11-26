; Groovy, written from tree-sitter-groovy (github.com/Decodetalkers/tree-sitter-groovy).
; ~keep This grammar parses Groovy as generic commands of whitespace-separated units, with no
; ~keep declaration nodes. A declaration is recognized by its keyword unit followed by the name
; ~keep unit (`class Name ...`, or `class Name {` where the name opens the block), a method by a
; ~keep block that opens with `name(params)` after its return type or `def`, and a field by a
; ~keep `Type name` command directly in a class body. A block that opens with
; ~keep `receiver.call(args)` (a call with a closure) is not a method. Overloads cannot be told
; ~keep apart from the units alone, so each tags.

; Packages
(command
  .
  (unit
    .
    (identifier) @_keyword .)
  .
  (unit) @name
  (#eq? @_keyword "package")) @definition.module

; `class Name`, `interface Name`, `trait Name` and `enum Name`
(command
  (unit
    .
    (identifier) @_keyword .)
  .
  [
    (unit
      .
      (identifier) @name .)
    (block
      .
      (unit
        .
        (identifier) @name .))
  ]
  (#eq? @_keyword "class")) @definition.class

(command
  (unit
    .
    (identifier) @_keyword .)
  .
  [
    (unit
      .
      (identifier) @name .)
    (block
      .
      (unit
        .
        (identifier) @name .))
  ]
  (#any-of? @_keyword "interface" "trait")) @definition.interface

(command
  (unit
    .
    (identifier) @_keyword .)
  .
  [
    (unit
      .
      (identifier) @name .)
    (block
      .
      (unit
        .
        (identifier) @name .))
  ]
  (#eq? @_keyword "enum")) @definition.enum

; Enum constants come first in an enum body: `HAPPY, SAD` or `HAPPY('x'), SAD('y')`.
(command
  (unit
    .
    (identifier) @_keyword .)
  .
  [
    (block
      .
      (unit)
      .
      (command
        [
          (unit
            .
            (identifier) @name .)
          (unit
            .
            (func
              .
              (identifier) @name))
        ] @definition.constant))
    (block
      .
      (command
        [
          (unit
            .
            (identifier) @name .)
          (unit
            .
            (func
              .
              (identifier) @name))
        ] @definition.constant))
  ]
  (#eq? @_keyword "enum"))

; `class Greeter implements Speaker, Polite` and `class Circle extends Shape`
(command
  (unit
    .
    (identifier) @_declaration .)
  (unit
    .
    (identifier) @_relation .)
  (unit
    .
    (identifier) @name .) @reference.implementation
  (#any-of? @_declaration "class" "interface" "trait" "enum")
  (#any-of? @_relation "extends" "implements")
  (#not-any-of? @name "extends" "implements"))

; The last of several supertypes opens the body block: `implements Speaker, Polite {`.
(command
  (unit
    .
    (identifier) @_declaration .)
  (unit
    .
    (identifier) @_relation .)
  (arg_splitter)
  .
  (block
    .
    (unit
      .
      (identifier) @name .) @reference.implementation)
  (#any-of? @_declaration "class" "interface" "trait" "enum")
  (#any-of? @_relation "extends" "implements"))

; Constructors: `Greeter(String name) {` in the body of class Greeter, with no return type.
(command
  (unit
    .
    (identifier) @_keyword .)
  [
    (unit
      .
      (identifier) @_class .)
    (block
      .
      (unit
        .
        (identifier) @_class .))
  ]
  (block
    (command
      .
      (block
        .
        (unit
          .
          (func
            .
            (identifier) @name))) @definition.constructor))
  (#any-of? @_keyword "class" "enum")
  (#eq? @_class @name))

; Abstract methods in an interface: `String speak(String text)`, with no body.
(command
  (unit
    .
    (identifier) @_keyword .)
  (block
    (command
      (unit)
      .
      (unit
        .
        (func
          .
          (identifier) @name)) .) @definition.method)
  (#eq? @_keyword "interface"))

; Methods: a definition in the body of a class, interface, trait or enum
(command
  (unit
    .
    (identifier) @_keyword .)
  (block
    (command
      (unit)
      .
      (block
        .
        (unit
          .
          (func
            .
            (identifier) @name))) @definition.method))
  (#any-of? @_keyword "class" "interface" "enum" "trait"))

; Fields: `static final String DEFAULT = 'Hello'` is a constant; `String name` and
; `int count = 0` are fields.
(command
  (unit
    .
    (identifier) @_keyword .)
  (block
    (command
      (unit
        .
        (identifier) @_modifier .)
      (unit
        .
        (identifier) @name .)
      .
      (operators) @_operator) @definition.constant)
  (#eq? @_keyword "class")
  (#eq? @_modifier "final")
  (#eq? @_operator "="))

(command
  (unit
    .
    (identifier) @_keyword .)
  (block
    [
      (command
        .
        (unit
          .
          (identifier) .)
        .
        (unit
          .
          (identifier) @name .) .)
      (command
        (unit
          .
          (identifier) .)
        .
        (unit
          .
          (identifier) @name .)
        .
        (operators) @_operator
        (#eq? @_operator "="))
    ] @definition.field)
  (#any-of? @_keyword "class" "trait"))

; Functions: `def name(params) {` or `Type name(params) {` elsewhere
(command
  (unit)
  .
  (block
    .
    (unit
      .
      (func
        .
        (identifier) @name))) @definition.function)

; Script variables: `def name = ...` at the top level
(source_file
  (command
    .
    (unit
      .
      (identifier) @_keyword .)
    .
    (unit
      .
      (identifier) @name .)
    .
    (operators) @_operator
    (#eq? @_keyword "def")
    (#eq? @_operator "=")) @definition.variable)

; `new Greeter('Groovy')`
(_
  (unit
    .
    (identifier) @_new .)
  .
  (unit
    .
    (func
      .
      (identifier) @name)) @reference.class
  (#eq? @_new "new"))

; Calls
(func
  .
  (identifier) @name) @reference.call
