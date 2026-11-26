; Yuck (eww) declarations are top-level forms whose head is a `def*` keyword and whose second
; symbol is the name. Every other form's head symbol invokes a widget, built-in or defined.

; `(defwidget bar [] ...)`
(source_file
  (list . (symbol) @_def . (symbol) @name
    (#eq? @_def "defwidget")) @definition.function)

; `(defwindow main :monitor 0 ...)`
(source_file
  (list . (symbol) @_def . (symbol) @name
    (#eq? @_def "defwindow")) @definition.module)

; `(defvar volume 50)`, `(defpoll time ...)` and `(deflisten workspaces ...)`
(source_file
  (list . (symbol) @_def . (symbol) @name
    (#any-of? @_def "defvar" "defpoll" "deflisten")) @definition.variable)

; `def*` heads name the declaration forms themselves.
(source_file
  (list . (symbol) @ignore
    (#any-of? @ignore "defwidget" "defwindow" "defvar" "defpoll" "deflisten")))

; `(clock)` and `(box :orientation "h" ...)`
(list . (symbol) @name) @reference.call
