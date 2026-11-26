; Each rule of the rule list defines a grammar rule. An incremental alternative
; (`name =/ ...`) extends a rule defined elsewhere, so it is not a second definition.

(rulelist
  (rule
    (rulename) @name
    (defined_as) @_op
    (#not-match? @_op "/")) @definition.function)
