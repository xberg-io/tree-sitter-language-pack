; V (`.v`). The module and its top-level declarations: functions, methods, structs, enums,
; interfaces, type aliases and sum types, constants and globals. Variables declared inside a
; function body are locals.

; `module main`
(module_clause
  (module_identifier) @name) @definition.module

; `fn (p Point) sum() int { ... }`: a receiver makes a method.
(function_declaration
  receiver: (_)
  name: (identifier) @name) @definition.method

(function_declaration
  name: (identifier) @name) @definition.function

(struct_declaration
  name: [(type_identifier) @name
         (generic_type . (type_identifier) @name)]) @definition.struct

(enum_declaration
  name: (type_identifier) @name) @definition.enum

(interface_declaration
  name: [(type_identifier) @name
         (generic_type . (type_identifier) @name)]) @definition.interface

; Interface methods: `area() f64` inside `interface Shape`
(interface_spec
  name: (field_identifier) @name) @definition.method

; `type Score = int` and `type Shape = Circle | Square`
(type_declaration
  name: (type_identifier) @name) @definition.type

; `const max_sides = 8`
(const_spec
  name: (identifier) @name) @definition.constant

; `__global counter = 0`
[(global_var_spec name: (identifier) @name)
 (global_var_type_initializer name: (identifier) @name)] @definition.variable

; `add(1, 2)` and `p.sum()`
(call_expression
  function: [(identifier) @name
             (selector_expression field: (identifier) @name)]) @reference.call
