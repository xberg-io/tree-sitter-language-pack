; User-defined functions: `f(x) = ...`
(def_func
  .
  (function
    name: (identifier) @name)) @definition.function

; User variables, arrays and named data blocks
(def_var
  .
  (identifier) @name) @definition.variable

(def_array
  .
  (array
    name: (identifier) @name)) @definition.variable

(def_datablock
  name: (datablock) @name) @definition.variable

; Function calls
(function
  name: (identifier) @name) @reference.call
