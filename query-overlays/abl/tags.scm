; OpenEdge ABL (`.p`, `.w`, `.cls`). Based on queries/tags.scm of
; https://github.com/usagi-coffee/tree-sitter-abl, with its roles mapped onto the common tag
; roles: properties are fields, events methods, temp-tables, work-tables, workfiles and datasets
; classes, and enum members constants. A package-qualified type name (`Geometry.Circle`) is
; tagged by its last identifier. A `FUNCTION ... FORWARD.` prototype is not tagged: the function
; it announces is tagged where it is defined.

(class_definition
  name: [(identifier) @name
         (qualified_name right: (identifier) @name .)]) @definition.class

(interface_definition
  name: [(identifier) @name
         (qualified_name right: (identifier) @name .)]) @definition.interface

(enum_statement
  name: [(identifier) @name
         (qualified_name right: (identifier) @name .)]) @definition.enum

(enum_statement
  (member
    name: (identifier) @name) @definition.constant)

(function_definition
  name: (_) @name) @definition.function

(procedure_definition
  name: (_) @name) @definition.function

(method_definition
  name: (identifier) @name) @definition.method

(constructor_definition
  name: (identifier) @name) @definition.constructor

(destructor_definition
  name: (identifier) @name) @definition.method

(property_definition
  name: (identifier) @name) @definition.field

(event_definition
  name: (identifier) @name) @definition.method

[(temp_table_definition
   name: (identifier) @name)
 (work_table_definition
   name: (identifier) @name)
 (workfile_definition
   name: (identifier) @name)
 (dataset_definition
   name: (identifier) @name)] @definition.class

; Static calls only: VALUE(...) and dynamic string targets are runtime values. `add(1, 2)`,
; `Geometry.Util:Twice(2)` and `hObj:GetValue()` name their last identifier.
(function_call
  function: [(identifier) @name
             (qualified_name right: (identifier) @name .)
             (scoped_name right: (identifier) @name .)
             (object_access right: (identifier) @name .)
             (object_access member: (identifier) @name .)]) @reference.call

; `RUN greet.`, `RUN utils/report.p.` and `RUN Geometry.Report.`
(run_statement
  procedure: [(identifier) @name
              (procedure_name) @name
              (qualified_name right: (identifier) @name .)]) @reference.call

(class_definition
  super: [(identifier) @name
          (qualified_name right: (identifier) @name .)]) @reference.class

(class_definition
  interface: [(identifier) @name
              (qualified_name right: (identifier) @name .)]) @reference.implementation

; `INTERFACE IShape INHERITS IDrawable`
(interface_definition
  super: [(identifier) @name
          (qualified_name right: (identifier) @name .)]) @reference.interface

(new_expression
  type: [(identifier) @name
         (qualified_name right: (identifier) @name .)]) @reference.class
