; ~keep This grammar parses Groovy as generic commands of whitespace-separated units, with no
; ~keep declaration nodes. A declaration is recognized by its keyword unit followed by the name
; ~keep unit, and a method by a block that opens with `name(params)` after its return type or
; ~keep `def`; a block that opens with `receiver.call(args)` (a call with a closure) does not.

; Packages
(command
  .
  (unit
    .
    (identifier) @_keyword .)
  .
  (unit) @name
  (#eq? @_keyword "package")) @definition.module

; Classes, interfaces, enums and traits: `class Name ...` or `class Name {`
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
  (#any-of? @_keyword "class" "interface" "enum" "trait")) @definition.class

; Methods: a definition in the body of a class
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

; Calls
(func
  .
  (identifier) @name) @reference.call
