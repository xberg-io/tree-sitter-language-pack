; GLSL (`.glsl`). Functions, structs, interface blocks, macros and global declarations such as
; `uniform`, `in` and `out` variables. Declarations inside function bodies are locals.

(function_definition
  declarator: (function_declarator
                declarator: (identifier) @name)) @definition.function

(struct_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

; Interface blocks: `uniform Camera { mat4 view; } camera;` names the block `Camera` and the
; instance `camera`.
(translation_unit
  (declaration
    (identifier) @name
    .
    (field_declaration_list)) @definition.interface)

(translation_unit
  (declaration
    (field_declaration_list)
    .
    (identifier) @name) @definition.variable)

; `#define PI 3.14159`
[(preproc_def name: (identifier) @name)
 (preproc_function_def name: (identifier) @name)] @definition.macro

; `uniform mat4 model;` and `const float scale = 2.0;`
(translation_unit
  (declaration
    declarator: [(identifier) @name
                 (array_declarator declarator: (identifier) @name)
                 (init_declarator declarator: (identifier) @name)]) @definition.variable)

; `lambert(n, l)`. Constructors of the built-in scalar, vector and matrix types such as
; `vec3(...)` parse as calls and are left out.
(call_expression
  function: (identifier) @name
  (#not-match? @name "^([biud]?vec[234]|d?mat[234](x[234])?|float|double|u?int|bool)$")) @reference.call
