; Classes: the script's own `class_name` and inner classes
(class_name_statement
  name: (name) @name) @definition.class

(class_definition
  name: (name) @name) @definition.class

; Methods of an inner class; functions of the script itself
(class_body
  (function_definition
    name: (name) @name) @definition.method)

(function_definition
  name: (name) @name) @definition.function

(constructor_definition
  "_init" @name) @definition.constructor

; Signals and enums. A signal is an event the class emits, tagged as a method like other events.
(signal_statement
  name: (name) @name) @definition.method

(enum_definition
  name: (name) @name) @definition.enum

; Member constants and variables, not function locals
(source
  [
    (const_statement
      name: (name) @name) @definition.constant
    (variable_statement
      name: (name) @name) @definition.variable
  ])

(class_body
  [
    (const_statement
      name: (name) @name) @definition.constant
    (variable_statement
      name: (name) @name) @definition.variable
  ])

; Inheritance
(extends_statement
  (type
    (identifier) @name)) @reference.class

; Calls
(call
  (identifier) @name) @reference.call

(attribute_call
  (identifier) @name) @reference.call

(base_call
  (identifier) @name) @reference.call
