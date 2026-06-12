; Class definitions
(class_definition name: (identifier) @name) @definition.class

; Trait definitions
(trait_definition name: (identifier) @name) @definition.interface

; Object definitions (singleton objects)
(object_definition name: (identifier) @name) @definition.module

; Function definitions (methods inside class bodies)
(function_definition name: (identifier) @name) @definition.method

; Class extends: `class Foo extends Bar`
; `extends_clause.type` can be a `generic_type` with a `stable_type_identifier` child,
; or a direct `type_identifier`.
(class_definition
  extend: (extends_clause
    type: (generic_type
      type: (stable_type_identifier
        (type_identifier) @name)))) @reference.implementation

(class_definition
  extend: (extends_clause
    type: (type_identifier) @name)) @reference.implementation

; Trait extends
(trait_definition
  extend: (extends_clause
    type: (type_identifier) @name)) @reference.implementation

; Object extends
(object_definition
  extend: (extends_clause
    type: (type_identifier) @name)) @reference.implementation

; Function calls
(call_expression function: (identifier) @name) @reference.call
