; Based on queries/tags.scm of https://github.com/tree-sitter/tree-sitter-ql, with `newtype`
; declarations tagged as types.

(classlessPredicate
  name: (predicateName) @name) @definition.function

(memberPredicate
  name: (predicateName) @name) @definition.method

(aritylessPredicateExpr
  name: (literalId) @name) @reference.call

(module
  name: (moduleName) @name) @definition.module

(dataclass
  name: (className) @name) @definition.class

; `newtype TNode = ...` declares a new type whose branches are each a type of their own
(datatype
  name: (className) @name) @definition.type

(datatypeBranch
  name: (className) @name) @definition.class

(qualifiedRhs
  name: (predicateName) @name) @reference.call

(typeExpr
  name: (className) @name) @reference.type
