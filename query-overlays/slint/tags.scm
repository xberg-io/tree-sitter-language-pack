; Components, globals, structs, enums and interfaces a Slint file declares, the properties,
; callbacks and functions of a component, plus element instantiations and function calls.

(component_definition name: (user_type_identifier) @name) @definition.class

(global_definition name: (user_type_identifier) @name) @definition.class

(interface_definition name: (_) @name) @definition.interface

(struct_definition name: (user_type_identifier) @name) @definition.struct

(enum_definition name: (user_type_identifier) @name) @definition.enum

(property name: (simple_identifier) @name) @definition.property

(callback name: (simple_identifier) @name) @definition.method

(function_definition name: (simple_identifier) @name) @definition.method

; `Row { ... }` and `inherits Rectangle`
(component type: (user_type_identifier) @name) @reference.class

(component_modifier base_type: (user_type_identifier) @name) @reference.class

(function_call name: (simple_identifier) @name) @reference.call
