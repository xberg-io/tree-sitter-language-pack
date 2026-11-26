; Justfile recipes are its functions; `:=` assignments its variables. Import paths are not
; tagged: the grammar exposes them only as a quoted string token.

; `build profile="dev":`
(recipe (recipe_header name: (identifier) @name)) @definition.function

; `version := "1.2.0"` and `export RUST_LOG := "info"`
(assignment left: (identifier) @name) @definition.variable

; `alias b := build`
(alias left: (identifier) @name) @definition.alias

(alias right: (identifier) @name) @reference.call

; `mod docs`
(module name: (identifier) @name) @definition.module

; `test: build` and `release: (build "release")`
(dependency name: (identifier) @name) @reference.call

(dependency_expression name: (identifier) @name) @reference.call

; `env_var_or_default("RUST_LOG", "info")`
(function_call name: (identifier) @name) @reference.call
