; C3. Module-level declarations: the module itself, functions, methods, macros, user-defined types
; (struct, union and bitstruct are classes) with their fields and enum constants, and globals. Locals declared inside function bodies are statements, not global declarations.

; `module shapes;`
(module_declaration
  path: (path_ident) @name) @definition.module

; `fn int Point.sum(Point* self)`: a method names the type it extends.
(func_definition
  (func_header
    method_type: (_)
    name: (ident) @name)) @definition.method

; `fn int area(int w, int h) { ... }` and extern `fn void puts(char*);`
(func_definition
  (func_header
    name: (ident) @name)) @definition.function

(global_declaration
  (func_declaration
    (func_header
      name: (ident) @name)) @definition.function)

; Interface methods: `fn void draw();` inside `interface Drawable`
(interface_func_declaration
  (func_declaration
    (func_header
      name: (ident) @name)) @definition.method)

; `macro square(x) { ... }`
(macro_declaration
  (macro_header
    name: (_) @name)) @definition.macro

; `struct Point { ... }` and `union Number { ... }`
(struct_declaration
  name: (type_ident) @name) @definition.class

(bitstruct_declaration
  name: (type_ident) @name) @definition.class

[(struct_member_declaration
   (identifier_list
     (ident) @name))
 (bitstruct_member_declaration
   (ident) @name)] @definition.field

; `RED,` in an enum and `FAST = 1,` in a constdef
(enum_constant
  name: (const_ident) @name) @definition.constant

[(enum_declaration name: (type_ident) @name)
 (constdef_declaration name: (type_ident) @name)] @definition.enum

(interface_declaration
  name: (type_ident) @name) @definition.interface

; `alias Score = int;` and `typedef Id = int;`
[(alias_declaration name: (_) @name)
 (typedef_declaration name: (type_ident) @name)] @definition.type

; `faultdef NOT_FOUND, IO_ERROR;`
(faultdef_declaration
  (const_ident) @name) @definition.constant

; Globals: `const int MAX = 8;` and `int counter = 0;`
(global_declaration
  (const_declaration
    name: (const_ident) @name) @definition.constant)

(global_declaration
  (declaration
    name: (_) @name) @definition.variable)

; Calls: `area(2, 3)`, `io::printn(x)` and `p.sum()`
(call_expr
  function: [(ident_expr (ident) @name)
             (field_expr field: (access_ident (ident) @name))]) @reference.call
