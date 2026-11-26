(module
  module_name: (identifier) @name) @definition.module

(submodule
  submodule_name: (_) @name) @definition.module

; Every other declaration is a generic statement named by its keyword.
(statement
  (statement_keyword) @_keyword
  (argument (node_identifier) @name)
  (#eq? @_keyword "typedef")) @definition.type

(statement
  (statement_keyword) @_keyword
  (argument (node_identifier) @name)
  (#any-of? @_keyword "identity" "feature")) @definition.constant

; Reusable node sets, data nodes that hold children, and event messages.
(statement
  (statement_keyword) @_keyword
  (argument (node_identifier) @name)
  (#any-of? @_keyword "grouping" "container" "list" "notification")) @definition.class

(statement
  (statement_keyword) @_keyword
  (argument (node_identifier) @name)
  (#any-of? @_keyword "leaf" "leaf-list" "anydata" "anyxml")) @definition.field

(statement
  (statement_keyword) @_keyword
  (argument (node_identifier) @name)
  (#any-of? @_keyword "rpc" "action")) @definition.method

(statement
  (statement_keyword) @_keyword
  (argument (node_identifier) @name)
  (#eq? @_keyword "uses")) @reference.class

(statement
  (statement_keyword) @_keyword
  (argument (node_identifier) @name)
  (#eq? @_keyword "type")) @reference.type

(statement
  (statement_keyword) @_keyword
  (argument (node_identifier) @name)
  (#any-of? @_keyword "import" "include")) @reference.module
