; Macros, blocks and template variables a Tera template defines, plus macro and function calls.

; `{% macro input(name) %}`
(macro_statement name: (identifier) @name) @definition.macro

; `{% block content %}`; the identifier after `endblock` repeats the name.
(block_statement . (identifier) @name) @definition.section

; `{% set title = "Home" %}`
(set_statement (assignment_expression left: (identifier) @name)) @definition.variable

; `{{ macros::input(name="q") }}`
(call_expression name: (identifier) @name) @reference.call
