; WGSL with the Bevy shader preprocessor (`#define_import_path`, `#import`). Module-scope
; declarations: functions, structs, type aliases, constants and overrides, and global variables,
; plus the import path a shader module defines. Declarations inside function bodies (`let`,
; `var`) are locals.

; `#define_import_path my_shaders::lighting`
(define_import_path
  path: (import_path) @name) @definition.module

(function_declaration
  name: (identifier) @name) @definition.function

(struct_declaration
  name: (identifier) @name) @definition.struct

; `type Color = vec3<f32>;`
(type_alias_declaration
  (identifier) @name) @definition.type

; `let PI: f32 = 3.14159;` and `override scale: f32 = 1.0;` at module scope
(global_constant_declaration
  [(identifier) @name
   (variable_identifier_declaration
     name: (identifier) @name)]) @definition.constant

; `@group(0) @binding(0) var<uniform> sun: Light;`
(global_variable_declaration
  (variable_declaration
    (variable_identifier_declaration
      name: (identifier) @name))) @definition.variable

; `lambert(n, sun)` and `max(a, b)`; constructors of built-in types such as `vec3<f32>(...)`
; carry no identifier and are not tagged.
(type_constructor_or_function_call_expression
  (type_declaration
    (identifier) @name)) @reference.call
