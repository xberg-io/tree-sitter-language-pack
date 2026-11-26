; Puppet manifests declare classes, defined types, functions and node definitions; resource
; declarations, `include` and `inherits` refer to them.

; `class greeter (...) { ... }` and `class greeter::install { ... }`
(class_definition . [(identifier) (class_identifier)] @name) @definition.class

; `define greeter::site (...) { ... }`
(defined_resource_type . [(identifier) (class_identifier)] @name) @definition.type

; `function greeter::url(String $host) >> String { ... }`
(function_declaration . [(identifier) (class_identifier)] @name) @definition.function

; `node 'web01.example.com' { ... }` and `node default { ... }`
(node_definition
  (node_name [(string (string_content) @name) (identifier) @name])) @definition.module

; `inherits greeter::params`
(class_inherits [(identifier) (class_identifier)] @name) @reference.class

; `include greeter::install`
(include_statement [(identifier) (class_identifier)] @name) @reference.class

; `service { 'greeter': ... }` and `greeter::site { 'main': ... }`
(resource_declaration type: [(identifier) (class_identifier)] @name) @reference.type

; `template('greeter/greeter.conf.erb')`
(function_call . (identifier) @name) @reference.call
