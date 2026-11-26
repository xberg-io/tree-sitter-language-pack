; Top-level `CREATE` statements are the declarations, with the kinds the `sql` overlay uses:
; tables and views are classes, since a query reads both the same way. The grammar parses a
; dotted path (`mydataset.events`) as one `identifier` whose parts are hidden tokens, so no node
; holds the unqualified object name: the name is the path as written. BigQuery names a table by
; its dataset path at every use (`FROM mydataset.events`), so definitions and references agree.
; A backquoted path keeps its backquotes in the token and is not tagged.

; `CREATE VIEW` and `CREATE MATERIALIZED VIEW`, which share the `CREATE TABLE` node, and
; `CREATE TABLE`, including its `LIKE`, `COPY`, `CLONE`, `SNAPSHOT` and `EXTERNAL` forms
(source_file
  [
    (create_table_statement table_name: (identifier) @name)
    (create_table_like_statement table_name: (identifier) @name)
    (create_table_copy_statement table_name: (identifier) @name)
    (create_table_clone_statement table_name: (identifier) @name)
    (create_snapshot_table_statement table_name: (identifier) @name)
    (create_external_table_statement table_name: (identifier) @name)
  ] @definition.class
  (#not-match? @name "`"))

(create_table_parameters
  (column_definition
    column_name: (identifier) @name) @definition.field
  (#not-match? @name "`"))

; `CREATE FUNCTION`, `CREATE TABLE FUNCTION`, remote functions and `CREATE PROCEDURE`
(source_file
  [
    (create_function_statement routine_name: (identifier) @name)
    (create_table_function_statement routine_name: (identifier) @name)
    (create_remote_function_statement routine_name: (identifier) @name)
    (create_procedure_statement routine_name: (identifier) @name)
  ] @definition.function
  (#not-match? @name "`"))

; `CREATE SCHEMA`
(source_file
  (create_schema_statement
    schema_name: (identifier) @name) @definition.module
  (#not-match? @name "`"))

; Tables read by a query or copied, and routines called.
((from_item table_name: (identifier) @name) @reference.class
  (#not-match? @name "`"))

((copy_clause source_table_name: (identifier) @name) @reference.class
  (#not-match? @name "`"))

((function_call function: (identifier) @name) @reference.call
  (#not-match? @name "`"))
