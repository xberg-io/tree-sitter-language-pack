; QML object members (properties, signals, functions), object ids and inline components, plus the
; object types a file instantiates and the JavaScript calls it makes.

; `component Badge: Rectangle { ... }`
(ui_inline_component name: (identifier) @name) @definition.class

; `property string title: "Card"`
(ui_property name: (identifier) @name) @definition.field

; `signal clicked(int index)`
(ui_signal name: (identifier) @name) @definition.method

; `function increment() { ... }`
(ui_object_initializer (function_declaration name: (identifier) @name) @definition.method)

; `id: root` names the object for references in the file.
((ui_binding name: (identifier) @_key value: (expression_statement (identifier) @name)) @definition.variable
  (#eq? @_key "id"))

; `Text { ... }`
(ui_object_definition type_name: (_) @name) @reference.class

(call_expression function: (identifier) @name) @reference.call

(call_expression function: (member_expression property: (property_identifier) @name)) @reference.call
