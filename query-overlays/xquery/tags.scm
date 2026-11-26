; Prolog declarations are the declarations: a library module's namespace prefix, functions and
; global variables. The name is the local part of a prefixed name (`area` of `geo:area`).
; `let` and `for` bindings in expressions are local.

(module_declaration
  (identifier) @name) @definition.module

(function_declaration
  [
    local: (identifier) @name
    ncname: (identifier) @name
  ]) @definition.function

(variable_declaration
  (variable
    [
      local: (identifier) @name
      ncname: (identifier) @name
    ])) @definition.variable
