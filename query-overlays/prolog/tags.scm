; Prolog. Clause heads define predicates: a fact `name(...).` or `name.`, and the head of a rule
; `name(...) :- body.`; a predicate's name is the first atom of its clause. The grammar nests a
; rule's `:-` at a varying depth of an operator spine, so the head atom is reached through the
; first named child at each level, for every depth up to nine. Only the first of consecutive
; clauses with the same name is tagged: a query cannot count arguments, so `p/2` directly followed
; by `p/3` is tagged once, and a comment between two clauses of one predicate, or a predicate
; whose clauses are not contiguous, tags it again. Of the directives only `:- module(...)`
; declares a name.
;
; Goals are not tagged as calls: the grammar parses every infix operator at one precedence, so
; `(male(X) -> K = son(X) ; K = daughter(X))` nests the data term `son(X)` as an operand of `;`,
; and a goal cannot be told apart from a term passed as data.

; A clause continuing the predicate of the clause before it.
((clause_term
   .
   [
     (_ . (_ . (atom) @_previous))
     (_ . (_ . (_ . (atom) @_previous)))
     (_ . (_ . (_ . (_ . (atom) @_previous))))
     (_ . (_ . (_ . (_ . (_ . (atom) @_previous)))))
     (_ . (_ . (_ . (_ . (_ . (_ . (atom) @_previous))))))
     (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @_previous)))))))
     (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @_previous))))))))
     (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @_previous)))))))))
   ])
  .
  (clause_term
    .
    [
      (_ . (_ . (atom) @ignore))
      (_ . (_ . (_ . (atom) @ignore)))
      (_ . (_ . (_ . (_ . (atom) @ignore))))
      (_ . (_ . (_ . (_ . (_ . (atom) @ignore)))))
      (_ . (_ . (_ . (_ . (_ . (_ . (atom) @ignore))))))
      (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @ignore)))))))
      (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @ignore))))))))
      (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @ignore)))))))))
    ])
  (#eq? @_previous @ignore))

(clause_term
  .
  [
    (_ . (_ . (atom) @name))
    (_ . (_ . (_ . (atom) @name)))
    (_ . (_ . (_ . (_ . (atom) @name))))
    (_ . (_ . (_ . (_ . (_ . (atom) @name)))))
    (_ . (_ . (_ . (_ . (_ . (_ . (atom) @name))))))
    (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @name)))))))
    (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @name))))))))
    (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (_ . (atom) @name)))))))))
  ]) @definition.function

; `:- module(family, [...]).`
(directive_term
  (compound_term
    (functional_notation
      name: (atom) @_directive
      arguments: (args . (atomic (atom) @name))))
  (#eq? @_directive "module")) @definition.module
