; Eiffel (`.e`). Classes and their features. A feature with an explicit value is a constant, one
; with a routine body (or deferred, external or with arguments) is a method, and one with only a
; type is an attribute. Local entities of a routine are not tagged.

(class_declaration
  (class_name) @name) @definition.class

; `Max_balance: INTEGER = 1000`
(feature_declaration
  (new_feature
    (extended_feature_name
      (identifier) @name))
  (explicit_value)) @definition.constant

; `count: INTEGER attribute Result := 0 end` is an attribute with a default body.
(feature_declaration
  (new_feature
    (extended_feature_name
      (identifier) @name))
  (attribute_or_routine
    (feature_body
      (attribute)))) @definition.field

; `deposit (amount: INTEGER) do ... end`
(feature_declaration
  (new_feature
    (extended_feature_name
      (identifier) @name))
  (attribute_or_routine)) @definition.method

(feature_declaration
  (new_feature
    (extended_feature_name
      (identifier) @name))
  (formal_arguments)) @definition.method

; `balance: INTEGER`
(feature_declaration
  (new_feature
    (extended_feature_name
      (identifier) @name))) @definition.field

; Calls with arguments: `print (balance)` and `account.deposit (10)`. An argument-less call reads
; the same as an attribute access, so it is not tagged.
(unqualified_call
  (identifier) @name
  (actuals)) @reference.call
