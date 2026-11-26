(typedef_definition
  (typedef_identifier) @name) @definition.type

(const_definition
  (identifier) @name) @definition.constant

(enum_definition
  type: (identifier) @name) @definition.enum

(enum_definition
  (identifier) @name @definition.constant)

(struct_definition
  type: (identifier) @name) @definition.class

(union_definition
  type: (identifier) @name) @definition.class

(exception_definition
  . (identifier) @name) @definition.class

(service_definition
  . type: (identifier) @name) @definition.interface

(function_definition
  (identifier) @name) @definition.method

(type
  type: (identifier) @name) @reference.type
