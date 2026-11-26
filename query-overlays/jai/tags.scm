; Jai (`.jai`). File-scope declarations: procedures, structs, enums, constants and globals.
; Declarations inside a procedure body, including nested procedures, are locals.

; `add :: (a: int, b: int) -> int { ... }`
(source_file
  (statement
    (procedure_declaration
      name: (identifier) @name) @definition.function))

; `Vector2 :: struct { ... }` and `Shape :: union { ... }`
(source_file
  (statement
    (struct_declaration
      name: (identifier) @name) @definition.class))

(source_file
  (statement
    (enum_declaration
      name: (identifier) @name) @definition.enum))

; `LIMIT :: 8;`
(source_file
  (statement
    (declarations_that_require_a_semicolon
      (const_declaration
        name: (identifier) @name) @definition.constant)))

; `counter := 0;` and `scale: float;`
(source_file
  (statement
    (declarations_that_require_a_semicolon
      (variable_declaration
        name: (identifier) @name) @definition.variable)))

; `add(1, 2)` and `list.push(x)`
(call_expression
  function: [(identifier) @name
             (member_expression (identifier) @name .)]) @reference.call
