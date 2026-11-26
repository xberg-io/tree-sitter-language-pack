; Declarations are the type signatures (`name : Type`), which the grammar parses as a
; `function` whose left-hand side holds a `function_name`; the defining clauses that follow
; (`name x = ...`) have no `function_name` and are not tagged again. Signatures inside a
; `data` body declare its constructors. Helpers in `where` and `let` blocks are local.

(module
  (module_name (qid) @name)) @definition.module

(data
  (data_name) @name) @definition.type

(data
  (function
    (lhs
      (function_name (atom (qid) @name)))) @definition.constructor)

(record
  (record_name (qid) @name)) @definition.struct

(record_constructor
  (id) @name) @definition.constructor

(fields
  (signature
    (field_name) @name) @definition.field)

(source_file
  (function
    (lhs
      (function_name (atom (qid) @name)))) @definition.function)

(module
  (function
    (lhs
      (function_name (atom (qid) @name)))) @definition.function)

(postulate
  (function
    (lhs
      (function_name (atom (qid) @name)))) @definition.function)

(private
  (function
    (lhs
      (function_name (atom (qid) @name)))) @definition.function)

(abstract
  (function
    (lhs
      (function_name (atom (qid) @name)))) @definition.function)

(mutual
  (function
    (lhs
      (function_name (atom (qid) @name)))) @definition.function)

(instance
  (function
    (lhs
      (function_name (atom (qid) @name)))) @definition.function)
