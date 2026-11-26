(program
  name: (identifier) @name) @definition.module

; Program-level variables and event flags (`int count;`, `evflag done;`).
(initial_defn
  (declaration
    (init_declarator
      (declarator (identifier) @name)))) @definition.variable

(funcdef
  (declarator
    (function_declarator
      declarator: (identifier) @name))) @definition.function

; A state set is a concurrent state machine; its states are entered by name from transitions.
(state_set
  name: (identifier) @name) @definition.class

(state
  name: (identifier) @name) @definition.method

(transition
  name: (identifier) @name) @reference.call

(call_expression
  function: (identifier) @name) @reference.call
