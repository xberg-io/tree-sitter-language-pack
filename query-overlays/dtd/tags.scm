(elementdecl
  . (Name) @name) @definition.type

(AttDef
  . (Name) @name) @definition.field

; General (`&name;`) and parameter (`%name;`) entities.
(GEDecl
  . (Name) @name) @definition.constant

(PEDecl
  . (Name) @name) @definition.macro

(NotationDecl
  . (Name) @name) @definition.type

; Elements named by a content model or an attribute list.
(children
  (Name) @name) @reference.type

(AttlistDecl
  . (Name) @name) @reference.type

(PEReference
  (Name) @name) @reference.macro
