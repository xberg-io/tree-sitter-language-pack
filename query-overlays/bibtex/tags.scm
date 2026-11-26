; Bibliography entries, named by the citation key, and `@string` abbreviations, plus the
; references to them inside field values.

; `@book{knuth84, ...}`
(entry key: (_) @name) @definition.label

; `@string{acm = "ACM Press"}`
(string name: (identifier) @name) @definition.constant

; `crossref = {knuth84}`
((field name: (identifier) @_field value: (value (token (brace_word) @name))) @reference.label
  (#match? @_field "^[Cc][Rr][Oo][Ss][Ss][Rr][Ee][Ff]$"))

; `publisher = acm`
(field value: (value (token (identifier) @name))) @reference.constant
