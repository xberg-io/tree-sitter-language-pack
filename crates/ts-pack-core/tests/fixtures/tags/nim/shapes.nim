import std/math

const PiApprox = 3.14

type
  Shape = ref object of RootObj
    name: string
  Circle = ref object of Shape
    radius: float
  Color = enum
    red, green

var created = 0

proc square(x: float): float =
  x * x

method area(s: Shape): float {.base.} =
  0.0

method area(c: Circle): float =
  PiApprox * square(c.radius)

func describe(s: Shape): string =
  s.name & " with area " & $s.area()

template twice(body: untyped) =
  body
  body

iterator countdown(n: int): int =
  var i = n
  while i > 0:
    yield i
    dec i

let circle = Circle(name: "c", radius: 2.0)
echo describe(circle)
