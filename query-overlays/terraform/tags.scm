; Terraform (`*.tf`, `*.tfvars`) parses to the same block and attribute shapes as `hcl`; this is
; query-overlays/hcl/tags.scm without its Terragrunt-only blocks (`dependency`, `include`, `inputs`),
; which a Terraform file cannot contain. Keep the shared patterns in step with the hcl overlay.
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

; `module "<name>" {}`
(config_file
  (body
    (block
      . (identifier) @_kind
      . [(string_lit (template_literal) @name) (identifier) @name]) @definition.module
    (#eq? @_kind "module")))

; `provider "<name>" {}`
(config_file
  (body
    (block
      . (identifier) @_kind
      . [(string_lit (template_literal) @name) (identifier) @name]) @definition.module
    (#eq? @_kind "provider")))

; `terraform {}`, which has no label of its own and is named by the block type.
(config_file
  (body
    (block . (identifier) @name) @definition.module
    (#eq? @name "terraform")))

; Each entry of a `locals {}` block.
(config_file
  (body
    (block
      . (identifier) @_kind
      (body (attribute . (identifier) @name) @definition.variable))
    (#eq? @_kind "locals")))
