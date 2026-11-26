; `let name = term` binds a named lambda term. `eval name : ...` blocks are named reduction
; checks, not definitions other code can refer to.

(source_file
  (definition
    (function) @name) @definition.function)
