; Modules, functions and top-level variables an OpenSCAD file declares, plus module and function
; calls.

(module_declaration name: (identifier) @name) @definition.module

(function_declaration name: (identifier) @name) @definition.function

(source_file (assignment left: (identifier) @name) @definition.variable)

(module_call name: (identifier) @name) @reference.call

(function_call function: (identifier) @name) @reference.call
