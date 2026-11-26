protocol Shape {
    func area() -> Double
}

struct Circle: Shape {
    let r: Double
    var diameter: Double { r * 2 }

    func area() -> Double {
        return Double.pi * r * r
    }
}

class Canvas {
    func draw(_ shape: Shape) {}
}

func total(_ shapes: [Shape]) -> Double {
    shapes.reduce(0) { $0 + $1.area() }
}
