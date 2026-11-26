; Top-level declarations: the module header, `type`/`struct`/`effect`/`alias` declarations,
; constructors and effect operations, and top-level `fun`, `val` and `extern` declarations.
; Functions and values declared inside a body are local.

(program
  (modulepath) @name) @definition.module

(typedecl
  type: (typeid) @name) @definition.type

(typedecl
  struct: (typeid) @name) @definition.struct

(constructor
  (conid) @name) @definition.constructor

; `effect name ... { ops }` and `named effect name ...`
(typedecl
  [
    effect_multiple: (varid) @name
    named_effect_multiple: (varid) @name
  ]) @definition.interface

; The operations of an effect, including a single-operation `effect ctl name(...)`
(operation
  (identifier) @name) @definition.method

(aliasdecl
  (typeid) @name) @definition.type

(topdecl
  (puredecl
    fun: (qidentifier) @name) @definition.function)

(topdecl
  (puredecl
    val: (binder (qidentifier) @name)) @definition.constant)

(topdecl
  (externdecl
    (qidentifier) @name) @definition.function)
