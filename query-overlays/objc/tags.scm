; Objective-C (`.m`), written from tree-sitter-objc (github.com/tree-sitter-grammars/tree-sitter-objc).
; Classes, protocols, methods, properties and the C declarations around them: functions, structs,
; enums, typedefs, macros and file-scope variables. Declarations inside method and function bodies
; are locals.
; ~keep A class is declared by `@interface` and defined by `@implementation`, and a method is
; ~keep declared in the interface and defined in the implementation: each tags once, the class at
; ~keep its `@interface` and the method at its definition. Protocol methods have no definition and
; ~keep tag at their declaration. A category (`@interface Shape (Scaling)`) extends a class
; ~keep declared elsewhere, so its name is not a class definition; its methods still tag.

(class_interface
  "@interface"
  .
  (identifier) @ignore
  .
  category: (identifier))

; `@interface Shape : NSObject <Drawable>`; the superclass is not the class's name.
(class_interface
  "@interface"
  .
  (identifier) @name) @definition.class

(class_interface
  superclass: (identifier) @name) @reference.implementation

(class_interface
  (parameterized_arguments
    (type_name
      (type_identifier) @name))) @reference.implementation

; `@protocol Drawable <NSObject>`
(protocol_declaration
  "@protocol"
  .
  (identifier) @name) @definition.interface

(protocol_declaration
  (protocol_reference_list
    (identifier) @name)) @reference.implementation

(protocol_declaration
  (method_declaration
    (method_type)
    .
    (identifier) @name) @definition.method)

; `- (float)area { ... }`: the first selector part names the method.
(method_definition
  (method_type)
  .
  (identifier) @name) @definition.method

; `@property (nonatomic) NSInteger sides;` and `@property NSString *label;`
(property_declaration
  (struct_declaration
    (struct_declarator
      [
        (identifier) @name
        (pointer_declarator
          declarator: (identifier) @name)
      ]))) @definition.field

(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition.function

(struct_specifier
  name: (type_identifier) @name
  body: (_)) @definition.class

(enum_specifier
  name: (type_identifier) @name
  body: (_)) @definition.enum

(enumerator
  name: (identifier) @name) @definition.constant

; `typedef struct { ... } Point2D;`
(type_definition
  declarator: (type_identifier) @name) @definition.type

; `#define MAX_SIDES 8`
[
  (preproc_def
    name: (identifier) @name)
  (preproc_function_def
    name: (identifier) @name)
] @definition.macro

; File-scope variables; `const` ones are constants.
(translation_unit
  (declaration
    (type_qualifier) @_qualifier
    declarator: (init_declarator
      declarator: (identifier) @name)) @definition.constant
  (#eq? @_qualifier "const"))

(translation_unit
  (declaration
    declarator: [
      (identifier) @name
      (pointer_declarator
        declarator: (identifier) @name)
      (init_declarator
        declarator: [
          (identifier) @name
          (pointer_declarator
            declarator: (identifier) @name)
        ])
    ]) @definition.variable)

; `[self area]` sends the message `area`; `NSLog(...)` is a C call.
(message_expression
  method: (identifier) @name) @reference.call

(call_expression
  function: (identifier) @name) @reference.call
