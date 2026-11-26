; Hare (`.ha`). Module-level declarations: functions, types (struct and union are classes) with
; their fields and enum values, constants and globals. Bindings made
; with `let` or `const` inside a function body are locals.

(function_declaration
  name: (identifier) @name) @definition.function

; `type point = struct { ... };` by what the type is
(type_declaration
  (identifier) @name
  .
  (struct_type)) @definition.class

(type_declaration
  (identifier) @name
  .
  (enum_type)) @definition.enum

(type_declaration
  (identifier) @name
  .
  (union_type)) @definition.class

(type_declaration
  (identifier) @name) @definition.type

; `x: int,` in a struct or union
[(struct_type (field (identifier) @name))
 (union_type (field (identifier) @name))] @definition.field

; `RED,` in an enum
(enum_field
  (identifier) @name) @definition.constant

; `export def LIMIT: int = 8;`
(constant_declaration
  (identifier) @name) @definition.constant

; `const greeting: str = "hi";` at module level
(global_declaration
  "const"
  (global_binding
    (identifier) @name)) @definition.constant

; `let counter: int = 0;` at module level
(global_declaration
  (global_binding
    (identifier) @name)) @definition.variable

; `add(1, 2)` and `fmt::println(x)`
(call_expression
  callee: [(identifier) @name
           (scoped_type_identifier name: (identifier) @name)]) @reference.call
