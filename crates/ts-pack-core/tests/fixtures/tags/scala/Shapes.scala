package geo

trait Shape {
  def area: Double
}

case class Circle(r: Double) extends Shape {
  def area: Double = math.Pi * r * r
}

object Shapes {
  def total(shapes: Seq[Shape]): Double = shapes.map(_.area).sum
}
