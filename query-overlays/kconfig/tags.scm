; Kconfig declares configuration symbols (`config`, `menuconfig`) and groups them in menus and
; choices; every other symbol is a reference. `y`, `n` and `m` are tristate values, not symbols.
; A `menu "Networking"` title is a prompt shown to the user, not a name anything refers to, so
; menus are not tagged.

; `config NET` and `menuconfig NET_TLS`
(config name: (name (symbol) @name)) @definition.variable

(menuconfig name: (name (symbol) @name)) @definition.variable

; `choice LOG_BACKEND`; an anonymous `choice` has no name.
(choice name: (name (symbol) @name)) @definition.module

; `depends on NET`, `select CRYPTO`, `default LOG_UART`, `if NET`
((symbol) @name @reference.variable
  (#not-match? @name "^[ynm]$"))

; `source "drivers/Kconfig"`, also its relative and optional variants
(source (string (string_content) @name)) @reference.module
