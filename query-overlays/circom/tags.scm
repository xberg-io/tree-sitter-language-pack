; Circom (`.circom`). Templates are classes: each instantiates as a component whose signals are
; read as members (`mult.out`). Signals and components declared in a template body are fields.
; Functions are functions; calls to a template or function are call references. `var` locals and
; parameters are not tagged.

(template_definition
  name: (identifier) @name) @definition.class

(function_definition
  name: (identifier) @name) @definition.function

(signal_declaration_statement
  name: (identifier) @name) @definition.field

(component_declaration_statement
  name: (identifier) @name) @definition.field

(call_expression
  (identifier) @name) @reference.call
