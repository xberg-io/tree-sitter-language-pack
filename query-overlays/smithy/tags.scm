(namespace_statement
  (namespace) @name) @definition.module

(service_statement
  . (identifier) @name) @definition.interface

(resource_statement
  . (identifier) @name) @definition.class

(operation_statement
  . (identifier) @name) @definition.method

(structure_statement
  . (identifier) @name) @definition.class

(union_statement
  . (identifier) @name) @definition.class

(enum_statement
  . (identifier) @name) @definition.enum

(enum_member
  (enum_field) @name) @definition.constant

; Aggregate (`list`, `map`, `set`) and simple (`string CityId`) shapes.
(list_statement
  . (identifier) @name) @definition.type

(map_statement
  . (identifier) @name) @definition.type

(set_statement
  . (identifier) @name) @definition.type

(simple_shape_statement
  (identifier) @name) @definition.type

; Trait applications (`@required`) name trait shapes, not data shapes: claimed without a tag.
(trait_statement
  (shape_id
    (root_shape_id
      [
        (identifier) @ignore
        (absolute_root_shape_id (identifier) @ignore)
      ])))

; Shapes named by members, operation inputs and outputs, and service and resource bindings.
(shape_id
  (root_shape_id
    [
      (identifier) @name
      (absolute_root_shape_id (identifier) @name)
    ])) @reference.type
