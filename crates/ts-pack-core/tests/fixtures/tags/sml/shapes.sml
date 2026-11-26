structure Shapes =
struct
  datatype shape = Circle of real | Square of real

  type area = real

  val pi = 3.14

  fun area (Circle r) = pi * r * r
    | area (Square s) = s * s

  exception NotFound

  fun scaled k =
    let
      fun helper x = x * k
    in
      helper 2.0
    end
end

signature SHAPE =
sig
  val area : Shapes.shape -> real
end

functor MakeShape (S : SHAPE) =
struct
  val f = S.area
end
