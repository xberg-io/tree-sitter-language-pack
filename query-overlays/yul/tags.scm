; `object "Token" {}` is not tagged: its name is a string literal whose node text keeps the
; quotes, and the grammar exposes no node for the bare name.
(functionDefinition
  name: (identifier) @name) @definition.function

; Calls of user-defined functions. The EVM dialect's builtins (`sstore`, `add`, `return`, ...)
; are opcodes the grammar parses as ordinary calls, so they are excluded by name.
((functionCall
  name: (identifier) @name) @reference.call
  (#not-any-of? @name
    "stop" "add" "sub" "mul" "div" "sdiv" "mod" "smod" "exp" "not" "lt" "gt" "slt" "sgt" "eq"
    "iszero" "and" "or" "xor" "byte" "shl" "shr" "sar" "addmod" "mulmod" "signextend" "keccak256"
    "pc" "pop" "mload" "mstore" "mstore8" "sload" "sstore" "tload" "tstore" "msize" "gas"
    "address" "balance" "selfbalance" "caller" "callvalue" "calldataload" "calldatasize"
    "calldatacopy" "codesize" "codecopy" "extcodesize" "extcodecopy" "returndatasize"
    "returndatacopy" "mcopy" "extcodehash" "create" "create2" "call" "callcode" "delegatecall"
    "staticcall" "return" "revert" "selfdestruct" "invalid" "log0" "log1" "log2" "log3" "log4"
    "chainid" "basefee" "blobbasefee" "origin" "gasprice" "blockhash" "blobhash" "coinbase"
    "timestamp" "number" "difficulty" "prevrandao" "gaslimit" "datasize" "dataoffset" "datacopy"
    "setimmutable" "loadimmutable" "linkersymbol" "memoryguard"))
