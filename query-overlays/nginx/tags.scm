; nginx's named blocks are its virtual servers, named by their `server_name` values, and `set`
; declares a variable. Locations are not tagged: the grammar lexes a location's route together
; with the space before `{`, and it does not parse `upstream` or `map` blocks.

; `server { server_name example.com www.example.com; }`, one tag per name
(attribute
  (keyword) @_server
  (block
    (attribute
      (keyword) @_directive
      (value) @name) @definition.module)
  (#eq? @_server "server")
  (#eq? @_directive "server_name"))

; `set $app_root /srv/www;`
(attribute
  (keyword) @_directive
  .
  (value) @name
  (#eq? @_directive "set")) @definition.variable
