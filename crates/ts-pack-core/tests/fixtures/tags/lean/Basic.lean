namespace Shapes

inductive Shape where
  | circle (r : Float)
  | square (s : Float)

structure Point where
  x : Float
  y : Float

def area : Shape → Float
  | .circle r => 3.14 * r * r
  | .square s => s * s
  where
    helper (x : Float) : Float := x

theorem add_zero (n : Nat) : n + 0 = n := rfl

abbrev Area := Float

axiom choice_exists : ∀ (α : Type), Nonempty α → α

opaque secret : Nat

class HasArea (α : Type) where
  areaOf : α → Float

instance : HasArea Shape where
  areaOf := area

end Shapes
