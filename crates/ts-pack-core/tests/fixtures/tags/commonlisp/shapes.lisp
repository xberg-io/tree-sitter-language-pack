(defun add (a b)
  (+ a b))

(defclass point ()
  ((x :initarg :x)))

(defvar *origin* (make-instance 'point :x 0))
