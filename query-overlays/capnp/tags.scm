(const
  (const_identifier) @name) @definition.constant

(struct
  . (type_identifier) @name) @definition.class

(enum
  . (enum_identifier) @name) @definition.enum

(enum_field
  (enum_member) @name) @definition.constant

(interface
  . (type_identifier) @name) @definition.interface

(method
  (method_identifier) @name) @definition.method

(custom_type
  (type_identifier) @name .) @reference.type

(named_return_type
  (type_identifier) @name) @reference.type
