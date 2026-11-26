namespace Geometry

exception InvalidShape of string

type Point = { X: float; Y: float }

type Shape =
    | Circle of radius: float
    | Square of side: float

type Color =
    | Red = 0
    | Green = 1

type IShape =
    interface
        abstract member Area: unit -> float
    end

type Pair = float * float

type Ring =
    new: radius: float -> Ring
    member Radius: float
    member Scale: factor: float -> Ring

module Shapes =
    val origin: Point
    val area: Shape -> float
