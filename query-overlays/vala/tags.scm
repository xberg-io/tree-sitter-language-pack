; Vala (`.vala`, `.vapi`). Namespaces, classes, interfaces, structs, enums, delegates, methods,
; constructors, signals, fields, properties and namespace-level constants. Variables declared
; inside a method body are locals.

; `namespace Demo { ... }`
(namespace
  .
  (identifier) @name) @definition.module

; `public class Greeter : Object, Drawable`: the name follows the keyword, the base types follow
; the colon.
(class_declaration
  "class"
  .
  (identifier) @name) @definition.class

(interface_declaration
  "interface"
  .
  (identifier) @name) @definition.interface

(struct_declaration
  "struct"
  .
  (identifier) @name) @definition.class

(enum_declaration
  name: (identifier) @name) @definition.enum

; `public delegate int Transform (int value);`
(function_definition
  (modifier) @_modifier
  name: (identifier) @name
  (#eq? @_modifier "delegate")) @definition.type

; Constructors: `public Greeter (string name) { ... }`
(class_constructor_definition
  name: (identifier) @name) @definition.constructor

; Methods, abstract methods and signals of a class, interface or struct
[(class_declaration
   (block
     (function_definition
       name: (identifier) @name) @definition.method))
 (interface_declaration
   (block
     (function_definition
       name: (identifier) @name) @definition.method))
 (struct_declaration
   (block
     (function_definition
       name: (identifier) @name) @definition.method))]

; `public static int main (string[] args) { ... }` outside any type
(function_definition
  name: (identifier) @name) @definition.function

; `public string name { get; set; }`
[(class_declaration
   (block
     (declaration
       type_name: (_)
       .
       (identifier) @name
       (gobject_property_acces)) @definition.field))
 (interface_declaration
   (block
     (declaration
       type_name: (_)
       .
       (identifier) @name
       (gobject_property_acces)) @definition.field))]

; `private int count = 0;` in a class or struct body
[(class_declaration
   (block
     (declaration
       type_name: (_)
       .
       (identifier) @name) @definition.field))
 (struct_declaration
   (block
     (declaration
       type_name: (_)
       .
       (identifier) @name) @definition.field))]

; `public const int MAX = 8;` at namespace or file scope
[(source_file
   (declaration
     type_name: (_)
     .
     (identifier) @name) @definition.constant)
 (namespace
   (block
     (declaration
       type_name: (_)
       .
       (identifier) @name) @definition.constant))]

; `print (...)` and `greeter.draw ()`
[(function_call identifier: (identifier) @name)
 (member_function identifier: (identifier) @name)] @reference.call
