; Top-level `CREATE` statements are the declarations. The name is the unqualified object name
; (`users` of `public.users`). Tables and views are both classes: a query reads a view exactly as
; it reads a table, so `FROM name` references either the same way. Indexes are not tagged: no
; statement refers to an index except to alter or drop it.

; `CREATE TABLE` and its columns
(program
  (statement
    (create_table (object_reference name: (identifier) @name)) @definition.class))

(create_table
  (column_definitions
    (column_definition name: (identifier) @name) @definition.field))

; `CREATE VIEW` and `CREATE MATERIALIZED VIEW`
(program
  (statement
    [(create_view (object_reference name: (identifier) @name))
     (create_materialized_view (object_reference name: (identifier) @name))] @definition.class))

; `CREATE TYPE name AS ENUM (...)` and the other `CREATE TYPE` forms
(program
  (statement
    (create_type
      (object_reference name: (identifier) @name)
      (keyword_enum)) @definition.enum))

(program
  (statement
    (create_type (object_reference name: (identifier) @name)) @definition.type))

; `CREATE FUNCTION` and `CREATE PROCEDURE`
(program
  (statement
    [(create_function (object_reference name: (identifier) @name))
     (create_procedure (object_reference name: (identifier) @name))] @definition.function))

; `CREATE TRIGGER name ... EXECUTE FUNCTION fn()`
(program
  (statement
    (create_trigger (keyword_trigger) . (object_reference name: (identifier) @name)) @definition.function))

(create_trigger
  (keyword_function)
  .
  (object_reference name: (identifier) @name)) @reference.call

; `CREATE SEQUENCE`
(program
  (statement
    (create_sequence (object_reference name: (identifier) @name)) @definition.variable))

; `CREATE SCHEMA`
(program
  (statement
    (create_schema (identifier) @name) @definition.module))

; Tables and views read by a query, and the table a foreign key references.
(relation (object_reference name: (identifier) @name)) @reference.class

(column_definition
  (keyword_references)
  .
  (object_reference name: (identifier) @name)) @reference.class

; `add_one(x)`
(invocation (object_reference name: (identifier) @name)) @reference.call
