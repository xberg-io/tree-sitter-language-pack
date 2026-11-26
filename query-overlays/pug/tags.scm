; Mixins and blocks a Pug template defines, plus mixin calls.

; `mixin card(title)`
(mixin_definition (mixin_name) @name) @definition.macro

; `block content`
(block_definition (block_name) @name) @definition.section

; `+card("Welcome")`
(mixin_use (mixin_name) @name) @reference.call
