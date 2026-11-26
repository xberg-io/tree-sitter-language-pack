; Follows query-overlays/bash/tags.scm where the two overlap: commands, modules, aliases and
; externs are declarations wherever they sit; variables are only those bound at the top level of
; the script (a `let` inside a command body is implementation).

; `def deploy [] { ... }` and `export def greet [...] { ... }`
(decl_def unquoted_name: (cmd_identifier) @name) @definition.function

; `def "build target" [...] { ... }`, a subcommand named by a string
(decl_def quoted_name: (val_string (string_content) @name)) @definition.function

; `extern "git push" [...]` declares the signature of an external command.
(decl_extern unquoted_name: (cmd_identifier) @name) @definition.function

(decl_extern quoted_name: (val_string (string_content) @name)) @definition.function

; `module tools { ... }`
(decl_module unquoted_name: (cmd_identifier) @name) @definition.module

(decl_module quoted_name: (val_string (string_content) @name)) @definition.module

; `alias ll = ls -l`
(decl_alias unquoted_name: (cmd_identifier) @name) @definition.alias

(decl_alias quoted_name: (val_string (string_content) @name)) @definition.alias

; `const PREFIX = "/usr/local"`
(nu_script (stmt_const (identifier) @name) @definition.constant)

; `let targets = [...]` and `mut attempts = 0`
(nu_script (stmt_let (identifier) @name) @definition.variable)

(nu_script (stmt_mut (identifier) @name) @definition.variable)

; `use std/log` and `use tools.nu *`
(decl_use module: (unquoted) @name) @reference.module

(decl_use module: (val_string (string_content) @name)) @reference.module
