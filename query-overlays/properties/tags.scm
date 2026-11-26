; Based on queries/tags.scm of https://github.com/tree-sitter-grammars/tree-sitter-properties.
; A key is a configuration variable and `${key}` substitutes it, so keys are variables.

(property
  (key) @name) @definition.variable

(substitution
  (key) @name) @reference.variable
