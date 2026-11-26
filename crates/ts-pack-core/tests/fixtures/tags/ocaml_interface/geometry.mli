(** Plane geometry. *)

open Format

type point = { x : float; y : float }

type shape = Circle of float | Square of float

type size = float

type t

exception Invalid_shape of string

val origin : point

val area : shape -> float

val scale : size -> shape -> shape

external hypot : float -> float -> float = "caml_hypot_float"

module type SHAPE = sig
  type t
  val name : string
  val area : t -> float
end

module Circle_shape : SHAPE with type t = float

module Shapes = Stdlib.List

module Names : Set.S with type elt = String.t

class type printable = object
  method print : unit
end

class counter : int -> object
  method increment : int -> unit
  method report : int
end

val describe : #printable -> Names.t -> string
