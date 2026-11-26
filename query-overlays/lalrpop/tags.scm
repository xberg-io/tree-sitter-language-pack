; Each nonterminal (`Name: Type = { ... };`, including macro nonterminals such as `Comma<T>`)
; defines a grammar rule. Its name is the identifier, without macro parameters.

(nonterminal
  (nonterminal_name
    .
    [(identifier) (macro_id)] @name)) @definition.function
