; YAML. An anchor (`&name`) names a node so later aliases (`*name`) can repeat it; mapping keys
; are data and are not tagged. The anchor_name and alias_name nodes exclude the `&` and `*`.

(anchor
  (anchor_name) @name) @definition.label

(alias
  (alias_name) @name) @reference.label
