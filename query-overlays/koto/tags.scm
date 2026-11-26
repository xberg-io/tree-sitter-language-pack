; Functions: a function assigned to a name at module level or in an export
(module
  (assign
    lhs: (identifier) @name
    rhs: (function)) @definition.function)

(export
  (assign
    lhs: (identifier) @name
    rhs: (function)) @definition.function)

; Methods: functions stored under a map key, including metakeys such as `@display`
([
  (entry_inline
    key: [
      (identifier) @name
      (meta
        id: (meta_id) @name)
    ]
    value: (function))
  (entry_block
    key: [
      (identifier) @name
      (meta
        id: (meta_id) @name)
    ]
    value: (function))
] @definition.method)

; Module-level values
(module
  (assign
    lhs: (identifier) @name) @definition.variable)

(export
  (assign
    lhs: (identifier) @name) @definition.variable)

; Calls: `f x`, `f(x)` and `x.f()`
(call
  function: (identifier) @name) @reference.call

(chain
  start: (identifier) @name
  .
  (call_args)) @reference.call

(chain
  (lookup
    (identifier) @name)
  .
  (call_args)) @reference.call
