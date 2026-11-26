package geo

const val MAX = 10

typealias Points = List<Point>

interface Shape {
    fun area(): Double
}

data class Point(val x: Int, val y: Int)

class Circle(private val r: Double) : Shape {
    override fun area(): Double = Math.PI * r * r
}

fun total(shapes: List<Shape>): Double = shapes.sumOf { it.area() }
