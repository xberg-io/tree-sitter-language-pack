;; shapes
(define pi-ish 3.14)

(define square (lambda (x) (* x x)))

(define (area r)
  (* pi-ish r r))

(define-syntax swap!
  (syntax-rules ()
    ((_ a b) (let ((tmp a)) (set! a b) (set! b tmp)))))

(define-record-type point
  (make-point x y)
  point?
  (x point-x))

(define (main)
  (let ((r 2))
    (display (area r))))
