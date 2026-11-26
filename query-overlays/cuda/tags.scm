; CUDA (`.cu`), a C++ dialect. Function definitions (kernels and host or device functions), types
; with a body, namespaces, typedefs and macros. Declarations inside function bodies are locals.

; `__global__ void saxpy(...) { ... }` and `float *alloc(int n) { ... }`
(function_definition
  declarator: [(function_declarator
                 declarator: (identifier) @name)
               (pointer_declarator
                 declarator: (function_declarator
                               declarator: (identifier) @name))
               (reference_declarator
                 (function_declarator
                   declarator: (identifier) @name))]) @definition.function

; Methods: defined inline in a class body or out of line as `void Grid::resize() { ... }`
(function_definition
  declarator: (function_declarator
                declarator: [(field_identifier) @name
                             (qualified_identifier
                               name: (identifier) @name)])) @definition.method

(struct_specifier
  name: (type_identifier) @name
  body: (_)) @definition.struct

(class_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(union_specifier
  name: (type_identifier) @name
  body: (_)) @definition.union

(enum_specifier
  name: (type_identifier) @name
  body: (_)) @definition.enum

; `typedef float real;` and `using real = float;`
(type_definition
  declarator: (type_identifier) @name) @definition.type

(alias_declaration
  name: (type_identifier) @name) @definition.type

(namespace_definition
  name: (namespace_identifier) @name) @definition.module

; `#define BLOCK 256` and `#define IDX(i, j) ...`
[(preproc_def name: (identifier) @name)
 (preproc_function_def name: (identifier) @name)] @definition.macro

; Calls, including kernel launches `saxpy<<<grid, block>>>(...)`
(call_expression
  function: [(identifier) @name
             (field_expression field: (field_identifier) @name)
             (qualified_identifier name: (identifier) @name)]) @reference.call
