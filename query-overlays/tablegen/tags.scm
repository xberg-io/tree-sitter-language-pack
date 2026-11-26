; LLVM TableGen (`.td`). Classes, multiclasses, the records a file defines with `def` and `defm`,
; and global `defvar`, `defset` and `deftype` names. The `def`s inside a multiclass body are
; templates of the records `defm` creates and are not tagged; neither are the fields of a record
; body.

; `class Reg<string n, int num> { ... }`
(class
  name: (identifier) @name) @definition.class

; `multiclass ALU<string opc> { ... }`
(multiclass
  .
  (identifier) @name) @definition.class

; `def R0 : Reg<"r0", 0>;` and `defm ADD : ALU<"add">;`, also under `let ... in`
(statement
  [(def
     .
     (value
       (identifier) @name))
   (defm
     .
     (value
       (identifier) @name))] @definition.constant)

; `defvar Width = 32;` at file scope and `defset list<Reg> AllRegs = { ... }`
(file
  (statement
    (defvar
      .
      (identifier) @name) @definition.variable))

(defset
  (type)
  .
  (identifier) @name) @definition.variable

; `deftype RegList = list<Reg>;`
(deftype
  .
  (identifier) @name) @definition.type

; `: Reg<...>` names the class a record or class derives from.
(parent_class_list
  (identifier) @name) @reference.class
