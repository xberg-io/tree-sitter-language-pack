; The grammar parses only `SELECT` and `EXECUTE` batches: it has no nodes for `CREATE TABLE`,
; `CREATE PROCEDURE`, `CREATE FUNCTION` or any other declaration, so there are no definitions to
; tag. Calls are references: the procedure an `EXEC`/`EXECUTE` statement (or a batch's leading
; bare procedure call) runs, and functions called in expressions. The name is the unqualified
; procedure or function name (`GetUser` of `dbo.GetUser`).

[
  (execute_body
    (func_proc_name_server_database_schema
      (func_proc_name_database_schema
        (func_proc_name_schema
          procedure: (id_) @name))))
  (execute_body_batch
    (func_proc_name_server_database_schema
      (func_proc_name_database_schema
        (func_proc_name_schema
          procedure: (id_) @name))))
  (function_call
    (scalar_function_name
      (func_proc_name_database_schema
        (func_proc_name_schema
          procedure: (id_) @name))))
] @reference.call
