; re2c lexer specifications (`.re`). Named definitions (`digit = [0-9];`) of global and rules
; blocks, and named rules blocks. A `/*!local:re2c` block's definitions are local to it and are
; not tagged. The surrounding C or C++ code is host-language text the grammar does not parse.

; `digit = [0-9];` in a `/*!re2c` or `/*!rules:re2c` block
[(global_block
   (body
     (named_definition
       name: (name) @name) @definition.variable))
 (rules_block
   (body
     (named_definition
       name: (name) @name) @definition.variable))]

; `/*!rules:re2c:tokens ... */`
(rules_block
  (block_name) @name) @definition.module

; `/*!use:re2c:tokens */`
(use_block
  (block_name) @name) @reference.module

; `number = digit+;` and the rule `number { ... }` use the definitions `digit` and `number`.
[(regex (name) @name)
 (concat (name) @name)
 (alternation (name) @name)
 (repetition (name) @name)
 (parenthesized (name) @name)
 (difference (name) @name)
 (lookahead (name) @name)] @reference.variable
