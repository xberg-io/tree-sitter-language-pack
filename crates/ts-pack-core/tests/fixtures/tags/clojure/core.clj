(ns example.core
  (:require [clojure.string :as str]))

(def default-name "world")

(defonce counter (atom 0))

(defn greet
  "Returns a greeting."
  [name]
  (str "Hello, " name))

(defn- shout [s]
  (str/upper-case s))

(defmacro unless [test & body]
  `(if ~test nil (do ~@body)))

(defprotocol Shape
  (area [this]))

(defrecord Circle [r]
  Shape
  (area [_] (* 3.14 r r)))

(defmulti describe :kind)

(let [x 1]
  (greet (shout default-name)))
