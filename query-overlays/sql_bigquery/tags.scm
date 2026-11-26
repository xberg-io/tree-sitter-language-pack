; Top-level `CREATE` statements are the declarations, with the kinds the `sql` overlay uses. The
; grammar parses a dotted path (`mydataset.events`) as one `identifier` token with no parts, so
; the name is the path as written rather than the unqualified object name.

; `CREATE VIEW` and `CREATE MATERIALIZED VIEW`, which share the `CREATE TABLE` node
(source_file
  (create_table_statement
    ["VIEW" "MATERIALIZED_VIEW"]
    table_name: (identifier) @name) @definition.type)

; `CREATE TABLE`, including its `LIKE`, `COPY`, `CLONE`, `SNAPSHOT` and `EXTERNAL` forms
(source_file
  [
    (create_table_statement table_name: (identifier) @name)
    (create_table_like_statement table_name: (identifier) @name)
    (create_table_copy_statement table_name: (identifier) @name)
    (create_table_clone_statement table_name: (identifier) @name)
    (create_snapshot_table_statement table_name: (identifier) @name)
    (create_external_table_statement table_name: (identifier) @name)
  ] @definition.struct)

; `CREATE FUNCTION`, `CREATE TABLE FUNCTION`, remote functions and `CREATE PROCEDURE`
(source_file
  [
    (create_function_statement routine_name: (identifier) @name)
    (create_table_function_statement routine_name: (identifier) @name)
    (create_remote_function_statement routine_name: (identifier) @name)
    (create_procedure_statement routine_name: (identifier) @name)
  ] @definition.function)

; `CREATE SCHEMA`
(source_file
  (create_schema_statement
    schema_name: (identifier) @name) @definition.module)
