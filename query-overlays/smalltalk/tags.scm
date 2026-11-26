; ~keep This grammar parses a single method (or a list of statements), not a class definition
; ~keep or a file-out of several methods: on such input it recovers by reading the first word as
; ~keep a unary selector (`Object subclass: #Counter` becomes method `Object`), so a method with
; ~keep an ERROR child is not tagged. A keyword selector such as `at:put:` is split into one node
; ~keep per keyword with the arguments in between, so no single node holds the whole selector:
; ~keep keyword methods and sends are tagged by their first keyword (`at:`).

; A method whose parse needed error recovery is not a method.
(method
  [
    (unary_selector (unary_identifier) @ignore)
    (binary_selector (binary_operator) @ignore)
    (keyword_selector . (keyword) @ignore)
  ]
  (ERROR))

; The method definition: `increment`, `+ other`, `at: index put: value`. One file holds one
; method, so the three selector forms share a pattern.
(method
  [
    (unary_selector (unary_identifier) @name)
    (binary_selector (binary_operator) @name)
    (keyword_selector . (keyword) @name)
  ]) @definition.method

; Message sends: `self limit`, `x foo; bar`, `count min: 1 max: 9`.
(unary_message
  (unary_identifier) @name) @reference.call

(cascaded_unary_message
  (unary_identifier) @name) @reference.call

(keyword_message
  receiver: (_)
  .
  (keyword) @name) @reference.call

(cascaded_keyword_message
  .
  (keyword) @name) @reference.call
