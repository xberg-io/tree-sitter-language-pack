; Pascal and Delphi (`.pas`). The program, unit or library; its types, constants, variables and
; routines; and the methods a class, record or interface type declares. Declarations local to a
; routine (its `var`, `const` and nested routines) are not tagged.

; `program Shapes;`, `unit Geometry;` and `library Plugin;`
[(program (moduleName) @name)
 (unit (moduleName) @name)
 (library (moduleName) @name)] @definition.module

; Routines: `function Add(A, B: Integer): Integer; begin ... end;`, and the routines a unit's
; interface section declares.
[(program (defProc header: (declProc name: (identifier) @name)) @definition.function)
 (library (defProc header: (declProc name: (identifier) @name)) @definition.function)
 (implementation (defProc header: (declProc name: (identifier) @name)) @definition.function)
 (interface (declProc name: (identifier) @name) @definition.function)]

; Method bodies: `function TShape.Area: Double; begin ... end;`
[(program (defProc header: (declProc name: (genericDot rhs: (identifier) @name))) @definition.method)
 (library (defProc header: (declProc name: (genericDot rhs: (identifier) @name))) @definition.method)
 (implementation (defProc header: (declProc name: (genericDot rhs: (identifier) @name))) @definition.method)]

; Methods declared in a class, record, interface or helper type
[(declClass (declProc name: (identifier) @name) @definition.method)
 (declClass (declSection (declProc name: (identifier) @name) @definition.method))
 (declIntf (declProc name: (identifier) @name) @definition.method)
 (declIntf (declSection (declProc name: (identifier) @name) @definition.method))
 (declHelper (declProc name: (identifier) @name) @definition.method)
 (declHelper (declSection (declProc name: (identifier) @name) @definition.method))]

; Types declared in a routine's local section are local; this claims them before the patterns
; below can tag them.
(defProc
  local: (declTypes
           (declType
             name: (identifier) @ignore)))

; Types, by what they define
(declTypes
  (declType
    name: (identifier) @name
    type: (declClass (kClass))) @definition.class)

(declTypes
  (declType
    name: (identifier) @name
    type: (declClass (kRecord))) @definition.struct)

(declTypes
  (declType
    name: (identifier) @name
    type: (declIntf)) @definition.interface)

(declTypes
  (declType
    name: (identifier) @name
    type: (type (declEnum))) @definition.enum)

[(program (declTypes (declType name: (identifier) @name) @definition.type))
 (library (declTypes (declType name: (identifier) @name) @definition.type))
 (interface (declTypes (declType name: (identifier) @name) @definition.type))
 (implementation (declTypes (declType name: (identifier) @name) @definition.type))]

; `const MaxSides = 8;`
[(program (declConsts (declConst name: (identifier) @name) @definition.constant))
 (library (declConsts (declConst name: (identifier) @name) @definition.constant))
 (interface (declConsts (declConst name: (identifier) @name) @definition.constant))
 (implementation (declConsts (declConst name: (identifier) @name) @definition.constant))]

; `var Counter: Integer;`
[(program (declVars (declVar name: (identifier) @name) @definition.variable))
 (library (declVars (declVar name: (identifier) @name) @definition.variable))
 (interface (declVars (declVar name: (identifier) @name) @definition.variable))
 (implementation (declVars (declVar name: (identifier) @name) @definition.variable))]

; Calls with arguments: `Add(1, 2)` and `Shape.Resize(2)`. A call without parentheses reads the
; same as a variable and is not tagged.
(exprCall
  entity: [(identifier) @name
           (exprDot rhs: (identifier) @name)]) @reference.call
