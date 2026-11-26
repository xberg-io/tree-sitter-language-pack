; Solidity (`.sol`). Based on the tags.scm of JoranHonig/tree-sitter-solidity, with kinds
; normalized and the forms it left out added. Contracts, structs and custom errors are classes,
; interfaces are interfaces and libraries are modules; functions, modifiers and the constructor
; inside a contract, interface or library body are methods; events are methods. Constants and
; `immutable` state variables are constants, other state variables are fields. Function
; parameters and local variables are not tagged.

(contract_declaration
  name: (identifier) @name) @definition.class

(interface_declaration
  name: (identifier) @name) @definition.interface

(library_declaration
  name: (identifier) @name) @definition.module

(struct_declaration
  name: (identifier) @name) @definition.class

(struct_member
  name: (identifier) @name) @definition.field

(enum_declaration
  name: (identifier) @name) @definition.enum

(enum_value) @name @definition.constant

; `event Transfer(address indexed from, ...)`
(event_definition
  name: (identifier) @name) @definition.method

; `error Unauthorized(address caller);`
(error_declaration
  name: (identifier) @name) @definition.class

; `type Price is uint256;`
(user_defined_type_definition
  name: (identifier) @name) @definition.type

; functions and modifiers in a contract, interface or library body
(contract_body
  [(function_definition
     name: (identifier) @name)
   (modifier_definition
     name: (identifier) @name)] @definition.method)

; `constructor() { ... }` has no name node; the keyword is the name it is known by.
(constructor_definition
  "constructor" @name) @definition.constructor

; `function clamp(...)` at file level
(source_file
  (function_definition
    name: (identifier) @name) @definition.function)

; `uint256 constant MAX_SUPPLY = 1;` at file level
(constant_variable_declaration
  name: (identifier) @name) @definition.constant

; `uint256 public constant DECIMALS = 18;` and `address public immutable owner;`
[(state_variable_declaration
   "constant"
   name: (identifier) @name)
 (state_variable_declaration
   (immutable)
   name: (identifier) @name)] @definition.constant

(state_variable_declaration
  name: (identifier) @name) @definition.field

;; References

; `clamp(amount)` and `balance.add(capped)`
(call_expression
  function: (expression
    [(identifier) @name
     (member_expression
       property: (identifier) @name)])) @reference.call

; `emit Transfer(...)`, `revert Unauthorized(...)` and the `onlyOwner` modifier on a function
[(emit_statement
   name: (expression
     (identifier) @name))
 (revert_statement
   error: (expression
     (identifier) @name))
 (modifier_invocation
   (identifier) @name)] @reference.call

; `contract Token is IToken`
(inheritance_specifier
  ancestor: (user_defined_type
    (identifier) @name .)) @reference.class

; `using SafeMath for uint256;`
(using_directive
  (type_alias
    (identifier) @name .)) @reference.module
