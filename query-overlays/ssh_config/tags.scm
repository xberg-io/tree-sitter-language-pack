; An ssh_config file's named blocks are its `Host` sections, one tag per host pattern they
; match. `Match` sections select hosts by runtime conditions and name nothing.

; `Host bastion` and `Host web-* db-*`
(host_declaration argument: (pattern) @name) @definition.module

; `ProxyJump bastion` refers to a host by its alias.
(parameter
  keyword: _ @_keyword
  host: (string) @name
  (#match? @_keyword "(?i)^proxyjump$")) @reference.module
