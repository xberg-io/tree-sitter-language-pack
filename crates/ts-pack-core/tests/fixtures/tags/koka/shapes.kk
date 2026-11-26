module shapes

import std/num/float64

type shape
  Circle(radius : float64)
  Square(side : float64)

struct point(x : float64, y : float64)

effect ask
  ctl ask() : int

alias area = float64

val pi-ish = 3.14

fun area(s : shape) : float64
  match s
    Circle(r) -> pi-ish * r * r
    Square(d) -> d * d

fun main()
  println(area(Circle(1.0)))
