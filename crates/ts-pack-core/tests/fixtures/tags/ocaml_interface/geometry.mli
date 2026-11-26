type point = { x : int; y : int }

type shape = Circle of float | Square of float

val area : shape -> float

module type S = sig
  val name : string
end
