; Zig (`.zig`). Container-level declarations, at file scope or inside a struct, enum, union or
; opaque body: functions and methods, the types bound by `const Name = struct { ... }` (struct and
; union are classes), struct and union fields, enum and error-set members, constants, variables
; and `test` blocks. Declarations inside a function body and `@import` aliases are claimed below
; without a tag.

; `const total = a + b;` inside a block is a local.
(block
  (variable_declaration
    ["const" "var"]
    .
    (identifier) @ignore))

; `const std = @import("std");` names an imported file, not a declaration of this one.
(variable_declaration
  "const"
  .
  (identifier) @ignore
  (builtin_function
    (builtin_identifier) @_builtin)
  (#eq? @_builtin "@import"))

; `const Point = struct { ... };` by the kind of container it binds
(variable_declaration
  "const"
  .
  (identifier) @name
  (struct_declaration)) @definition.class

(variable_declaration
  "const"
  .
  (identifier) @name
  (enum_declaration)) @definition.enum

(variable_declaration
  "const"
  .
  (identifier) @name
  (union_declaration)) @definition.class

(variable_declaration
  "const"
  .
  (identifier) @name
  [(opaque_declaration) (error_set_declaration)]) @definition.type

; `x: i32,` in a struct or union; `red,` in an enum
[(struct_declaration (container_field name: (identifier) @name))
 (union_declaration (container_field name: (identifier) @name))] @definition.field

(enum_declaration
  (container_field
    name: (identifier) @name)) @definition.constant

; `error{ NotFound, AccessDenied }`
(error_set_declaration
  (identifier) @name) @definition.constant

; `pub fn sum(self: Point) i32 { ... }` inside a container body
[(struct_declaration (function_declaration name: (identifier) @name) @definition.method)
 (enum_declaration (function_declaration name: (identifier) @name) @definition.method)
 (union_declaration (function_declaration name: (identifier) @name) @definition.method)
 (opaque_declaration (function_declaration name: (identifier) @name) @definition.method)]

(function_declaration
  name: (identifier) @name) @definition.function

; `pub const max_sides: u32 = 8;`
(variable_declaration
  "const"
  .
  (identifier) @name) @definition.constant

; `var counter: u32 = 0;`
(variable_declaration
  "var"
  .
  (identifier) @name) @definition.variable

; `test "addition" { ... }`
(test_declaration
  (string
    (string_content) @name)) @definition.function

; `add(1, 2)` and `std.debug.print(...)`
(call_expression
  function: [(identifier) @name
             (field_expression member: (identifier) @name)]) @reference.call
