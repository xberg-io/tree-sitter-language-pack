; Based on the vendored tree-sitter-c queries/tags.scm (github.com/xberg-io/tree-sitter-c).
; ~keep A prototype and its definition name one function: only `function_definition` tags, so a
; ~keep function declared but defined in another file has no tag in this one.

; `struct point { ... }` and `union value { ... }`; a bare `struct point p;` is a use.
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

; `typedef struct point point_t;`, `typedef struct node *node_ptr;` and
; `typedef int (*compare_fn)(int, int);`
(type_definition
  declarator: (type_identifier) @name) @definition.type

(type_definition
  declarator: (pointer_declarator
    declarator: (type_identifier) @name)) @definition.type

(type_definition
  declarator: (function_declarator
    declarator: (parenthesized_declarator
      (pointer_declarator
        declarator: (type_identifier) @name)))) @definition.type

; `int add(int a, int b) { ... }` and `char *describe(...) { ... }`
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.function

(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition.function

(preproc_def
  name: (identifier) @name) @definition.macro

(preproc_function_def
  name: (identifier) @name) @definition.macro

; File-scope variables. An `extern` declaration names a variable defined elsewhere; a `const`
; one is a constant. Declarations inside function bodies are locals.
(translation_unit
  (declaration
    (storage_class_specifier) @_storage
    declarator: [
      (identifier) @ignore
      (init_declarator
        declarator: (identifier) @ignore)
      (array_declarator
        declarator: (identifier) @ignore)
    ])
  (#eq? @_storage "extern"))

(translation_unit
  (declaration
    (type_qualifier) @_qualifier
    declarator: [
      (identifier) @name
      (init_declarator
        declarator: (identifier) @name)
    ]) @definition.constant
  (#eq? @_qualifier "const"))

(translation_unit
  (declaration
    declarator: [
      (identifier) @name
      (pointer_declarator
        declarator: (identifier) @name)
      (array_declarator
        declarator: (identifier) @name)
      (init_declarator
        declarator: [
          (identifier) @name
          (pointer_declarator
            declarator: (identifier) @name)
          (array_declarator
            declarator: (identifier) @name)
        ])
    ]) @definition.variable)

(call_expression
  function: (identifier) @name) @reference.call
