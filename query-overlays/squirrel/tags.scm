; ~keep This grammar has no fields: a declaration's name is its first (identifier) child, and
; ~keep a class's second one is the base class after `extends`.

; Classes and inheritance
(class_declaration
  .
  (identifier) @name) @definition.class

(class_declaration
  .
  (identifier)
  .
  (identifier) @name) @reference.class

; Methods, constructors and fields of a class
(member_declaration
  (function_declaration
    .
    (identifier) @name) @definition.method)

(member_declaration
  "constructor" @name) @definition.method

(member_declaration
  .
  (identifier) @name) @definition.field

; Functions
(function_declaration
  .
  (identifier) @name) @definition.function

; Constants, enums and script-level locals
(const_declaration
  .
  (identifier) @name) @definition.constant

(enum_declaration
  .
  (identifier) @name) @definition.enum

(script
  (local_declaration
    .
    (identifier) @name) @definition.variable)

; Calls: `f()` and `obj.m()`
(call_expression
  function: (identifier) @name) @reference.call

(call_expression
  function: (deref_expression
    (identifier) @name .)) @reference.call
