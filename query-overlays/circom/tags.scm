; The grammar's lexer folds the spaces between `template` and the name into the name token
; (`template Multiplier` names " Multiplier"); a name on the next line is clean.
(template_definition
  name: (identifier) @name) @definition.class

(function_definition
  name: (identifier) @name) @definition.function

(component_declaration_statement
  name: (identifier) @name) @definition.variable

(call_expression
  . (identifier) @name) @reference.call
