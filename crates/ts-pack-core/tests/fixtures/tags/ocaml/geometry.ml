module Geometry = struct
  type point = { x : int; y : int }

  type shape = Circle of float | Square of float

  let area = function
    | Circle r -> 3.14 *. r *. r
    | Square s -> s *. s
end

let main () = print_float (Geometry.area (Geometry.Circle 1.0))
