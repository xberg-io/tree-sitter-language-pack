# small helpers
(def max-items 10)

(var counter 0)

(defn add [a b]
  (+ a b))

(defn- clamp [x]
  (let [limit max-items]
    (if (> x limit) limit x)))

(defmacro unless [cond & body]
  ~(if ,cond nil (do ,;body)))

(set counter (add counter 1))
