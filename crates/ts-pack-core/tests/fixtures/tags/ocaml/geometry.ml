(** Plane geometry. *)

module type SHAPE = sig
  type t
  val area : t -> float
  val name : string
end

module Geometry = struct
  type point = { x : float; y : float }

  type shape = Circle of float | Square of float

  type size = float

  let pi = 3.14159

  let area = function
    | Circle r -> pi *. r *. r
    | Square s -> s *. s

  let rec perimeter shape =
    let double v = 2.0 *. v in
    match shape with
    | Circle r -> double (pi *. r)
    | Square s -> 4.0 *. s
end

module Circle_shape : SHAPE = struct
  type t = float
  let name = "circle"
  let area r = Geometry.area (Geometry.Circle r)
end

external hypot : float -> float -> float = "caml_hypot_float"

exception Invalid_shape of string

let ( +: ) a b = { Geometry.x = a; y = b }

module Names = Set.Make (String)

type names = Set.Make(String).t

class type printable = object
  method print : unit
end

let show (p : #printable) = p#print

class counter start = object (self)
  val mutable count : int = start
  method increment step = count <- count + step; self#report
  method report = count
end

let main () =
  let c = new counter 0 in
  c#increment 2;
  Geometry.perimeter (Geometry.Square 2.0) |> print_float;
  print_endline @@ string_of_float (hypot 3.0 4.0)
