; templ (`.templ`). The vendored tags.scm of vrischmann/tree-sitter-templ is a copy of Go's, so
; it tags none of templ's own declarations. This overlay carries the Go declarations the grammar
; exposes (see query-overlays/go/tags.scm) plus templ components, `css` classes and `script`
; functions. A component with a receiver, `templ (i Item) card()`, is a method.

; `templ (i Item) card() { ... }`
(component_declaration
  receiver: (parameter_list)
  name: (component_identifier) @name) @definition.method

; `templ greeting(name string) { ... }`
(component_declaration
  name: (component_identifier) @name) @definition.function

; `css highlight() { ... }` defines a CSS class generator called as a function.
(css_declaration
  name: (css_identifier) @name) @definition.function

; `script logClick(msg string) { ... }`
(script_declaration
  name: (script_identifier) @name) @definition.function

; Locals: `const n = 1`, `var tmp int` and `type inner struct{}` inside a function body.
(block
  (statement_list
    [(const_declaration (const_spec name: (identifier) @ignore))
     (var_declaration (var_spec name: (identifier) @ignore))
     (var_declaration (var_spec_list (var_spec name: (identifier) @ignore)))
     (type_declaration (type_spec name: (type_identifier) @ignore))
     (type_declaration (type_alias name: (type_identifier) @ignore))]))

(block
  (statement_list
    (type_declaration
      (type_spec
        type: (struct_type
          (field_declaration_list
            (field_declaration name: (field_identifier) @ignore)))))))

(package_clause
  (package_identifier) @name) @definition.module

(function_declaration
  name: (identifier) @name) @definition.function

; `func (p *Point) Move(dx int)`
(method_declaration
  name: (field_identifier) @name) @definition.method

(type_spec
  name: (type_identifier) @name
  type: (struct_type)) @definition.class

(type_spec
  name: (type_identifier) @name
  type: (interface_type)) @definition.interface

; `type Celsius float64` and `type ID = string`
[(type_spec name: (type_identifier) @name)
 (type_alias name: (type_identifier) @name)] @definition.type

; `Area() float64` inside an interface type
(method_elem
  name: (field_identifier) @name) @definition.method

(field_declaration
  name: (field_identifier) @name) @definition.field

(const_spec
  name: (identifier) @name) @definition.constant

(var_spec
  name: (identifier) @name) @definition.variable

(import_spec
  path: (interpreted_string_literal
    (interpreted_string_literal_content) @name)) @reference.module

(call_expression
  function: [
    (identifier) @name
    (parenthesized_expression (identifier) @name)
    (selector_expression field: (field_identifier) @name)
    (parenthesized_expression (selector_expression field: (field_identifier) @name))
  ]) @reference.call

; `fmt.Stringer`, `Point{X: 1}` and the receiver type of a method or component
[(qualified_type name: (type_identifier) @name)
 (composite_literal type: (type_identifier) @name)
 (method_declaration
   receiver: (parameter_list
     (parameter_declaration
       type: [(type_identifier) @name
              (pointer_type (type_identifier) @name)])))
 (component_declaration
   receiver: (parameter_list
     (parameter_declaration
       type: [(type_identifier) @name
              (pointer_type (type_identifier) @name)])))] @reference.type
