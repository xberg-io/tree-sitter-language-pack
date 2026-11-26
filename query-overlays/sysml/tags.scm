; Based on queries/tags.scm of https://gitlab.com/nomograph/tree-sitter-sysml, with the
; references rewritten to the shapes this grammar produces and `enum def` tagged as an enum.

(package_definition
  (identification (name (identifier) @name))) @definition.module

(part_definition
  (identification (name (identifier) @name))) @definition.class

(item_definition
  (identification (name (identifier) @name))) @definition.class

(port_definition
  (identification (name (identifier) @name))) @definition.class

(action_definition
  (identification (name (identifier) @name))) @definition.function

(state_definition
  (identification (name (identifier) @name))) @definition.class

(constraint_definition
  (identification (name (identifier) @name))) @definition.class

(requirement_definition
  (identification (name (identifier) @name))) @definition.class

(attribute_definition
  (identification (name (identifier) @name))) @definition.class

(enumeration_definition
  (identification (name (identifier) @name))) @definition.enum

(connection_definition
  (identification (name (identifier) @name))) @definition.class

(interface_definition
  (identification (name (identifier) @name))) @definition.interface

(allocation_definition
  (identification (name (identifier) @name))) @definition.class

(flow_definition
  (identification (name (identifier) @name))) @definition.class

(metadata_definition
  (identification (name (identifier) @name))) @definition.class

(use_case_definition
  (identification (name (identifier) @name))) @definition.function

(calc_definition
  (identification (name (identifier) @name))) @definition.function

(occurrence_definition
  (identification (name (identifier) @name))) @definition.class

(verification_definition
  (identification (name (identifier) @name))) @definition.function

(analysis_definition
  (identification (name (identifier) @name))) @definition.function

(individual_definition
  (identification (name (identifier) @name))) @definition.class

(concern_definition
  (identification (name (identifier) @name))) @definition.class

(viewpoint_definition
  (identification (name (identifier) @name))) @definition.class

(view_definition
  (identification (name (identifier) @name))) @definition.class

(rendering_definition
  (identification (name (identifier) @name))) @definition.class

(case_definition
  (identification (name (identifier) @name))) @definition.class

(generic_definition
  (identification (name (identifier) @name))) @definition.class

(part_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(attribute_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(port_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(action_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.method

(state_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(constraint_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(requirement_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(connection_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(interface_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(allocation_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(flow_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(calc_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.method

(analysis_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.method

(verification_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.method

(use_case_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.method

(case_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.method

(view_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(rendering_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(item_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(occurrence_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(reference_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(concern_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(viewpoint_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(enumeration_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(generic_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(individual_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(actor_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(stakeholder_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(objective_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(event_occurrence_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(timeslice_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(snapshot_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(variant_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(variation_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(exhibit_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(parameter_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(metadata_usage
  (identification (name (identifier) @name))) @definition.field

(event_usage
  (identification (name (identifier) @name))) @definition.field

(end_usage
  (usage_declaration
    (identification (name (identifier) @name)))) @definition.field

(end_usage
  (identification (name (identifier) @name))) @definition.field

(feature_usage
  (identification (name (identifier) @name))) @definition.field

; A definition that specializes another (`part def Car :> Vehicle`) and the type a usage is typed
; by (`part engine : Engine`), by the last segment of the qualified name.
(definition_specialization
  (qualified_name
    (name (identifier) @name) .)) @reference.class

(typing_part
  (qualified_name
    (name (identifier) @name) .)) @reference.type

; `import ScalarValues::*;` names the package it imports from. Subsetting, redefinition and
; reference parts (`subsets`, `redefines`, `::>`) name features, which no reference kind
; describes, and are not tagged; neither are member imports (`import ISQ::MassValue;`), which
; name an element rather than a package.
(import_statement
  (import_reference
    (wildcard_import
      (name (identifier) @name) .))) @reference.module
