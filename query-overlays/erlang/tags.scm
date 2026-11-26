; Module-level forms are the declarations. The grammar gives each clause of a function its own
; `fun_decl`, so a function is tagged once, at its first clause: a clause that follows a clause
; ending in `;` continues the same function and is claimed without a tag.

(source_file
  (fun_decl ";")
  .
  (fun_decl
    clause: (function_clause name: (atom) @ignore)))

(source_file
  (fun_decl
    clause: (function_clause name: (atom) @name)) @definition.function)

; `-module(name).`
(module_attribute
  name: (atom) @name) @definition.module

; `-record(name, {field, ...}).` and its fields
(record_decl
  name: (atom) @name) @definition.class

(record_decl
  (record_field
    name: (atom) @name) @definition.field)

; `-type`, `-opaque` and `-nominal` type declarations
[
  (type_alias name: (type_name name: (atom) @name))
  (opaque name: (type_name name: (atom) @name))
  (nominal name: (type_name name: (atom) @name))
] @definition.type

; `-callback name(...) -> ...`: a function a behaviour's implementations must define
(callback
  fun: (atom) @name) @definition.method

; `-define(NAME, ...)` and `-define(NAME(Args), ...)`
(pp_define
  lhs: (macro_lhs name: (_) @name)) @definition.macro
