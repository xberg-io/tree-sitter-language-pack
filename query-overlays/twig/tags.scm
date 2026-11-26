; Macros, blocks and template variables a Twig template defines, plus function and macro calls.

; `{% macro input(name) %}`
(macro_statement (method) @name) @definition.macro

; `{% block content %}`
((tag_statement (tag) @_tag . (variable) @name) @definition.section
  (#eq? @_tag "block"))

; `{% set title = "Home" %}`
(assignment_statement (variable) @name) @definition.variable

; `{{ input("q") }}`, `{{ forms.input("q") }}`
(function_call (function_identifier) @name) @reference.call
