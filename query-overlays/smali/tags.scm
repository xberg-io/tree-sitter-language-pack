; Smali (`.smali`), Dalvik bytecode assembly. One class per file, with its fields and methods.
; The class is tagged by its simple name (`Greeter` of `Lcom/example/Greeter;`). Registers and
; jump labels inside a method body are local and are not tagged.

; `.class public Lcom/example/Greeter;`
(class_directive
  (class_identifier
    (identifier) @name .)) @definition.class

; `.method public constructor <init>(Ljava/lang/String;)V`
(method_definition
  (method_signature
    (method_identifier) @name
    (#eq? @name "<init>"))) @definition.constructor

; `.method public greet()Ljava/lang/String;`
(method_definition
  (method_signature
    (method_identifier) @name)) @definition.method

; `.field private name:Ljava/lang/String;`
(field_definition
  (field_identifier) @name) @definition.field

; `invoke-static {v0}, Lcom/example/Greeter;->shout(Ljava/lang/String;)Ljava/lang/String;`
(expression
  (opcode) @_opcode
  (body
    (full_method_signature
      (method_signature
        (method_identifier) @name)))
  (#match? @_opcode "^invoke-")) @reference.call
