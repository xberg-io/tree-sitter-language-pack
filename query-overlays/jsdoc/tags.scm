; JSDoc. `@typedef` and `@callback` declare types that exist only in doc comments; every other
; tag documents a symbol the host JavaScript grammar already tags. `@type {Name}` and `{Name}`
; type expressions are not tagged as references: the grammar gives the whole braced expression
; as one `type` node, so no clean name can be captured.

(tag
  (tag_name) @_tag
  (identifier) @name
  (#eq? @_tag "@typedef")) @definition.type

(tag
  (tag_name) @_tag
  (identifier) @name
  (#eq? @_tag "@callback")) @definition.type
