package com.example.geo

import scala.math.{Pi, sqrt}

val MaxShapes: Int = 64
var shapeCount = 0

type Points = Seq[Point]

opaque type Meters = Double

trait Shape extends Ordered[Shape] {
  val name: String
  def area: Double
  def compare(that: Shape): Int = area.compare(that.area)
}

enum Color(val hex: String) {
  case Red extends Color("#f00")
  case Green extends Color("#0f0")
}

enum Direction {
  case North, South
}

case class Point(x: Double, y: Double) {
  def norm: Double = sqrt(x * x + y * y)
}

abstract class Base(val label: String) extends Shape {
  var scale: Double = 1.0
  private val history = List.empty[Double]

  def resize(factor: Double): Unit = {
    scale = factor
  }

  def resize(factor: Int): Unit = resize(factor.toDouble)
}

class Circle(r: Double) extends Base("circle") with Serializable {
  val name = "circle"
  def area: Double = Pi * r * r * scale
}

object Circle {
  def unit(): Circle = new Circle(1.0)
}

given Ordering[Point] = Ordering.by(_.norm)

given pointOrdering: Ordering[Point] with {
  def compare(a: Point, b: Point): Int = a.norm.compare(b.norm)
}

extension (p: Point) {
  def scaled(k: Double): Point = Point(p.x * k, p.y * k)
}

def total(shapes: Seq[Shape]): Double = {
  val areas = shapes.map(_.area)
  areas.sum
}

@main def run(): Unit = {
  val circle = Circle.unit()
  println(total(Seq(circle)))
}
