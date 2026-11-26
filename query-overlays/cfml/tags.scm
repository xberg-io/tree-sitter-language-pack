; CFML components (`.cfc`) in tag syntax. Based on queries/tags.scm of
; https://github.com/cfmleditor/tree-sitter-cfml (the `cfml` grammar). The script-syntax patterns
; of that query (`function_declaration`, `method_definition`, named `function_expression`) are
; left out: this grammar keeps `<cfscript>` and `component { ... }` bodies as opaque content for
; injection, so script functions only appear nested in an expression's function literal, where
; they are locals. Its `cf_tag` pattern tagged every tag name as a definition and is left out too.
; `<cfproperty>` is not tagged: the grammar hides the tag name, so it cannot be told apart from
; `<cfargument>`.

; `<cfcomponent name="Greeter">`
(cf_component_open_tag
  (cf_tag_attributes
    (cf_attribute
      (cf_attribute_name) @_name
      (quoted_cf_attribute_value
        (attribute_value) @name)))
  (#eq? @_name "name")) @definition.class

; `<cfcomponent extends="Base">`
(cf_component_open_tag
  (cf_tag_attributes
    (cf_attribute
      (cf_attribute_name) @_name
      (quoted_cf_attribute_value
        (attribute_value) @name)))
  (#eq? @_name "extends")) @reference.class

; `<cffunction name="greet">`
(cf_function_tag
  (cf_attribute
    (cf_attribute_name) @_name
    (quoted_cf_attribute_value
      (attribute_value) @name))
  (#eq? @_name "name")) @definition.function

; Calls in tag expressions: `<cfreturn ucase(msg)>` and `<cfset x = this.greet("a")>`
(call_expression
  function: (identifier) @name) @reference.call

(call_expression
  function: (member_expression
    property: (property_identifier) @name)) @reference.call
