; gitattributes. `[attr]name ...` defines an attribute macro that later lines set like any
; attribute. Uses are not tagged as references: a set attribute name may equally be a builtin
; or a plain user attribute, and only the macro definitions are declarations.

(macro_def
  macro_name: (attr_name) @name) @definition.macro
