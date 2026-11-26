; Grammar rules (`name: productions` and the newer `let name := ...`) and the terminals declared
; with `%token` are the declarations. `%start`, `%type` and precedence declarations refer to
; symbols declared elsewhere.

(old_rule
  (symbol (lid) @name)) @definition.function

(new_rule
  .
  (lid) @name) @definition.function

(declaration
  (terminal_alias_attrs
    (uid) @name) @definition.constant)
