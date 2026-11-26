; Modules. ~keep The declaration also holds the empty end-of-rule (eor) token, so the name is
; ~keep the atom only.
(module_declaration
  (atom) @name) @definition.module

(import_declaration
  (atom) @name) @reference.module

; Functions and predicates, one tag per clause
(function_rule
  name: (atom) @name) @definition.function

(function_fact
  name: (atom) @name) @definition.function

(predicate_rule
  name: (atom) @name) @definition.function

(predicate_fact
  name: (atom) @name) @definition.function

(action_rule
  name: (atom) @name) @definition.function

(nonbacktrackable_predicate_rule
  name: (atom) @name) @definition.function

; Calls: `f(X)` and `X.f(Y)`
(function_call
  function: (atom) @name) @reference.call

(dot_expression
  right: (atom) @name) @reference.call
