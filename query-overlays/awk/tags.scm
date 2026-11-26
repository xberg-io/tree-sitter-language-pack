; `function name(params) { ... }`, also `func`; a gawk namespaced name (`ns::name`) is one tag.
(func_def name: (_) @name) @definition.function

; `name(args)`, including calls to builtins such as `sprintf`.
(func_call name: (_) @name) @reference.call
