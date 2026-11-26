; Devicetree source (`.dts`, `.dtsi`). Nodes are the named units of the tree; a label such as
; `status_led:` names a node so other nodes can refer to it as `&status_led`. Properties are
; key/value data of their node and are not tagged.

; `leds { ... }` and `memory@80000000 { ... }`; the root node `/` has no name to find it by.
(node
  name: (identifier) @name
  (#not-eq? @name "/")) @definition.module

; `status_led: led-0 { ... }`
(node
  label: (identifier) @name) @definition.label

; `&uart0 { ... }` and `gpios = <&gpio 13 0>;`
(reference
  label: (identifier) @name) @reference.label

; `#define LED_GPIO 13`
[(preproc_def name: (identifier) @name)
 (preproc_function_def name: (identifier) @name)] @definition.macro
