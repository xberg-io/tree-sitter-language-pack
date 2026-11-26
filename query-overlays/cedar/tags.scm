; Cedar policies. Based on the tags.scm of github.com/DuskSystems/tree-sitter-cedar, which captured
; the quoted policy id and used a role outside the tag vocabulary (definition.policy). A policy
; is a named rule, tagged as a function by its `@id` annotation; a policy without one has no
; name and is not tagged.

(policy
  (annotation
    (identifier) @_annotation
    (string (string_content) @name)
    (#eq? @_annotation "id"))) @definition.function

; `Action::"read"` names an action, which a schema declares.
(entity_reference
  (identifier) @_type
  .
  (string (string_content) @name)
  (#eq? @_type "Action")) @reference.call

; `PhotoApp::User::"guest"` is an entity of type `User`; `principal is PhotoApp::User` names it.
(entity_reference
  (identifier) @name
  .
  (string)
  (#not-eq? @name "Action")) @reference.type

(scope_constraint
  (name (identifier) @name .)) @reference.type
