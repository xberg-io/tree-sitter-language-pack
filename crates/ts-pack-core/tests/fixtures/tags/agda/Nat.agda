module Nat where

data Nat : Set where
  zero : Nat
  suc  : Nat -> Nat

record Pair (A B : Set) : Set where
  constructor _,_
  field
    fst : A
    snd : B

postulate
  Axiom : Set

_+_ : Nat -> Nat -> Nat
zero  + n = n
suc m + n = suc (m + n)

double : Nat -> Nat
double n = twice n
  where
    twice : Nat -> Nat
    twice m = m + m

module Inner where
  one : Nat
  one = suc zero
