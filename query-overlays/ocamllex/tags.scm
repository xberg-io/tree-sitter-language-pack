; `let name = regexp` defines a named regular expression and each `rule name = parse ...` entry
; (and its `and name = ...` siblings) a lexer entry point, which ocamllex turns into an OCaml
; function. Names bound with `as` inside a case are local to its action.

(named_regexp
  name: (regexp_name) @name) @definition.constant

(lexer_entry
  name: (lexer_entry_name) @name) @definition.function
