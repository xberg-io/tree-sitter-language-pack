; Follows query-overlays/bash/tags.scm where the two overlap: functions are declarations
; wherever they sit; variables are only those assigned at the top level of the script (an
; assignment inside a function or script block is implementation). Classes and enums add their
; members.

; `function Invoke-Build { ... }`, also `filter` and `workflow`
(function_statement (function_name) @name) @definition.function

; `class Service { ... }`
(class_statement . (simple_name) @name) @definition.class

; A method named after its class is a constructor: `Service([string] $name) { ... }`
(class_statement
  . (simple_name) @_class
  (class_method_definition (simple_name) @name) @definition.constructor
  (#eq? @name @_class))

; `[string] Url() { ... }`
(class_method_definition (simple_name) @name) @definition.method

; Class properties (`[int] $Port = 8080`) are not tagged: the grammar's `variable` is a single
; token that keeps the `$`, while members are used as `$this.Port`, without it.

; `enum Stage { Build; Deploy = 2 }`
(enum_statement . (simple_name) @name) @definition.enum

(enum_member (simple_name) @name) @definition.constant

; `$Targets = @('api', 'worker')` at the top level; the grammar nests the target variable in the
; full expression-precedence chain.
(program
  (statement_list
    (pipeline
      (assignment_expression
        (left_assignment_expression
          (logical_expression
            (bitwise_expression
              (comparison_expression
                (additive_expression
                  (multiplicative_expression
                    (format_expression
                      (range_expression
                        (array_literal_expression
                          (unary_expression (variable) @name))))))))))) @definition.variable)))
