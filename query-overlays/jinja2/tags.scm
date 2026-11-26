; The grammar parses each `{% ... %}` statement as one opaque token, so macro, block and `set`
; names are not nodes and cannot be tagged. Only the calls in `{{ ... }}` expressions are.

(fn_call fn_name: (identifier) @name) @reference.call
