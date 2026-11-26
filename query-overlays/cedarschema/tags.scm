; Cedar schemas. Based on the cedarschema tags.scm of github.com/DuskSystems/tree-sitter-cedar,
; which captured quoted action names with their quotes, used a role outside the tag vocabulary
; (definition.namespace) and tagged entity types as plain types. Actions are tagged as functions
; because Cedar policies call them (`Action::"view"`, see query-overlays/cedar/tags.scm).

(namespace
  (name) @name) @definition.module

; `entity Color enum ["Red", "Green"];` enumerates its entities.
(entity_declaration
  (identifier_list (identifier) @name)
  (enum_type)) @definition.enum

; `entity User in [Group] { ... };` and `entity Album, Photo;`
(entity_declaration
  (identifier_list (identifier) @name)) @definition.class

(common_type_declaration
  .
  (identifier) @name) @definition.type

(attribute_declaration
  .
  [
    (identifier) @name
    (string (string_content) @name)
  ]) @definition.field

; `action "view" ...;` and `action share, edit ...;`
(action_declaration
  (action_name_list
    [
      (identifier) @name
      (string (string_content) @name)
    ])) @definition.function

; `in ["view"]` makes the action a member of the action group `view`.
(action_parents
  (qualified_name_list
    (qualified_name (string (string_content) @name) .))) @reference.call

; Type uses, without the primitive and extension types.
((name (identifier) @name .)
  (#not-match? @name "^(String|Long|Bool|Boolean|Set|Record|Entity|Extension|ipaddr|decimal|datetime|duration)$")) @reference.type
