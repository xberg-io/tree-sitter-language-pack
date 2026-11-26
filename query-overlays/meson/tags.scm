; Meson build files. A build target is reached through the variable it is assigned to
; (`link_with : greet_lib`), and the grammar keeps a target's name string together with its
; quotes, so targets are tagged by that variable. Only top-level `=` assignments declare;
; `+=` extends an existing variable.

; `greet_lib = static_library('greet', ...)`
(source_file
  (operatorunit . (identifier) @name "=") @definition.variable)

; `executable(...)` and the method in `meson.get_compiler('c')`
(normal_command command: (identifier) @name) @reference.call
