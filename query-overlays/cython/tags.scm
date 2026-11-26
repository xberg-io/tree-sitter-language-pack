; Cython (`.pyx`, `.pxd`). Based on the tags.scm of b0o/tree-sitter-cython, which covered only the
; Python forms. Adds `cdef`/`cpdef` functions and methods, `cdef class` attributes, `cdef struct`
; (a class) with its members, `cdef enum` with its values, `ctypedef` (a type), functions declared
; in a `cdef extern from` block and module-level `cdef` variables. `cdef` declarations inside a
; function body are locals.

; Python and `cdef` classes
(class_definition
  name: (identifier) @name) @definition.class

; `cdef struct Vec2:` and its members
(struct
  (c_identifier
    name: (identifier) @name)) @definition.class

(struct_suite
  (cvar_decl
    (identifier) @name)) @definition.field

; `cdef enum Color:` and its values
(enum
  (identifier) @name) @definition.enum

(enum_block
  (c_identifier
    name: (identifier) @name)) @definition.constant

; `ctypedef double real_t`
(ctypedef_statement
  (cvar_decl
    (identifier) @name)) @definition.type

; `int abs(int x)` inside `cdef extern from "stdlib.h":`
(extern_suite
  (cvar_decl
    (c_name
      (identifier) @name)
    (c_function_definition))) @definition.function

; `cpdef double scaled(self, double k):` and `def area(self):` in a class body
(class_definition
  body: (block
    [(cdef_statement
       (cvar_def
         (maybe_typed_name
           name: (identifier) @name)
         (c_function_definition)))
     (function_definition
       name: (identifier) @name)] @definition.method))

; `cdef public double x` in a `cdef class` body
(class_definition
  body: (block
    (cdef_statement
      (cvar_def
        (maybe_typed_name
          name: (identifier) @name))) @definition.field))

; `cdef int square(int x):` and `cpdef double norm(Vec2 v):` at module level
(module
  (cdef_statement
    (cvar_def
      (maybe_typed_name
        name: (identifier) @name)
      (c_function_definition))) @definition.function)

(function_definition
  name: (identifier) @name) @definition.function

; `cdef int counter = 0` at module level
(module
  (cdef_statement
    (cvar_def
      (maybe_typed_name
        name: (identifier) @name))) @definition.variable)

(module
  (expression_statement
    (assignment
      left: (identifier) @name) @definition.constant))

(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call
