namespace Geometry

open System

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

type Callback = delegate of Shape -> unit

type Ring(radius: float) =
    member this.Radius = radius
    member this.Scale(factor: float) = Ring(radius * factor)
    static member Unit() = Ring(1.0)
    interface IShape with
        member this.Area() = Math.PI * radius * radius

module Shapes =
    let origin = { X = 0.0; Y = 0.0 }

    let area shape =
        let square x = x * x
        match shape with
        | Circle r -> Math.PI * square r
        | Square s -> square s
