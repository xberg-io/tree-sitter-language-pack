; X resources files are run through the C preprocessor, whose `#define`s are the file's true
; definitions. Each resource line sets a database entry, named by its full component path
; (`XTerm*faceName`), so it is a property.

; `#define BASE03 #002b36`
(define_directive name: (identifier) @name) @definition.constant

; `#define FONT(size) xft:Iosevka:size=size`
(define_function_directive name: (identifier) @name) @definition.macro

; `URxvt.scrollBar: false`
(resource name: (components) @name) @definition.property

; `#include "fonts.xresources"`
(include_directive file: (string (string_content) @name)) @reference.module
