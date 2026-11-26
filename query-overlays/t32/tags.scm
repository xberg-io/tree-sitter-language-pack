; SPDX-FileCopyrightText: 2022 Christoph Sax <c_sax@mailbox.org>
;
; SPDX-License-Identifier: MIT
;
; TRACE32 PRACTICE scripts. Based on the tags.scm of github.com/xasc/tree-sitter-t32, which
; tagged subroutines, labelled blocks and `GOSUB` calls. Jump labels, `GOTO`/`JUMPTO` targets
; and `GLOBAL` macros are added. A macro keeps its `&`, which it carries at every use. `LOCAL`
; and `PRIVATE` macros and `PARAMETERS` are local to their script or block and are not tagged.

; Subroutine definitions
(subroutine_block
  command: (identifier)
  subroutine: (identifier) @name) @definition.function

; `name:` followed by a block is a subroutine entered with `GOSUB`.
(labeled_expression
  label: (identifier) @name
  (block)) @definition.function

; A bare `name:` is a jump target.
(labeled_expression
  label: (identifier) @name) @definition.label

; Subroutine calls
(subroutine_call_expression
  command: (identifier)
  subroutine: (identifier) @name) @reference.call

; `GOTO name` and `JUMPTO name`
(command_expression
  command: (identifier) @_command
  arguments: (argument_list . (identifier) @name)
  (#match? @_command "(?i)^(goto|jumpto)$")) @reference.label

; `GLOBAL &name`
(macro_definition
  command: (identifier) @_command
  macro: (macro) @name
  (#match? @_command "(?i)^global$")) @definition.variable
