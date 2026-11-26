; COBOL. The program named by `PROGRAM-ID` and the top-level data items (level 01 and 77) of the
; DATA DIVISION. Subordinate items (levels 02-49) are fields of their record and are not tagged.
;
; Paragraph and section definitions are not tagged: the grammar's `paragraph_header` and
; `section_header` hold the name as a hidden token, so a capture would include the trailing
; period. `PERFORM` targets are tagged as references.

; `PROGRAM-ID. PAYROLL.`
(program_name) @name @definition.module

; `01 WS-TOTAL PIC 9(5).` and `77 WS-COUNT PIC 9.`
(data_description
  (level_number) @_level
  (entry_name) @name
  (#match? @_level "^(0?1|77)$")) @definition.variable

; `PERFORM CALC-PARA.`
(perform_procedure
  (label
    (qualified_word) @name)) @reference.call
