(struct
  . (identifier) @name) @definition.class

; Tasks and workflows are both invoked by `call`.
(task
  . (identifier) @name) @definition.function

(workflow
  . (identifier) @name) @definition.function

; `call align` and `call qc.check as check_sam`: the last segment names the task or workflow.
(call
  (qualified_identifier (identifier) @name .)) @reference.call

(user_type
  (identifier) @name) @reference.type
