; Vim script. Functions, user commands and autocommand groups are declarations wherever they
; sit; variables only when set at the top level of the script (a `let` inside a function is
; implementation). A scoped name keeps its scope prefix: `s:Format`, `g:greeter_name`.

; `function! s:Format(name) abort` and `function! greeter#Greet(...)`
(function_declaration name: (_) @name) @definition.function

; `command! -nargs=? Greet call greeter#Greet(<f-args>)`
(command_statement name: (command_name) @name) @definition.function

; `augroup greeter`; `augroup END` closes the group.
((augroup_statement (augroup_name) @name) @definition.module
  (#not-match? @name "^[Ee][Nn][Dd]$"))

; `let g:greeter_name = 'world'`
(script_file
  (let_statement . [(identifier) (scoped_identifier)] @name) @definition.variable)

; `const s:prefix = 'Hello'`
(script_file
  (const_statement . [(identifier) (scoped_identifier)] @name) @definition.constant)

; `s:Format(l:name)` and `call greeter#Greet()`
(call_expression function: [(identifier) (scoped_identifier)] @name) @reference.call
