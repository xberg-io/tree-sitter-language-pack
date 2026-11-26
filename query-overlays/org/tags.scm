; Headlines are the sections of an Org document, named by their text. Named targets are the
; `CUSTOM_ID` of a headline's property drawer and the `#+NAME:` of a block.

(headline item: (item) @name) @definition.section

((property name: (expr) @_key value: (value) @name) @definition.label
  (#match? @_key "^[Cc][Uu][Ss][Tt][Oo][Mm]_[Ii][Dd]$"))

((block directive: (directive name: (expr) @_key value: (value) @name) @definition.label)
  (#match? @_key "^[Nn][Aa][Mm][Ee]$"))
