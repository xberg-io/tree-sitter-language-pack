; Top-level `CREATE` statements are the declarations, with the kinds the `sql` overlay uses:
; tables and views are classes, since a query reads both the same way, and indexes are not tagged.
; The name is the unqualified object name (`users` of `app.users`): the last element of a
; qualified name, or the whole name when it has no schema.

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
        ]) @definition.class)))

; Columns of `CREATE TABLE` and `ALTER TABLE ... ADD COLUMN`
(columnDef . (ColId) @name) @definition.field

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
      ] @definition.class)))

; `CREATE TYPE name AS ENUM (...)`
(source_file
  (toplevel_stmt
    (stmt
      (DefineStmt
        (kw_type)
        [
          (any_name (attrs (attr_name) @name .))
          (any_name . (ColId) @name .)
        ]
        (kw_enum)) @definition.enum)))

; The other `CREATE TYPE` forms and `CREATE DOMAIN`
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

; `CREATE TRIGGER`
(source_file
  (toplevel_stmt
    (stmt
      (CreateTrigStmt
        (name (ColId) @name)) @definition.function)))

; `EXECUTE FUNCTION app.audit()`
(CreateTrigStmt
  [
    (func_name (indirection (indirection_el (attr_name) @name) .))
    (func_name . [(type_function_name) (ColId)] @name .)
  ]) @reference.call

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

; Tables and views read by a query, and the table a foreign key references.
(relation_expr
  [
    (qualified_name (indirection (indirection_el (attr_name) @name) .))
    (qualified_name . (ColId) @name .)
  ]) @reference.class

(ColConstraintElem
  (kw_references)
  .
  [
    (qualified_name (indirection (indirection_el (attr_name) @name) .))
    (qualified_name . (ColId) @name .)
  ]) @reference.class

; `app.add(1, 2)` and `count(*)`
(func_application
  [
    (func_name (indirection (indirection_el (attr_name) @name) .))
    (func_name . [(type_function_name) (ColId)] @name .)
  ]) @reference.call
