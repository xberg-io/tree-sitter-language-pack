; Module-level declarations. The grammar gives each equation of a function its own `function`
; node, so a function is tagged once, at its first equation: an equation that directly follows
; one with the same name continues that function and is claimed without a tag. Type signatures
; are not tagged; the equations they describe are. Bindings in `where` and `let` are local.

(header
  module: (module) @name) @definition.module

(declarations
  [
    (data_type name: (name) @name)
    (newtype name: (name) @name)
    (type_synonym name: (name) @name)
    (type_family name: (name) @name)
  ] @definition.type)

(data_constructor
  constructor: [
    (prefix name: (constructor) @name)
    (record name: (constructor) @name)
  ]) @definition.constructor

(newtype_constructor
  name: (constructor) @name) @definition.constructor

(gadt_constructor
  name: (constructor) @name) @definition.constructor

; Record fields of a data constructor
(data_constructor
  constructor: (record
    fields: (fields
      (field
        name: (field_name (variable) @name)) @definition.field)))

; `class` and the method signatures it declares
(declarations
  (class
    name: (name) @name) @definition.interface)

(class
  declarations: (class_declarations
    (signature
      name: (variable) @name) @definition.method))

; `instance Class Type`: an implementation of the class
(declarations
  (instance
    name: (name) @name) @reference.implementation)

(declarations
  (function name: (variable) @_previous)
  .
  (function name: (variable) @ignore)
  (#eq? @_previous @ignore))

(declarations
  [
    (function name: (variable) @name)
    (bind name: (variable) @name)
  ] @definition.function)
