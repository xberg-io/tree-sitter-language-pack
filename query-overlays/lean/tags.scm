; Commands at the top level or inside `namespace`/`section` are the declarations; `let`, `have`
; and `where` bindings are local. A `class` is parsed as a `structure` with the `class` keyword.

(namespace
  name: (identifier) @name) @definition.module

(declaration
  (structure
    "class"
    name: (identifier) @name) @definition.interface)

(declaration
  (structure
    name: (identifier) @name) @definition.class)

(structure
  (field
    name: (identifier) @name) @definition.field)

(declaration
  (inductive
    name: (identifier) @name) @definition.type)

(inductive
  (ctor_alt
    name: (identifier) @name) @definition.constructor)

(declaration
  [
    (def name: (identifier) @name)
    (abbrev name: (identifier) @name)
    (opaque name: (identifier) @name)
    (theorem name: (identifier) @name)
    (instance name: (identifier) @name)
  ] @definition.function)

(declaration
  [
    (axiom name: (identifier) @name)
    (constant name: (identifier) @name)
  ] @definition.constant)

; `instance : Class T where ...`: an implementation of the class
(declaration
  (instance
    type: [
      (app fn: (identifier) @name)
      (identifier) @name
    ]) @reference.implementation)
