import Foundation

let defaultRadius = 1.0
var shapeCount = 0

typealias AreaCallback = (Double) -> Void

protocol Shape {
    associatedtype Unit
    var name: String { get }
    init()
    func area() -> Double
    subscript(index: Int) -> Double { get }
}

protocol Solid: Shape {}

enum Measure: String {
    case metric, imperial
    case custom

    var label: String { rawValue }

    func describe() -> String {
        return label.uppercased()
    }

    subscript(scale: Int) -> Double {
        return Double(scale)
    }
}

struct Circle<T>: Shape {
    typealias Unit = Double

    static let unitRadius = 1.0
    let r: Double
    var diameter: Double { r * 2 }

    init() {
        self.r = Circle.unitRadius
    }

    func area() -> Double {
        let local = r * r
        return Double.pi * local
    }

    subscript(index: Int) -> Double {
        return area() * Double(index)
    }

    struct Style {
        var color: String
    }
}

actor Store {
    var shapes: [Circle<Double>] = []
}

class Canvas {
    deinit {}

    func draw(_ shape: Circle<Double>) {
        print(shape.area())
    }
}

extension Canvas: CustomStringConvertible {
    var description: String { "canvas" }
}

func total(_ shapes: [Circle<Double>]) -> Double {
    shapes.reduce(0) { $0 + $1.area() }
}
