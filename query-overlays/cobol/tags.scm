; COBOL. The program named by `PROGRAM-ID`, the files named by `SELECT`, the top-level data items
; (level 01 and 77) of the DATA DIVISION and the condition names (level 88). Subordinate items
; (levels 02-49) are fields of their record and are not tagged.
;
; Paragraph and section definitions are not tagged: the grammar's `paragraph_header` and
; `section_header` hold the name in the hidden `_WORD` token, which a query cannot capture, so
; the only capturable node is the whole header and its text keeps the trailing period
; (`MAIN-PARA.`) or the `SECTION` keyword. `CALL "PROG"` is not tagged either: the program name
; is a `string` node with no child, so its text keeps the quotes. `PERFORM` targets are tagged
; as references.

; `PROGRAM-ID. PAYROLL.`
(program_name) @name @definition.module

; `SELECT EMP-FILE ASSIGN TO "EMP.DAT".`
(select_statement
  file_name: (WORD) @name) @definition.variable

; `01 WS-TOTAL PIC 9(5).` and `77 WS-COUNT PIC 9.`
(data_description
  (level_number) @_level
  (entry_name) @name
  (#match? @_level "^(0?1|77)$")) @definition.variable

; `88 WS-DONE VALUE "Y".` names a condition on its parent item.
(data_description
  (level_number) @_level
  (entry_name) @name
  (#eq? @_level "88")) @definition.constant

; `PERFORM CALC-PARA.`
(perform_procedure
  (label
    (qualified_word) @name)) @reference.call
