; Struct / class definitions
(class_specifier name: (type_identifier) @name) @definition.class

; Function / method definitions
(function_definition declarator: (function_declarator declarator: (identifier) @name)) @definition.function

; Class inheritance: `class Foo : public Bar`
; `base_class_clause` is a child of `class_specifier` and contains `type_identifier`.
(class_specifier
  (base_class_clause (type_identifier) @name)) @reference.implementation

; Function calls
(call_expression function: (identifier) @name) @reference.call
(call_expression function: (field_expression field: (field_identifier) @name)) @reference.call
