; `function name(params) { ... }`, also `func`; a gawk namespaced name (`ns::name`) is one tag.
(func_def name: (_) @name) @definition.function

; `name(args)`: calls of user functions. Builtins such as `sprintf` parse as their own
; nodes, not `func_call`, and are not tagged.
(func_call name: (_) @name) @reference.call
