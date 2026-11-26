; AL (Business Central, `.al`). Based on queries/tags.scm of
; https://github.com/SShadowS/tree-sitter-al, with these changes:
; - A quoted name (`"Customer Card"`) is one `quoted_identifier` token that includes its quotes,
;   with no node for the text inside, so only unquoted names are tagged.
; - Parameters, return values and the `var` section of a procedure or trigger are locals and are
;   not tagged; variables of an object's own `var` section are.
; - Enum objects are enums; the object kinds of the same shape share one pattern per role.
; - An object id (`Codeunit 50100`) used as a type reference is a number, not a name.
; - The `[Test]` patterns, which carry no tag role, are left out.

(namespace_declaration
  name: (namespace_name) @name) @definition.module

[(table_declaration object_name: (identifier) @name)
 (page_declaration object_name: (identifier) @name)
 (codeunit_declaration object_name: (identifier) @name)
 (report_declaration object_name: (identifier) @name)
 (query_declaration object_name: (identifier) @name)
 (xmlport_declaration object_name: (identifier) @name)
 (controladdin_declaration object_name: (identifier) @name)
 (tableextension_declaration object_name: (identifier) @name)
 (pageextension_declaration object_name: (identifier) @name)
 (enumextension_declaration object_name: (identifier) @name)
 (reportextension_declaration object_name: (identifier) @name)
 (profileextension_declaration object_name: (identifier) @name)
 (permissionsetextension_declaration object_name: (identifier) @name)
 (profile_declaration object_name: (identifier) @name)
 (permissionset_declaration object_name: (identifier) @name)
 (entitlement_declaration object_name: (identifier) @name)
 (pagecustomization_declaration object_name: (identifier) @name)] @definition.class

(enum_declaration
  object_name: (identifier) @name) @definition.enum

(interface_declaration
  object_name: (identifier) @name) @definition.interface

; Procedures, triggers (`trigger OnRun()`, and `trigger Customer::OnAfterInsert()` named by its
; member), control add-in events and interface procedures
[(procedure name: (identifier) @name)
 (event_declaration name: (identifier) @name)
 (trigger_declaration name: (identifier) @name)
 (trigger_declaration name: (member_trigger_name member: (identifier) @name))
 (interface_procedure name: (identifier) @name)
 (preproc_split_procedure name: (identifier) @name)] @definition.method

; Table fields, keys and field groups
[(field_declaration name: (identifier) @name)
 (key_declaration name: (identifier) @name)
 (fieldgroup_declaration name: (identifier) @name)] @definition.field

(enum_value_declaration
  value_name: (identifier) @name) @definition.constant

; Object-level `var` section, not a procedure's
(declaration_body
  (var_section
    body: (var_body
      (variable_declaration
        name: (identifier) @name) @definition.variable)))

; Report `labels { TitleLbl = 'Greetings'; }`
(label_declaration
  name: (identifier) @name) @definition.constant

; Page actions
[(action_declaration name: (identifier) @name)
 (customaction_declaration name: (identifier) @name)
 (systemaction_declaration name: (identifier) @name)
 (fileuploadaction_declaration name: (identifier) @name)
 (actionref_declaration action_name: (identifier) @name)] @definition.function

(view_definition
  name: (identifier) @name) @definition.field

; Calls: `Greet('World')`, the parenless `LogGreeting;` and `Entry.Insert()`
[(call_expression
   function: [(identifier) @name
              (member_expression member: (identifier) @name)])
 (call_statement
   function: (identifier) @name)] @reference.call

; Type uses: `Record GreetingEntry` and `Codeunit Greeter`
[(record_type
   reference: (identifier) @name)
 (object_reference_type
   reference: (identifier) @name)] @reference.type

(database_reference
  table_name: (identifier) @name) @reference.class

; `implements IGreeter`, interface parents (also inside an `#if` group) and the interface of an
; `Implementation = IGreeter = Greeter` mapping
[(implements_clause
   interface: [(identifier) (qualified_name)] @name)
 (interface_declaration
   extends_interface: [(identifier) (qualified_name)] @name)
 (preproc_conditional_extends
   extends_interface: [(identifier) (qualified_name)] @name)
 (implementation_value
   interface: (identifier) @name)] @reference.implementation

(implementation_value
  implementation: (identifier) @name) @reference.class

; `GreetingStyle::Formal`
(qualified_enum_value
  enum_type: (identifier) @name) @reference.class

(qualified_enum_value
  value: (identifier) @name) @reference.constant
