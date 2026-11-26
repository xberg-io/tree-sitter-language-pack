; Slang (`.slang`). Interfaces, structs and other user types, namespaces, functions, methods,
; macros and global declarations. Declarations inside function bodies are locals.

(interface_specifier
  name: (type_identifier) @name
  body: (_)) @definition.interface

(struct_specifier
  name: (type_identifier) @name
  body: (_)) @definition.struct

(class_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(enum_specifier
  name: (type_identifier) @name
  body: (_)) @definition.enum

(namespace_definition
  name: (namespace_identifier) @name) @definition.module

; `typedef float3 Color;` and `using Color = float3;`
[(type_definition declarator: (type_identifier) @name)
 (alias_declaration name: (type_identifier) @name)] @definition.type

; Interface requirements: `float3 illuminate(float3 n);` inside `interface ILight`
(interface_specifier
  body: (field_declaration_list
          (field_declaration
            declarator: (function_declarator
                          declarator: (field_identifier) @name)) @definition.method))

; Methods defined in a struct body or as `float3 DirLight::illuminate(...) { ... }`
(function_definition
  declarator: (function_declarator
                declarator: [(field_identifier) @name
                             (qualified_identifier
                               name: (identifier) @name)])) @definition.method

(function_definition
  declarator: (function_declarator
                declarator: (identifier) @name)) @definition.function

; `#define MAX_LIGHTS 4`
[(preproc_def name: (identifier) @name)
 (preproc_function_def name: (identifier) @name)] @definition.macro

; `static const float GAMMA = 2.2;` and `uniform float4 tint;`
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

; `light.illuminate(n)` and `util::square(x)`. Constructors of the built-in scalar, vector and
; matrix types such as `float3(...)` parse as calls and are left out.
(call_expression
  function: [(identifier) @name
             (field_expression field: (field_identifier) @name)
             (qualified_identifier name: (identifier) @name)]
  (#not-match? @name "^(float|half|double|u?int|bool|int(8|16|64)_t|uint(8|16|64)_t)([1-4](x[1-4])?)?$")) @reference.call
