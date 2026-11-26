; Snippets are the named declarations of Svelte markup; `{@render name()}` calls them. Script
; declarations belong to the injected JavaScript or TypeScript.

(snippet_start_expr (snippet_name) @name) @definition.function

(render_expr (snippet_name) @name) @reference.call
