; Based on queries/tags.scm of https://github.com/WardBrian/tree-sitter-stan (grammars/stan),
; plus the variables a program block declares. `print`, `reject` and `fatal_error` are statements
; of the language, not calls of a named function, and are not tagged.

(functions "functions" @name) @definition.module
(data "data" @name) @definition.module
(transformed_data "transformed data" @name) @definition.module
(parameters "parameters" @name) @definition.module
(transformed_parameters "transformed parameters" @name) @definition.module
(model "model" @name) @definition.module
(generated_quantities "generated quantities" @name) @definition.module

(function_declarator
  name: (identifier) @name) @definition.function

; `int<lower=0> N;` and `real y_mean = mean(y);` at the top of a block. Variables declared in
; `model` or inside a function body are locals.
[
  (top_var_decl_no_assign
    name: (identifier) @name)
  (top_var_decl
    name: (identifier) @name)
] @definition.variable

(function_expression
  name: (identifier) @name) @reference.call

; `normal_lpdf(y | mu, sigma)`
(distr_expression
  name: (identifier) @name) @reference.call

; `y ~ normal(mu, sigma)` calls the `normal` distribution's density.
(sampling_statement
  name: (identifier) @name) @reference.call
