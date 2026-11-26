; `local f(x) = ...` and `local x = ...`.
(bind
  function: (id) @name) @definition.function

(bind
  . (id) @name) @definition.variable

; Object fields that take parameters: `deployment(name):: {...}`. Plain fields are data.
(field
  function: (fieldname (id) @name)) @definition.method

(functioncall
  . [
    (id) @name
    (fieldaccess last: (id) @name)
  ]) @reference.call
