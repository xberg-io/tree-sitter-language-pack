; Apex, written from tree-sitter-sfapex (github.com/aheber/tree-sitter-sfapex, apex/), starting
; from its vendored apex/queries/tags.scm. Locals and parameters are not definitions.

(class_declaration
  name: (identifier) @name) @definition.class

(interface_declaration
  name: (identifier) @name) @definition.interface

(enum_declaration
  name: (identifier) @name) @definition.enum

(enum_constant
  name: (identifier) @name) @definition.constant

; Overloads name one method or constructor: the first declaration in a body tags, later ones are
; claimed.
([
  (class_body
    (method_declaration
      name: (identifier) @_first)
    (method_declaration
      name: (identifier) @ignore))
  (interface_body
    (method_declaration
      name: (identifier) @_first)
    (method_declaration
      name: (identifier) @ignore))
  (class_body
    (constructor_declaration
      name: (identifier) @_first)
    (constructor_declaration
      name: (identifier) @ignore))
]
  (#eq? @_first @ignore))

(method_declaration
  name: (identifier) @name) @definition.method

(constructor_declaration
  name: (identifier) @name) @definition.constructor

; `static final` fields are constants; other fields and properties (`String name { get; set; }`)
; are fields. The grammar's `constant_declaration` (a Java interface field) does not tag: Apex
; interfaces cannot declare fields.
(field_declaration
  (modifiers
    [
      (modifier
        (static))
      (modifier
        (final))
    ]
    .
    [
      (modifier
        (static))
      (modifier
        (final))
    ])
  declarator: (variable_declarator
    name: (identifier) @name)) @definition.constant

(field_declaration
  declarator: (variable_declarator
    name: (identifier) @name)) @definition.field

; `extends BaseGreeter implements Speaker, Comparable`
(superclass
  (type_identifier) @name) @reference.implementation

(interfaces
  (type_list
    (type_identifier) @name)) @reference.implementation

; `new Envelope(...)`
(object_creation_expression
  type: (type_identifier) @name) @reference.class

(method_invocation
  name: (identifier) @name) @reference.call
