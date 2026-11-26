; Declarations at the top level or directly in a `struct ... end` body: structures, signatures,
; functors, datatypes with their constructors, type abbreviations, exceptions, `fun` and `val`
; bindings, and the values a signature specifies. A `fun` with several clauses is tagged once, at
; its first clause. Declarations in `let ... in` are local.

(strbind
  name: (strid) @name) @definition.module

(fctbind
  name: (fctid) @name) @definition.module

(sigbind
  name: (sigid) @name) @definition.interface

(datbind
  name: (tycon) @name) @definition.type

(conbind
  name: (vid) @name) @definition.constructor

(typbind
  name: (tycon) @name) @definition.type

(exbind
  name: (vid) @name) @definition.constructor

(valdesc
  name: (vid) @name) @definition.method

[
  (source_file (fun_dec (fvalbind . (fmrule name: (vid) @name)) @definition.function))
  (struct_strexp strdec: (fun_dec (fvalbind . (fmrule name: (vid) @name)) @definition.function))
]

[
  (source_file (val_dec (valbind pat: (vid_pat) @name) @definition.variable))
  (struct_strexp strdec: (val_dec (valbind pat: (vid_pat) @name) @definition.variable))
]
