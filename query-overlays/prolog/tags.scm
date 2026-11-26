; Clause heads define predicates: a fact `name(...).` or `name.`, and the left-hand side of a
; rule `name(...) :- body.`. The grammar nests a rule's `:-` at a varying depth of a left-leaning
; operator spine, so a clause cannot be related to the one before it: every clause of a predicate
; is tagged. Directives (`:- dynamic ...`) declare nothing new, and goals in a body are calls.

(clause_term
  .
  [
    (compound_term (functional_notation name: (atom) @name))
    (atomic (atom) @name)
  ]) @definition.function

(infix_operator
  left: [
    (compound_term (functional_notation name: (atom) @name))
    (atomic (atom) @name)
  ]
  operator: (operator) @_neck
  (#eq? @_neck ":-")) @definition.function
