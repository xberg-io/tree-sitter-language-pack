(service
  service_name: (identifier) @name) @definition.interface

; Methods and events share the method node.
(method
  name: (identifier) @name) @definition.method

(dto
  name: (identifier) @name) @definition.class

(enum
  name: (identifier) @name) @definition.enum

(error_set
  name: (identifier) @name) @definition.enum

(values_block
  (identifier) @name @definition.constant)

(type
  name: (identifier) @name) @reference.type
