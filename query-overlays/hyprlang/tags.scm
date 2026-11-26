; Hyprland configuration. Its named blocks are the category sections, nested ones included;
; `$name = value` declares a variable. References are not tagged: the grammar lexes a `$name`
; inside a value together with the whitespace before it.

; `device:epic-mouse-v1 { ... }` is named by its device.
(section device: (device_name) @name) @definition.module

; Its `device` keyword names no block.
(section name: (name) @ignore device: (device_name))

; `general { ... }` and nested `blur { ... }`
(section name: (name) @name) @definition.module

; `$mainMod = SUPER`
(declaration name: (variable) @name) @definition.variable
