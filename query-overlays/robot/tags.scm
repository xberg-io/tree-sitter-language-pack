; Test cases, user keywords and variables a Robot Framework file defines, plus keyword calls.

(test_case_definition (name) @name) @definition.function

(keyword_definition (name) @name) @definition.function

; `${URL}`, `@{USERS}` and `&{OPTIONS}` are one variable, used with any of the sigils, so the
; name is the part inside the braces.
(variable_definition
  [(scalar_variable (variable_name) @name)
   (list_variable (variable_name) @name)
   (dictionary_variable (variable_name) @name)]) @definition.variable

(keyword_invocation (keyword) @name) @reference.call
