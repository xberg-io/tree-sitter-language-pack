; Godot shading language (`.gdshader`). Based on queries/tags.scm of
; https://github.com/airblast-dev/tree-sitter-gdshader, extended with the file-level
; declarations: constants, uniforms, varyings, struct fields and macros. Declarations inside a
; function body are locals and are not tagged.

(function_definition
  declarator: (identifier) @name) @definition.function

(struct_definition
  name: (identifier) @name) @definition.class

(field_definition
  declarator: (identifier) @name) @definition.field

; `const float PI = 3.14;`
(source_file
  (const_declaration
    declarator: [(identifier) @name
                 (init_declarator declarator: (identifier) @name)]) @definition.constant)

; `uniform vec4 tint : source_color;` and `varying vec3 world_normal;`
[(uniform_declaration
   declarator: [(identifier) @name
                (init_declarator declarator: (identifier) @name)])
 (varying_declaration
   declarator: [(identifier) @name
                (init_declarator declarator: (identifier) @name)])] @definition.variable

; `#define STRENGTH 0.5` and `#define SCALE(v) ...`
[(preproc_define name: (identifier) @name)
 (preproc_define_func name: (identifier) @name)] @definition.macro

; `brightness(c)` and `texture(tex, UV)`. Constructors of the built-in types such as `vec3(...)`
; have a `primitive_type` callee and are not calls of a definition.
(call_expression
  function: (identifier) @name) @reference.call

; `values.length()`
(method_expression
  method: (identifier) @name) @reference.call

; A user-defined struct used as a type: `Light make_light(...)`
(type_identifier) @name @reference.class
