; Sections of a Razor view and the members its `@functions`/`@code` blocks declare.

; `@section Scripts { ... }`
(razor_section . (identifier) @name) @definition.section

(razor_block (method_declaration name: (identifier) @name) @definition.method)

(razor_block (property_declaration name: (identifier) @name) @definition.field)

(razor_block (field_declaration (variable_declaration (variable_declarator name: (identifier) @name))) @definition.field)
