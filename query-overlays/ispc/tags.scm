; ISPC (`.ispc`), a C dialect. Based on queries/tags.scm of
; https://github.com/tree-sitter-grammars/tree-sitter-ispc. Function definitions (including
; `export` and `task` functions), types with a body, typedefs, enumerators, file-scope variables
; and macros. A prototype without a body declares a function defined elsewhere (or below) and is
; not tagged, so a declared-then-defined function is tagged once. Declarations inside function
; bodies are locals.

(function_definition
  declarator: (function_declarator
                declarator: (identifier) @name)) @definition.function

(struct_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(union_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(enum_specifier
  name: (type_identifier) @name
  body: (_)) @definition.enum

(enumerator
  name: (identifier) @name) @definition.constant

(type_definition
  declarator: (type_identifier) @name) @definition.type

; `static const uniform float SCALE = 2.0f;` and `uniform int call_count = 0;` at file scope
(translation_unit
  (declaration
    (type_qualifier) @_qualifier
    declarator: [(identifier) @name
                 (init_declarator declarator: (identifier) @name)]) @definition.constant
  (#eq? @_qualifier "const"))

(translation_unit
  (declaration
    declarator: [(identifier) @name
                 (init_declarator declarator: (identifier) @name)]) @definition.variable)

; `#define LANES 8` and `#define SQUARE(x) ...`
[(preproc_def name: (identifier) @name)
 (preproc_function_def name: (identifier) @name)] @definition.macro

(call_expression
  function: (identifier) @name) @reference.call
