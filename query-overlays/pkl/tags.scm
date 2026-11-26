(moduleClause
  (qualifiedIdentifier) @name) @definition.module

(typeAlias
  . (identifier) @name) @definition.type

(clazz
  (identifier) @name) @definition.class

; Members of a class body are fields and methods; module-level members are the module's
; properties and functions.
(classBody
  (classProperty
    (identifier) @name) @definition.field)

(classBody
  (classMethod
    (methodHeader . (identifier) @name)) @definition.method)

(module
  (classProperty
    (identifier) @name) @definition.variable)

(classMethod
  (methodHeader . (identifier) @name)) @definition.function

; The last segment of a qualified type name is the referenced type.
(declaredType
  (qualifiedIdentifier (identifier) @name .)) @reference.type

(unqualifiedAccessExpr
  (identifier) @name
  (argumentList)) @reference.call

(qualifiedAccessExpr
  (identifier) @name
  (argumentList)) @reference.call
