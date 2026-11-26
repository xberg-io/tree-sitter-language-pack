; Shared verbatim by `hcl` (`*.hcl`, Terragrunt) and `terraform` (`*.tf`, `*.tfvars`), which parse
; to the same block and attribute shapes; keep query-overlays/{hcl,terraform}/tags.scm identical.
; Only top-level blocks are declarations.

; `resource "<type>" "<name>" {}` and `data "<type>" "<name>" {}`, named by the second label.
; A label is a quoted string or a bare identifier (`resource aws_s3_bucket logs {}`).
(config_file
  (body
    (block
      . (identifier) @_kind
      . [(string_lit) (identifier)]
      . [(string_lit (template_literal) @name) (identifier) @name]) @definition.class
    (#any-of? @_kind "resource" "data")))

; `variable "<name>" {}`
(config_file
  (body
    (block
      . (identifier) @_kind
      . [(string_lit (template_literal) @name) (identifier) @name]) @definition.variable
    (#eq? @_kind "variable")))

; `output "<name>" {}`
(config_file
  (body
    (block
      . (identifier) @_kind
      . [(string_lit (template_literal) @name) (identifier) @name]) @definition.constant
    (#eq? @_kind "output")))

; `module "<name>" {}`, and Terragrunt `dependency "<name>" {}` and `include "<name>" {}`.
(config_file
  (body
    (block
      . (identifier) @_kind
      . [(string_lit (template_literal) @name) (identifier) @name]) @definition.module
    (#any-of? @_kind "module" "dependency" "include")))

; `provider "<name>" {}`
(config_file
  (body
    (block
      . (identifier) @_kind
      . [(string_lit (template_literal) @name) (identifier) @name]) @definition.module
    (#eq? @_kind "provider")))

; Blocks without a label of their own, named by the block type: `terraform {}` and Terragrunt
; `include {}`.
(config_file
  (body
    (block . (identifier) @name) @definition.module
    (#eq? @name "terraform")))

(config_file
  (body
    (block . (identifier) @name . (block_start)) @definition.module
    (#eq? @name "include")))

; Each entry of a `locals {}` block.
(config_file
  (body
    (block
      . (identifier) @_kind
      (body (attribute . (identifier) @name) @definition.variable))
    (#eq? @_kind "locals")))

; Terragrunt `inputs = { ... }`.
(config_file
  (body
    (attribute . (identifier) @name) @definition.variable
    (#eq? @name "inputs")))
