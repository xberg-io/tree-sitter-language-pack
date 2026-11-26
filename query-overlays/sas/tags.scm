; SAS. Based on the tags.scm of github.com/ix-infrastructure/tree-sitter-sas, which tagged every
; dataset name part and every PROC as a module and captured lineage under non-tag roles.
; Datasets are tables (class) tagged once by their member name, the last part of
; `library.member`; a PROC step runs a procedure, so it is a call, not a definition.

; `%macro name(params); ... %mend;`
(macro_definition
  name: (macro_name (identifier) @name)) @definition.macro

; `%let name = value;`
(macro_variable_assignment
  name: (identifier) @name) @definition.variable

; `%name(args);` and inline `%name(args)`; the `%` is outside the `macro_name` node.
(macro_call_statement
  name: (macro_name) @name) @reference.call

(macro_call
  name: (macro_name) @name) @reference.call

; `data work.scores;` creates the dataset `scores` in library `work`; `data _null_;` creates none.
(data_step_header
  (dataset_name (identifier) @name .)
  (#not-match? @name "(?i)^_null_$")) @definition.class

; `proc sql; create table work.top as ...`
(sql_create_statement
  output: (dataset_name (identifier) @name .)) @definition.class

; `proc sort ...;`
(proc_step_header
  name: (identifier) @name) @reference.call

; Datasets read by `set`, `merge`, `update` and SQL `from`/`join`.
(set_statement (dataset_name (identifier) @name .)) @reference.class

(merge_statement (dataset_name (identifier) @name .)) @reference.class

(update_statement (dataset_name (identifier) @name .)) @reference.class

(table_reference (dataset_name (identifier) @name .)) @reference.class

; `libname sales '/data/sales';` declares a library.
(libname_statement
  libref: (identifier) @name) @definition.module
