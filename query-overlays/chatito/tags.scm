; Chatito (`.chatito`) NLU training data. Based on queries/tags.scm of
; https://github.com/tree-sitter-grammars/tree-sitter-chatito, with its roles mapped onto the
; common tag roles: intents and aliases are the sentence-generating rules of the grammar
; (functions an intent or alias body expands), slots are the variables a sentence fills.

; `%[greet]`
(intent_def
  (intent) @name) @definition.function

; `~[hi]`
(alias_def
  (alias) @name) @definition.function

; `@[name]`
(slot_def
  (slot) @name) @definition.variable

(slot_ref
  (slot) @name) @reference.variable

(alias_ref
  (alias) @name) @reference.call
