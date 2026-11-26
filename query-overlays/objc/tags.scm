; Objective-C (`.m`). Classes, protocols, methods, properties and the C declarations around them:
; functions, structs, enums, typedefs and macros. Declarations inside method and function bodies
; are locals.

; `@interface Shape : NSObject` and `@implementation Shape`; the superclass and a category name
; are not the class's name.
(class_interface
  "@interface"
  .
  (identifier) @name) @definition.class

(class_implementation
  "@implementation"
  .
  (identifier) @name) @definition.class

; `@protocol Drawable`
(protocol_declaration
  "@protocol"
  .
  (identifier) @name) @definition.interface

; `- (float)area;` and `- (float)area { ... }`: the first selector part names the method.
(method_declaration
  (method_type)
  .
  (identifier) @name) @definition.method

(method_definition
  (method_type)
  .
  (identifier) @name) @definition.method

; `@property (nonatomic) NSInteger sides;`
(property_declaration
  (struct_declaration
    (struct_declarator
      (identifier) @name))) @definition.property

(function_definition
  declarator: (function_declarator
                declarator: (identifier) @name)) @definition.function

(struct_specifier
  name: (type_identifier) @name
  body: (_)) @definition.struct

(enum_specifier
  name: (type_identifier) @name
  body: (_)) @definition.enum

; `typedef struct { ... } Point2D;`
(type_definition
  declarator: (type_identifier) @name) @definition.type

; `#define MAX_SIDES 8`
[(preproc_def name: (identifier) @name)
 (preproc_function_def name: (identifier) @name)] @definition.macro

; `[self area]` sends the message `area`; `NSLog(...)` is a C call.
(message_expression
  method: (identifier) @name) @reference.call

(call_expression
  function: (identifier) @name) @reference.call
