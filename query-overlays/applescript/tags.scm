; Handlers
(simple_handler
  name: (identifier) @name) @definition.function

(positional_handler
  name: (identifier) @name) @definition.function

(labeled_handler
  name: (identifier) @name) @definition.function

; Script objects
(script_statement
  name: (identifier) @name) @definition.class

; ~keep At the top level of a file this grammar pin parses `script Name` as a two-word
; ~keep application command and the block's `end script` as a separate command, so the
; ~keep script object is only recognizable from the command words.
(application_command
  command: (property_name
    (identifier) @_keyword)
  .
  command: (property_name
    (identifier) @name)
  (#eq? @_keyword "script")) @definition.class

; Properties of a script object, or of the script itself at the top level
(property_statement
  name: (identifier) @name) @definition.field

; Calls: application commands, handler calls and Objective-C bridge methods. A block's
; closing `end` misparsed as a command is not a call.
(application_command
  command: (identifier) @name
  (#not-eq? @name "end")) @reference.call

(handler_call
  name: (identifier) @name) @reference.call

(method_call
  method: (method_call_label) @name) @reference.call
