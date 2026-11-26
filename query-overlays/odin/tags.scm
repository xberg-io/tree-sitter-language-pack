; Odin (`.odin`). The package and its file-scope declarations: procedures, structs, enums, unions,
; bit fields, constants and globals. Declarations inside a procedure body, including nested
; procedures, are locals. The name is the identifier right before `::` (or `:=`), which skips
; leading attributes such as `@(private)`.

(package_declaration
  (identifier) @name) @definition.module

; `add :: proc(a, b: int) -> int { ... }` and `area :: proc{area_circle, area_square}`
(source_file
  [(procedure_declaration
     (identifier) @name
     .
     "::")
   (overloaded_procedure_declaration
     (identifier) @name
     .
     "::")] @definition.function)

(source_file
  (struct_declaration
    (identifier) @name
    .
    "::") @definition.struct)

(source_file
  (enum_declaration
    (identifier) @name
    .
    "::") @definition.enum)

(source_file
  (union_declaration
    (identifier) @name
    .
    "::") @definition.union)

(source_file
  (bit_field_declaration
    (identifier) @name
    .
    "::") @definition.struct)

; `Score :: distinct int`
(source_file
  (const_declaration
    (identifier) @name
    .
    "::"
    .
    (distinct_type)) @definition.type)

; `LIMIT :: 8` and `SCALE : f32 : 2.0`
(source_file
  [(const_declaration
     (identifier) @name
     .
     "::")
   (const_type_declaration
     (identifier) @name
     .
     ":")] @definition.constant)

; `counter := 0` and `total: int`
(source_file
  [(variable_declaration
     (identifier) @name
     .
     ":=")
   (var_declaration
     (identifier) @name
     .
     ":")] @definition.variable)

; `add(1, 2)` and `fmt.println(x)`
(call_expression
  function: (identifier) @name) @reference.call
