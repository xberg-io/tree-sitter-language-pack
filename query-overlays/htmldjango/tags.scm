; Blocks are the named, overridable sections of a Django template.

; `{% block content %}`; the variable after `endblock` repeats the name.
((paired_statement (tag_name) @_tag . (variable (variable_name) @name)) @definition.section
  (#eq? @_tag "block"))
