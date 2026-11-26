; ~keep This grammar parses one method (or a list of statements), and a keyword selector such
; ~keep as `at:put:` is split into one node per keyword with the arguments in between, so no
; ~keep single node holds its name. Only unary, binary and one-keyword selectors are tagged.

; Method definitions
(method
  (unary_selector
    (unary_identifier) @name)) @definition.method

(method
  (binary_selector
    (binary_operator) @name)) @definition.method

(method
  (keyword_selector
    .
    (keyword) @name
    .
    (identifier) .)) @definition.method

; Message sends
(unary_message
  (unary_identifier) @name) @reference.call

(cascaded_unary_message
  (unary_identifier) @name) @reference.call

(keyword_message
  receiver: (_)
  .
  (keyword) @name
  .
  (_) .) @reference.call
