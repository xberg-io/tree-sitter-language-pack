; G-code (LinuxCNC/RS274NGC). An `o100 sub` ... `o100 endsub` block defines a subroutine and
; `o100 call` calls it. Only the first label names the subroutine: the label repeated on `endsub`
; closes it.
;
; Named subroutines (`o<square> sub`) are not tagged: the grammar lexes `<square>` as one
; property_name token with the angle brackets included and has no node for the bare name, so
; their names would carry the brackets.

(subroutine_definition
  .
  (label
    label_type: (direct_label
      (number) @name))) @definition.function

(subroutine_call
  (label
    label_type: (direct_label
      (number) @name))) @reference.call
