; Each assignment in a `.env` file defines an environment variable; `${NAME}` and `$NAME` inside
; a value refer to one.

; `DB_HOST=localhost` and `export LOG_LEVEL=debug`
(assignment key: (identifier) @name) @definition.variable

; `"postgres://${DB_HOST}"`
(variable (identifier) @name) @reference.variable
