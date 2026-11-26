; Properties of the global `*` section are the variables of a rofi theme; `@name` and `var(name)`
; reference them.

(rule_set
  selectors: (selectors (global_selector))
  body: (block (declaration (property_name) @name) @definition.variable))

(reference_value name: (identifier) @name) @reference.variable
