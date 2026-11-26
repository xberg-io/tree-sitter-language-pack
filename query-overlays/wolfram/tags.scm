; ~keep This grammar classifies symbols by spelling, not by meaning: a capitalized symbol is a
; ~keep builtin_symbol even when the file defines it, so both kinds name definitions.

; Functions: top-level `f[args] := body` or `f[args] = body`
(source_file
  (set_delayed_top
    .
    (apply
      .
      [
        (user_symbol)
        (builtin_symbol)
      ] @name)) @definition.function)

(source_file
  (set_top
    .
    (apply
      .
      [
        (user_symbol)
        (builtin_symbol)
      ] @name)) @definition.function)

; Variables: top-level `x = value` or `x := value`
(source_file
  (set_top
    .
    [
      (user_symbol)
      (builtin_symbol)
    ] @name) @definition.variable)

(source_file
  (set_delayed_top
    .
    [
      (user_symbol)
      (builtin_symbol)
    ] @name) @definition.variable)

; Calls: `f[args]`
(apply
  .
  [
    (user_symbol)
    (builtin_symbol)
  ] @name) @reference.call
