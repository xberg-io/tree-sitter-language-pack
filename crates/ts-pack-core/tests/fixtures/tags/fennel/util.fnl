;; utility helpers
(local max-items 10)

(var counter 0)

(global VERSION "0.1.0")

(fn add [a b]
  (+ a b))

(fn clamp [x]
  (let [limit max-items]
    (if (> x limit) limit x)))

(lambda greet [name]
  (print (.. "hi " name)))

(macro unless [cond ...]
  `(if (not ,cond) (do ,...)))

(set counter (add counter 1))

{: add : clamp}
