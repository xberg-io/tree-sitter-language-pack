; Modules. ~keep The declaration also holds the empty end-of-rule (eor) token, so the name is
; ~keep the atom only.
(module_declaration
  (atom) @name) @definition.module

(import_declaration
  (atom) @name) @reference.module

; Functions and predicates are tagged at their first clause. A later clause of the same name
; right after it (in the same definition or the next one) is claimed without a tag.
(_
  [(function_rule name: (atom) @_first)
   (function_fact name: (atom) @_first)
   (predicate_rule name: (atom) @_first)
   (predicate_fact name: (atom) @_first)
   (action_rule name: (atom) @_first)
   (nonbacktrackable_predicate_rule name: (atom) @_first)]
  .
  [(function_rule name: (atom) @ignore)
   (function_fact name: (atom) @ignore)
   (predicate_rule name: (atom) @ignore)
   (predicate_fact name: (atom) @ignore)
   (action_rule name: (atom) @ignore)
   (nonbacktrackable_predicate_rule name: (atom) @ignore)]
  (#eq? @_first @ignore))

([(function_definition
   [(function_rule name: (atom) @_first)
    (function_fact name: (atom) @_first)])
  (predicate_definition
   [(predicate_rule name: (atom) @_first)
    (predicate_fact name: (atom) @_first)])
  (actor_definition
   [(action_rule name: (atom) @_first)
    (nonbacktrackable_predicate_rule name: (atom) @_first)])]
  .
  [(function_definition
    .
    [(function_rule name: (atom) @ignore)
     (function_fact name: (atom) @ignore)])
   (predicate_definition
    .
    [(predicate_rule name: (atom) @ignore)
     (predicate_fact name: (atom) @ignore)])
   (actor_definition
    .
    [(action_rule name: (atom) @ignore)
     (nonbacktrackable_predicate_rule name: (atom) @ignore)])]
  (#eq? @_first @ignore))

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

; ~keep nonbacktrackable_predicate_rule is not tagged on its own: the grammar only reaches it
; ~keep after an action rule, where it is a later clause of that rule's predicate.

; Calls: `f(X)` and `X.f(Y)`
(function_call
  function: (atom) @name) @reference.call

(dot_expression
  right: (atom) @name) @reference.call
