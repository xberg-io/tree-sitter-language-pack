; Named templates of a Go template file. A name is the quoted string literal the grammar exposes.

; `{{ define "header" }}`
(define_action name: (_) @name) @definition.function

; `{{ block "content" . }}` defines a template and executes it in place.
(block_action name: (_) @name) @definition.section

; `{{ template "header" . }}`
(template_action name: (_) @name) @reference.call
