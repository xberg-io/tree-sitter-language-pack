module Vect

import Data.Nat

data Vect : Nat -> Type -> Type where
  Nil  : Vect Z a
  (::) : a -> Vect k a -> Vect (S k) a

record Point where
  constructor MkPoint
  x : Double
  y : Double

interface Shape a where
  area : a -> Double

implementation Shape Point where
  area p = 0

length : Vect n a -> Nat
length Nil = Z
length (x :: xs) = S (length xs)
  where
    helper : Nat -> Nat
    helper n = n

origin : Point
origin = MkPoint 0 0
