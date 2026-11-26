; Top-level `CREATE` statements are the declarations. The name is the unqualified object name
; (`users` of `public.users`).

; `CREATE TABLE`
(program
  (statement
    (create_table (object_reference name: (identifier) @name)) @definition.struct))

; `CREATE VIEW` and `CREATE MATERIALIZED VIEW`
(program
  (statement
    [(create_view (object_reference name: (identifier) @name))
     (create_materialized_view (object_reference name: (identifier) @name))] @definition.type))

; `CREATE TYPE`
(program
  (statement
    (create_type (object_reference name: (identifier) @name)) @definition.type))

; `CREATE FUNCTION` and `CREATE PROCEDURE`
(program
  (statement
    [(create_function (object_reference name: (identifier) @name))
     (create_procedure (object_reference name: (identifier) @name))] @definition.function))

; `CREATE INDEX <name> ON <table>`; an index without a name has nothing to find it by.
(program
  (statement
    (create_index column: (identifier) @name) @definition.variable))

; `CREATE TRIGGER`
(program
  (statement
    (create_trigger (keyword_trigger) . (object_reference name: (identifier) @name)) @definition.function))

; `CREATE SEQUENCE`
(program
  (statement
    (create_sequence (object_reference name: (identifier) @name)) @definition.variable))

; `CREATE SCHEMA`
(program
  (statement
    (create_schema (identifier) @name) @definition.module))
