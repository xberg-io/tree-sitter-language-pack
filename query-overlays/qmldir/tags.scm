(command
  (keyword) @_keyword
  . (unit) @name
  (#eq? @_keyword "module")) @definition.module

; Type entries: `Button 1.0 Button.qml`, `singleton Theme 1.0 Theme.qml` and
; `internal Helper Helper.qml`.
(command
  . (identifier) @name
  . (float)) @definition.class

(command
  (keyword) @_keyword
  . (unit) @name
  (#any-of? @_keyword "singleton" "internal")) @definition.class

(command
  (keyword) @_keyword
  . (unit) @name
  (#eq? @_keyword "depends")) @reference.module

(command
  . (identifier) @_keyword
  . (unit) @name
  (#eq? @_keyword "import")) @reference.module
