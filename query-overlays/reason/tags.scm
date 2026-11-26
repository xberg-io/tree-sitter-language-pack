; Declarations at the top level of a file or of a module body: modules, types with their
; variant constructors and record fields, `let` bindings (a function when bound to a function
; expression), `external` bindings and exceptions. Bindings inside function bodies are local.

(module_declaration
  name: (module_identifier) @name) @definition.module

(type_declaration
  .
  (type_identifier) @name) @definition.type

(variant_declaration
  (variant_identifier) @name) @definition.constructor

(record_type
  (record_type_field
    (property_identifier) @name) @definition.field)

(exception_declaration
  (variant_identifier) @name) @definition.constructor

(external_declaration
  (value_identifier) @name) @definition.function

[
  (source_file (let_binding . (value_identifier) @name (function)) @definition.function)
  (module_declaration
    definition: (block (let_binding . (value_identifier) @name (function)) @definition.function))
]

[
  (source_file (let_binding . (value_identifier) @name) @definition.variable)
  (module_declaration
    definition: (block (let_binding . (value_identifier) @name) @definition.variable))
]
