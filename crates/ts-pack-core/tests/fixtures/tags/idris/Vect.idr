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

namespace Geometry
  scale : Double -> Point -> Point
  scale k (MkPoint a b) = MkPoint (k * a) (k * b)

mutual
  isEven : Nat -> Bool
  isEven Z = True
  isEven (S k) = isOdd k

  isOdd : Nat -> Bool
  isOdd Z = False
  isOdd (S k) = isEven k

parameters (unit : Double)
  shift : Point -> Point
  shift (MkPoint a b) = MkPoint (a + unit) b
