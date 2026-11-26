; Global declarations are the declarations: `proctype` processes, `typedef` structures, named
; `never` claims and `ltl` formulas, global variables and channels, and the constants an `mtype`
; declaration introduces. Variables declared inside a process body are local.

(program
  (proc
    (uname) @name) @definition.function)

(program
  (utype
    (uname) @name) @definition.class)

(program
  [
    (claim (uname) @name)
    (ltl (uname) @name)
  ] @definition.function)

(program
  (one_decl
    (var_list
      (ivar
        (vardcl
          .
          (uname) @name)))) @definition.variable)

; `mtype = { request, release };`
(program
  (one_decl
    (nlst
      (uname) @name)) @definition.constant)
