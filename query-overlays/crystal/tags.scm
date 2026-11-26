; Crystal (`.cr`). Modules, classes and structs (both classes), enums with their members, aliases,
; annotations, constants, typed instance variables and methods. A `def`
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
         (generic_type . (constant) @name)]) @definition.class

(enum_def
  name: (constant) @name) @definition.enum

; `Red` inside an enum body; the enum's own name is claimed by the pattern above.
(enum_def
  (constant) @name) @definition.constant

; `@label : String = "point"` in a type body. The `@` is part of the name at every use.
(type_declaration
  var: (instance_var) @name) @definition.field

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

; `class Square < Shape`
(class_def
  superclass: [(constant) @name
               (generic_instance_type . (constant) @name)]) @reference.class
