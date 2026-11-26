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

private
  helper : Nat -> Nat
  helper n = suc n

abstract
  hidden : Nat
  hidden = zero

mutual
  even : Nat -> Nat
  even zero = suc zero
  even (suc n) = odd n

  odd : Nat -> Nat
  odd zero = zero
  odd (suc n) = even n

record Show (A : Set) : Set where
  field
    show : A -> Nat

instance
  showNat : Show Nat
  showNat = record { show = double }
