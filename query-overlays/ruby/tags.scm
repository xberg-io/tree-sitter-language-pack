; Class definitions
(class name: [(constant) (scope_resolution name: (constant) @name)] @name) @definition.class

; Module definitions
(module name: [(constant) (scope_resolution name: (constant) @name)] @name) @definition.module

; Method definitions
(method name: (identifier) @name) @definition.method
(singleton_method name: (identifier) @name) @definition.method

; Class inheritance: `class Foo < Bar`
; The `superclass` node wraps the parent constant.
(class
  superclass: (superclass (constant) @name)) @reference.implementation

; Method calls
(call method: (identifier) @name) @reference.call
