(entity_declaration
  name: (_) @name) @definition.module

; An architecture implements the entity it names.
(architecture_body
  name: (_) @name) @definition.class

(architecture_body
  entity: (simple_name) @name) @reference.implementation

(package_declaration
  name: (_) @name) @definition.module

(package_instantiation_declaration
  name: (_) @name) @definition.module

(configuration_declaration
  name: (_) @name) @definition.module

(context_declaration
  name: (_) @name) @definition.module

; A component declares the interface of an entity to instantiate.
(component_declaration
  name: (_) @name) @definition.interface

(constant_declaration
  (identifier_list (identifier) @name)) @definition.constant

(full_type_declaration
  name: (_) @name) @definition.type

(subtype_declaration
  name: (_) @name) @definition.type

(enumeration_type_definition
  literal: (identifier) @name) @definition.constant

; Declarations and bodies of functions and procedures.
(function_declaration
  designator: (_) @name) @definition.function

(function_body
  designator: (_) @name) @definition.function

(procedure_declaration
  designator: (_) @name) @definition.function

(procedure_body
  designator: (_) @name) @definition.function

(signal_declaration
  (identifier_list (identifier) @name)) @definition.variable

; `entity work.counter(rtl)` names the last segment; `u : counter` names a component.
(entity_instantiation
  entity: [
    (simple_name) @name
    (selected_name suffix: (simple_name) @name)
  ]) @reference.module

(component_instantiation
  component: [
    (simple_name) @name
    (selected_name suffix: (simple_name) @name)
  ]) @reference.module

(type_mark
  [
    (simple_name) @name
    (selected_name suffix: (simple_name) @name)
  ]) @reference.type

(function_call
  function: (simple_name) @name) @reference.call
