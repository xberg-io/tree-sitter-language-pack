; Mojo (`.mojo`). The grammar (HerringtonDarkholme/tree-sitter-mojo) extends tree-sitter-python
; with `struct` and `fn`. Tags structs (classes), the functions and methods in them and top-level
; `fn` and `def` functions, plus calls and `from ... import` modules.
;
; The grammar predates most of Mojo's declaration syntax and does not parse `alias`, `trait`,
; `var` and `let` declarations, struct fields, compile-time parameter lists (`struct Box[T: ...]`,
; `fn add[T: ...]`), trait conformance (`struct Circle(Shape)`), decorators on a struct, or argument
; conventions (`out self`, `inout self`, `owned s`), so those forms, and constructors that need
; `out self`, are not tagged. Mojo has no classes, so the Python `class` the grammar inherits is
; not tagged either.

; `fn norm(self) -> Float64:` and `@staticmethod fn origin()` in a struct body
(struct_definition
  body: (block
    [(function_definition
       name: (identifier) @name)
     (decorated_definition
       definition: (function_definition
         name: (identifier) @name))] @definition.method))

(struct_definition
  name: (identifier) @name) @definition.class

(function_definition
  name: (identifier) @name) @definition.function

; `from math import sqrt`
(import_from_statement
  module_name: (dotted_name
    (identifier) @name .)) @reference.module

(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call
