; Snippets are the named declarations of Svelte markup; `{@render name()}` calls them. A tag
; named with a capital letter (`<Badge />`) instantiates a component. Script declarations belong
; to the injected JavaScript or TypeScript, so markup holds no other definitions.

(snippet_start_expr (snippet_name) @name) @definition.function

(render_expr (snippet_name) @name) @reference.call

([
  (start_tag (tag_name) @name)
  (self_closing_tag (tag_name) @name)
] @reference.class
  (#match? @name "^[A-Z]"))
