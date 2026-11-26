(define-constant contract-owner tx-sender)
(define-constant err-not-owner (err u100))

(define-data-var counter uint u0)
(define-map balances principal uint)
(define-fungible-token token)
(define-non-fungible-token badge uint)

(define-trait counter-trait
  ((increment () (response uint uint))))

(define-private (is-owner)
  (is-eq tx-sender contract-owner))

(define-public (increment)
  (begin
    (asserts! (is-owner) err-not-owner)
    (ok (var-set counter (+ (var-get counter) u1)))))

(define-read-only (get-counter)
  (ok (var-get counter)))
