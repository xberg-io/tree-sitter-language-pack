; Snakemake (`Snakefile`, `.smk`). The vendored tags.scm of osthomas/tree-sitter-snakemake is
; Python's verbatim. This overlay keeps the Python declarations (classes, functions, methods and
; module-level assignments) and adds Snakemake's own: rules and checkpoints (build targets, so
; functions) and `module` blocks. Assignments inside a function body are locals.

; `rule process:` and `checkpoint split:`
[(rule_definition
   name: (identifier) @name)
 (checkpoint_definition
   name: (identifier) @name)] @definition.function

; `module qc:`
(module_definition
  name: (identifier) @name) @definition.module

; `use rule * from qc as qc_*`
(rule_import
  module_name: (identifier) @name) @reference.module

(class_definition
  name: (identifier) @name) @definition.class

(class_definition
  body: (block
    (function_definition
      name: (identifier) @name) @definition.method))

(function_definition
  name: (identifier) @name) @definition.function

(module
  (expression_statement
    (assignment
      left: (identifier) @name) @definition.constant))

(call
  function: [
      (identifier) @name
      (attribute
        attribute: (identifier) @name)
  ]) @reference.call
