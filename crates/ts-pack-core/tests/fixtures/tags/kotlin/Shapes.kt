package com.example.geo

import kotlin.math.PI
import kotlin.math.sqrt

const val MAX_SHAPES = 64
val defaultName = "shape"
var shapeCount = 0

typealias Points = List<Point>

interface Shape {
    val name: String
    fun area(): Double
}

data class Point(val x: Double, val y: Double) {
    fun norm(): Double = sqrt(x * x + y * y)
}

enum class Color(val hex: String) {
    RED("#f00"),
    GREEN("#0f0");

    fun shade(): String = hex.lowercase()
}

sealed class Result {
    data class Ok(val value: Double) : Result()
    object Empty : Result()
}

@JvmInline
value class Meters(val amount: Double)

open class Circle(private val r: Double) : Shape, Comparable<Circle> {
    override val name = "circle"
    private var scale = 1.0

    constructor(diameter: Int) : this(diameter / 2.0)

    override fun area(): Double = PI * r * r * scale

    fun resize(factor: Double) {
        scale = factor
    }

    fun resize(factor: Int) {
        resize(factor.toDouble())
    }

    override fun compareTo(other: Circle): Int = area().compareTo(other.area())

    companion object Factory {
        const val UNIT = 1.0
        fun unit(): Circle = Circle(UNIT)
    }
}

object Registry {
    private val shapes = mutableListOf<Shape>()

    fun register(shape: Shape) {
        shapes.add(shape)
    }
}

fun Shape.describe(): String = "$name: ${area()}"

fun total(shapes: Points): Double {
    val sum = shapes.sumOf { it.norm() }
    return sum
}

fun main() {
    val circle = Circle.unit()
    Registry.register(circle)
    println(circle.describe())
}
