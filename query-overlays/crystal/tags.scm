; Crystal (`.cr`). Modules, classes, structs, enums, aliases, constants and methods. A `def`
; inside a type or module body is a method; a `def` at top level is a function. Local variables
; assigned inside a method are not tagged.

(module_def
  name: [(constant) @name
         (generic_type . (constant) @name)]) @definition.module

(class_def
  name: [(constant) @name
         (generic_type . (constant) @name)]) @definition.class

(struct_def
  name: [(constant) @name
         (generic_type . (constant) @name)]) @definition.struct

(enum_def
  name: (constant) @name) @definition.enum

; `alias Coord = Tuple(Int32, Int32)`
(alias
  name: (constant) @name) @definition.type

; `annotation Experimental; end`
(annotation_def
  name: (constant) @name) @definition.type

; `MAX_SIDES = 8`
(const_assign
  lhs: (constant) @name) @definition.constant

; `def describe(shape)` at top level
(source_file
  (method_def
    name: (_) @name) @definition.function)

; `def area : Float64`, `def self.wrap(value)` and `abstract def area` in a type or module body
[(method_def name: (_) @name)
 (abstract_method_def name: (_) @name)] @definition.method

; `shape.area`, `puts x` and `new(value)`
(call
  method: (identifier) @name) @reference.call
