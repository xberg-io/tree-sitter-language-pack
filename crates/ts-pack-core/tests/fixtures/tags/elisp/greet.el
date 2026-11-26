;;; greet.el --- Greetings -*- lexical-binding: t -*-

(require 'cl-lib)
(require 'subr-x)

(defgroup greet nil
  "Greetings."
  :group 'tools)

(defcustom greet-name "world"
  "Who to greet."
  :type 'string)

(defvar greet-count 0
  "How many greetings were sent.")

(defconst greet-version "1.0")

(cl-defstruct greet-entry name time)

(defun greet-format (name)
  "Format a greeting for NAME."
  (let ((prefix "Hello"))
    (format "%s %s" prefix name)))

(defsubst greet-inc ()
  (setq greet-count (1+ greet-count)))

(defmacro greet-when (cond &rest body)
  `(if ,cond (progn ,@body)))

(define-minor-mode greet-mode
  "Greet on activation."
  :lighter " Greet")

(defun greet (&optional name)
  "Greet NAME."
  (interactive)
  (greet-inc)
  (greet-when greet-mode
    (message "%s" (greet-format (or name greet-name)))))

(provide 'greet)
