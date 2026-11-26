; Classes, enums and typedefs
(decl_class
  typename: (identifier) @name) @definition.class

(decl_enum
  typename: (identifier) @name) @definition.enum

(enum_member
  name: (identifier) @name) @definition.constant

(typedef
  alias: (identifier) @name) @definition.type

; `void InventoryItem(string name)` in `class InventoryItem`: a method named after its class is
; the constructor, claimed before the method pattern below.
((decl_class
  typename: (identifier) @_class
  body: (class_body
    (decl_method
      name: (identifier) @name) @definition.constructor))
  (#eq? @name @_class))

; Methods of a class; free functions at file scope
(class_body
  (decl_method
    name: (identifier) @name) @definition.method)

(compilation_unit
  (decl_method
    name: (identifier) @name) @definition.function)

; Class fields and global variables
(decl_field
  name: (identifier) @name) @definition.field

(compilation_unit
  (decl_variable
    name: (identifier) @name) @definition.variable)

; Inheritance
(superclass
  typename: (identifier) @name) @reference.class

; Calls and instantiation
(invokation
  invoked: (identifier) @name) @reference.call

(new
  type: (type_identifier
    (identifier) @name)) @reference.class
