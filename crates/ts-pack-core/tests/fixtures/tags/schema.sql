CREATE SCHEMA billing;

CREATE TABLE IF NOT EXISTS public.users (id INT PRIMARY KEY, email TEXT);

CREATE VIEW active_users AS SELECT id FROM users;

CREATE MATERIALIZED VIEW user_counts AS SELECT count(*) FROM users;

CREATE TYPE mood AS ENUM ('sad', 'happy');

CREATE FUNCTION add_one(x INT) RETURNS INT AS $$ SELECT x + 1 $$ LANGUAGE sql;

CREATE INDEX users_email_idx ON users (email);

CREATE INDEX ON users (id);

CREATE SEQUENCE order_seq;

CREATE TRIGGER audit_users AFTER INSERT ON users FOR EACH ROW EXECUTE FUNCTION audit();
