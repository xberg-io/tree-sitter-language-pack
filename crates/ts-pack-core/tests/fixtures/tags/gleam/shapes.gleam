import gleam/io

pub type Shape {
  Circle(radius: Float)
  Square(side: Float)
}

pub fn area(shape: Shape) -> Float {
  case shape {
    Circle(r) -> 3.14 *. r *. r
    Square(s) -> s *. s
  }
}

pub fn main() {
  io.println("hi")
}
