; Routines, named by the keyword that declares them. An exported name is followed by `*`,
; which is a separate node.
(routine
  (keyw) @_keyword
  .
  (symbol) @name
  (#any-of? @_keyword "proc" "func" "iterator" "converter")) @definition.function

(routine
  (keyw) @_keyword
  .
  (symbol) @name
  (#eq? @_keyword "method")) @definition.method

(routine
  (keyw) @_keyword
  .
  (symbol) @name
  (#any-of? @_keyword "template" "macro")) @definition.macro

; Types: each name in a `type` section, then its definition after an optional export marker,
; generic parameters and pragmas
(typeDef
  (symbol) @name
  .
  (opr)?
  .
  (genericParamList)?
  .
  (pragma)?
  .
  (primaryTypeDef
    (objectDecl)) @definition.class)

(typeDef
  (symbol) @name
  .
  (opr)?
  .
  (genericParamList)?
  .
  (pragma)?
  .
  (primaryTypeDef
    (enumDecl)) @definition.enum)

(typeDef
  (symbol) @name @definition.type)

; Module-level constants and variables
(module
  (stmt
    (declaration
      (constant
        (declColonEquals
          (symbol) @name) @definition.constant))))

(module
  (stmt
    (declaration
      (variable
        (declColonEquals
          (symbol) @name) @definition.variable))))

; Calls: `f(x)`, `x.f()` and command calls `f x`
(primary
  (symbol) @name
  .
  (primarySuffix
    [
      (functionCall)
      (cmdCall)
    ])) @reference.call

(primary
  (primarySuffix
    (qualifiedSuffix
      (symbol) @name))
  .
  (primarySuffix
    [
      (functionCall)
      (cmdCall)
    ])) @reference.call
