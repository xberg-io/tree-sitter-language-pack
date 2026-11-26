; Go (`.go`). Based on the tags.scm of tree-sitter/tree-sitter-go, reworked so that every
; declaration form carries a role: the package, functions, methods, struct and interface types,
; type aliases and defined types, constants (including `iota` groups), package-level variables,
; interface method specs and struct fields. Constants, variables and types declared inside a
; function body are locals. Type references are limited to positions that name a declared type
; (qualified types, composite literals, method receivers), so builtins such as `int` and type
; parameters such as `T` are not tagged.

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

; `fmt.Stringer`, `Point{X: 1}` and the receiver type of a method
[(qualified_type name: (type_identifier) @name)
 (composite_literal type: (type_identifier) @name)
 (method_declaration
   receiver: (parameter_list
     (parameter_declaration
       type: [(type_identifier) @name
              (pointer_type (type_identifier) @name)])))] @reference.type
