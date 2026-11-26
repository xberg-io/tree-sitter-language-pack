; A Caddyfile's named blocks are its site blocks, one tag per address they serve. Snippets are
; left out: the grammar only parses a snippet whose first directive shares the line with `{`,
; which Caddy rejects, and it lexes a named matcher's `@name` together with its indentation.

; `example.com, www.example.com { ... }`; an address in a directive (`reverse_proxy host:port`)
; is not a site.
(site_block (site_address) @name) @definition.module
