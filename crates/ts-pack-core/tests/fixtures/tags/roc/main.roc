app [main!] { pf: platform "https://example.com/pf.tar.br" }

import pf.Stdout
import Shapes exposing [area]

Color : [Red, Green, Blue]

Point : { x : F64, y : F64 }

Counter := U64

pi = 3.14159

origin : Point
origin = { x: 0, y: 0 }

add : I64, I64 -> I64
add = |a, b| a + b

scale = |point, factor|
    { x: point.x * factor, y: point.y * factor }

apply = |ops, value| ops.run(value)

describe = |color|
    when color is
        Red -> "red"
        Green -> "green"
        Blue -> "blue"

main! = |_args|
    Stdout.line!(describe(Red))
