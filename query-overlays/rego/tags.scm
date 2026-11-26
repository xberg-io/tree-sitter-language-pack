(module
  (package)
  . (ref) @name) @definition.module

; A rule may be defined by several clauses (`default allow`, then `allow if {...}` twice); it is
; tagged at its first clause, and a clause right after one of the same name is claimed untagged.
((rule
  (rule_head
    . (var) @_first))
  .
  (rule
    (rule_head
      . (var) @ignore))
  (#eq? @_first @ignore))

; Rules with arguments are functions; the rest define constant values, sets
; and objects.
(rule
  (rule_head
    . (var) @name
    (rule_args))) @definition.function

(rule
  (rule_head
    . (var) @name)) @definition.constant

; `count(x)`, `time.now_ns()`: the last segment names the called function.
(expr_call
  func_name: (fn_name (var) @name .)) @reference.call
