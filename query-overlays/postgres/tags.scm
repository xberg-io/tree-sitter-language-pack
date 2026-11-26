; Top-level `CREATE` statements are the declarations, with the kinds the `sql` overlay uses. The
; name is the unqualified object name (`users` of `app.users`): the last element of a qualified
; name, or the whole name when it has no schema.

; `CREATE TABLE`; the name follows `TABLE` or `IF NOT EXISTS`, before any `PARTITION OF` parent.
(source_file
  (toplevel_stmt
    (stmt
      (CreateStmt
        [(kw_table) (kw_exists)]
        .
        [
          (qualified_name (indirection (indirection_el (attr_name) @name) .))
          (qualified_name . (ColId) @name .)
        ]) @definition.struct)))

; `CREATE VIEW` and `CREATE MATERIALIZED VIEW`
(source_file
  (toplevel_stmt
    (stmt
      [
        (ViewStmt
          [
            (qualified_name (indirection (indirection_el (attr_name) @name) .))
            (qualified_name . (ColId) @name .)
          ])
        (CreateMatViewStmt
          (create_mv_target
            [
              (qualified_name (indirection (indirection_el (attr_name) @name) .))
              (qualified_name . (ColId) @name .)
            ]))
      ] @definition.type)))

; `CREATE TYPE` and `CREATE DOMAIN`
(source_file
  (toplevel_stmt
    (stmt
      [
        (DefineStmt
          (kw_type)
          [
            (any_name (attrs (attr_name) @name .))
            (any_name . (ColId) @name .)
          ])
        (CreateDomainStmt
          [
            (any_name (attrs (attr_name) @name .))
            (any_name . (ColId) @name .)
          ])
      ] @definition.type)))

; `CREATE FUNCTION` and `CREATE PROCEDURE`
(source_file
  (toplevel_stmt
    (stmt
      (CreateFunctionStmt
        [
          (func_name (indirection (indirection_el (attr_name) @name) .))
          (func_name . [(type_function_name) (ColId)] @name .)
        ]) @definition.function)))

; `CREATE INDEX <name> ON <table>`; an index without a name has nothing to find it by.
(source_file
  (toplevel_stmt
    (stmt
      (IndexStmt
        [
          (opt_single_name (ColId) @name)
          (name (ColId) @name)
        ]) @definition.variable)))

; `CREATE TRIGGER`
(source_file
  (toplevel_stmt
    (stmt
      (CreateTrigStmt
        (name (ColId) @name)) @definition.function)))

; `CREATE SEQUENCE`
(source_file
  (toplevel_stmt
    (stmt
      (CreateSeqStmt
        [
          (qualified_name (indirection (indirection_el (attr_name) @name) .))
          (qualified_name . (ColId) @name .)
        ]) @definition.variable)))

; `CREATE SCHEMA`
(source_file
  (toplevel_stmt
    (stmt
      (CreateSchemaStmt
        [
          (ColId) @name
          (opt_single_name (ColId) @name)
        ]) @definition.module)))
