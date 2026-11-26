(defun my-greet (name)
  "Greet NAME."
  (message "Hello %s" name))

(defmacro my-when (cond &rest body)
  `(if ,cond (progn ,@body)))
