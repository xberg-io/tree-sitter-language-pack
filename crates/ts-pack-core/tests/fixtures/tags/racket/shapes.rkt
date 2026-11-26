#lang racket

(provide area)

(define pi-ish 3.14)

(define square (lambda (x) (* x x)))

(define (area r)
  (* pi-ish r r))

(define-syntax-rule (unless* c body)
  (if c (void) body))

(struct point (x y))

(define (main)
  (let ([r 2])
    (displayln (area r))))

(module helpers racket/base
  (provide twice)
  (define (twice x) (* 2 x)))
