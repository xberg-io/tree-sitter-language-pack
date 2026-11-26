; LuaCATS/EmmyLua annotations (luadoc). The grammar parses one annotation per injected `---@`
; comment line, so a document holds a single class, alias or enum declaration. Parent classes
; after `:` are references; field and parameter types are not tagged, since builtin types and
; structured types (`fun(...)`, `table<K, V>`) have no single name.

(class_annotation
  .
  (identifier) @name) @definition.class

(class_annotation
  ":"
  (identifier) @name) @reference.class

(alias_annotation
  (identifier) @name) @definition.type

(enum_annotation
  (identifier) @name) @definition.enum
