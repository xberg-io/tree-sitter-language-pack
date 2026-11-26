; Module-level declarations. Each equation of a function is its own `function` node, so a
; function is tagged once, at its first equation: an equation that directly follows one with the
; same name continues that function and is claimed without a tag. Type signatures are not
; tagged; the equations they describe are. Bindings in `where` and `let` are local.

(purescript
  name: (qualified_module) @name) @definition.module

(purescript
  [
    (data name: (type) @name)
    (newtype name: (type) @name)
    (type_alias name: (type) @name)
  ] @definition.type)

(data
  (constructor) @name) @definition.constructor

(newtype_constructor
  (constructor) @name) @definition.constructor

; `class` and the method signatures it declares
(purescript
  (class_declaration
    (class_head
      (class_name) @name)) @definition.interface)

(class_body
  (signature
    name: (variable) @name) @definition.method)

; `instance name :: Class Type where ...`: an implementation of the class
(purescript
  (class_instance
    (instance_head
      (class_name) @name)) @reference.implementation)

(purescript
  (function name: (variable) @_previous)
  .
  (function name: (variable) @ignore)
  (#eq? @_previous @ignore))

(purescript
  (function
    name: (variable) @name) @definition.function)

(purescript
  (foreign_import
    name: (variable) @name) @definition.function)
