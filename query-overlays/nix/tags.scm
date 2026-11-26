; Nix (`.nix`). Based on the tags.scm of nix-community/tree-sitter-nix, which tagged only
; function bindings and two-argument calls. Tags the bindings of the file's top-level `let` (a
; function when the bound value is a lambda, otherwise a variable) and the attributes of attribute
; sets and `rec` sets (a function when the value is a lambda, otherwise a field). The name is the
; last attribute of the binding's path, so `meta.description = ...` is `description`. Attribute
; sets passed as call arguments (`mkDerivation { pname = ...; }`) configure the callee rather than
; declare anything and are claimed without a tag, as are function parameters and bindings of a
; `let` nested inside a function.

; `mkDerivation { pname = "app"; meta = { description = "demo"; }; }`
(apply_expression
  argument: [(attrset_expression
               (binding_set
                 (binding
                   attrpath: (attrpath (identifier) @ignore .))))
             (rec_attrset_expression
               (binding_set
                 (binding
                   attrpath: (attrpath (identifier) @ignore .))))])

(apply_expression
  argument: (_
    (binding_set
      (binding
        expression: (_
          (binding_set
            (binding
              attrpath: (attrpath (identifier) @ignore .))))))))

; `greet = name: "Hello ${name}";` in the top-level `let`
[(source_code
   expression: (let_expression
     (binding_set
       (binding
         attrpath: (attrpath (identifier) @name .)
         expression: (function_expression)) @definition.function)))
 (source_code
   expression: (function_expression
     body: (let_expression
       (binding_set
         (binding
           attrpath: (attrpath (identifier) @name .)
           expression: (function_expression)) @definition.function))))]

; `version = "1.2.0";` in the top-level `let`
[(source_code
   expression: (let_expression
     (binding_set
       (binding
         attrpath: (attrpath (identifier) @name .)) @definition.variable)))
 (source_code
   expression: (function_expression
     body: (let_expression
       (binding_set
         (binding
           attrpath: (attrpath (identifier) @name .)) @definition.variable))))]

; `format = text: lib.toLower text;` in an attribute set
[(attrset_expression
   (binding_set
     (binding
       attrpath: (attrpath (identifier) @name .)
       expression: (function_expression)) @definition.function))
 (rec_attrset_expression
   (binding_set
     (binding
       attrpath: (attrpath (identifier) @name .)
       expression: (function_expression)) @definition.function))]

; `hello = greet "world";` and `meta.description = "demo";` in an attribute set
[(attrset_expression
   (binding_set
     (binding
       attrpath: (attrpath (identifier) @name .)) @definition.field))
 (rec_attrset_expression
   (binding_set
     (binding
       attrpath: (attrpath (identifier) @name .)) @definition.field))]

; `greet "world"`, `wrap "!" "x"` (the innermost application) and `lib.toLower text`
(apply_expression
  function: [(variable_expression
               name: (identifier) @name)
             (select_expression
               attrpath: (attrpath (identifier) @name .))]) @reference.call
