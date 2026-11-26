; `def name: body;` and `def name(args): body;` define functions. A program's definitions nest
; inside one another's continuation queries, so definitions are matched at any depth.

(funcdef
  .
  (identifier) @name) @definition.function
