(defpackage :shapes
  (:use :cl)
  (:export #:area #:make-circle))

(in-package :shapes)

(defparameter *default-radius* 1.0)

(defvar *registry* (make-hash-table))

(defconstant +tau+ (* 2 pi))

(defstruct point
  (x 0)
  (y 0))

(defstruct (circle (:include point))
  (radius *default-radius*))

(defclass shape ()
  ((name :initarg :name :accessor shape-name)))

(defclass square (shape)
  ((side :initarg :side :reader square-side)))

(defgeneric area (shape)
  (:documentation "Area of SHAPE."))

(defmethod area ((s square))
  (let ((side (square-side s)))
    (* side side)))

(defmacro with-shape ((var shape) &body body)
  `(let ((,var ,shape)) ,@body))

(defun register (shape)
  (setf (gethash (shape-name shape) *registry*) shape))

(defun unit-square ()
  (register (make-instance 'square :name "unit" :side 1)))
