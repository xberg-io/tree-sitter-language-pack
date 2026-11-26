module Geometry

type Point = { X: int; Y: int }

type Shape =
    | Circle of float
    | Square of float

let area shape =
    match shape with
    | Circle r -> 3.14 * r * r
    | Square s -> s * s

let origin = { X = 0; Y = 0 }
