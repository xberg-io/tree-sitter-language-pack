; HLSL (`.hlsl`). Functions, structs and other user types, macros and global declarations such
; as resources (`Texture2D albedo : register(t0);`) and uniforms. Declarations inside function
; bodies are locals.

(function_definition
  declarator: (function_declarator
                declarator: (identifier) @name)) @definition.function

; Methods defined inline in a struct or class body, or as `void Light::apply() { ... }`
(function_definition
  declarator: (function_declarator
                declarator: [(field_identifier) @name
                             (qualified_identifier
                               name: (identifier) @name)])) @definition.method

(struct_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(class_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(enum_specifier
  name: (type_identifier) @name
  body: (_)) @definition.enum

(enumerator
  name: (identifier) @name) @definition.constant

(type_definition
  declarator: (type_identifier) @name) @definition.type

(namespace_definition
  name: (namespace_identifier) @name) @definition.module

; `#define MAX_LIGHTS 4`
[(preproc_def name: (identifier) @name)
 (preproc_function_def name: (identifier) @name)] @definition.macro

; `float4x4 viewProj;`, `static const float k = 2.0;` and `Texture2D albedo : register(t0);`
(translation_unit
  (declaration
    declarator: [(identifier) @name
                 (array_declarator declarator: (identifier) @name)
                 (init_declarator declarator: (identifier) @name)]) @definition.variable)

; `[shader("fragment")]` and `[numthreads(8, 8, 1)]` are attributes, not calls; this claims
; their names before the call pattern below can tag them.
(hlsl_attribute
  (call_expression
    function: (identifier) @ignore))

; `shade(n)` and `tex.Sample(s, uv)`. Constructors of the built-in scalar, vector and matrix
; types such as `float3(...)`, and the `register(t0)` and `packoffset(c0)` bindings of a
; declaration, parse as calls and are left out.
(call_expression
  function: [(identifier) @name
             (field_expression field: (field_identifier) @name)]
  (#not-match? @name "^((float|half|double|u?int|bool|dword|min1[06](float|u?int))([1-4](x[1-4])?)?|register|packoffset)$")) @reference.call
