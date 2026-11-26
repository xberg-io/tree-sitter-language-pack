; Functions and variables a Vento template defines.

; `{{ function greet(name) }}`
(function_tag_start (identifier) @name) @definition.function

; `{{ set title = "Home" }}`
(set_tag (identifier) @name) @definition.variable
