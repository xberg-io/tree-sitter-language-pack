; Rust (`.rs`). Based on the tags.scm of grammar-orchard/tree-sitter-rust-orchard, with kinds
; normalized (struct and union are classes, an enum is an enum, a type alias is a type) and the
; forms it left out added: constants and statics, enum variants, struct fields, trait method
; signatures, extern functions and associated constants and types. A function is a method only
; inside an `impl` or `trait` body; a function inside a `mod` body is a function. A `const` or
; `static` inside a function body is a local.

(block
  [(const_item name: (identifier) @ignore)
   (static_item name: (identifier) @ignore)])

; ADT definitions

(struct_item
  name: (type_identifier) @name) @definition.class

(union_item
  name: (type_identifier) @name) @definition.class

(enum_item
  name: (type_identifier) @name) @definition.enum

(enum_variant
  name: (identifier) @name) @definition.constant

(field_declaration
  name: (field_identifier) @name) @definition.field

; `pub type Id = u64;` and an associated `type Unit;`
(type_item
  name: (type_identifier) @name) @definition.type

; `const MAX: u32 = 8;`, including associated constants
(const_item
  name: (identifier) @name) @definition.constant

; `static mut COUNTER: u32 = 0;`
(static_item
  name: (identifier) @name) @definition.variable

; methods: functions and signatures inside an impl or trait body

[(impl_item
   body: (declaration_list
     (function_item
       name: (identifier) @name) @definition.method))
 (trait_item
   body: (declaration_list
     [(function_item
        name: (identifier) @name)
      (function_signature_item
        name: (identifier) @name)] @definition.method))]

; function definitions, and `fn abs(input: i32) -> i32;` inside `extern "C" { ... }`

[(function_item
   name: (identifier) @name)
 (function_signature_item
   name: (identifier) @name)] @definition.function

(trait_item
  name: (type_identifier) @name) @definition.interface

(mod_item
  name: (identifier) @name) @definition.module

(macro_definition
  name: (identifier) @name) @definition.macro

; references

; `add(1, 2)`, `p.area()` and `Point::new(1, 2)`
(call_expression
  function: [(identifier) @name
             (field_expression field: (field_identifier) @name)
             (scoped_identifier name: (identifier) @name)]) @reference.call

; `println!(...)` and `std::write!(...)`
(macro_invocation
  macro: [(identifier) @name
          (scoped_identifier name: (identifier) @name)]) @reference.macro

; implementations: the trait of `impl Shape for Point`, the type of `impl Point`

(impl_item
  trait: [(type_identifier) @name
          (scoped_type_identifier name: (type_identifier) @name)
          (generic_type type: (type_identifier) @name)]) @reference.implementation

(impl_item
  type: [(type_identifier) @name
         (scoped_type_identifier name: (type_identifier) @name)
         (generic_type type: (type_identifier) @name)]
  !trait) @reference.implementation
