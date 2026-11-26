import gleam/float
import gleam/io
import gleam/list.{map}

pub const pi = 3.14159

pub type Shape {
  Circle(radius: Float)
  Square(side: Float)
}

pub type Labelled {
  Labelled(label: String, shape: Shape)
}

pub type Area =
  Float

pub type Handle

@external(erlang, "math", "sqrt")
pub fn sqrt(x: Float) -> Float

pub fn area(shape: Shape) -> Area {
  case shape {
    Circle(r) -> pi *. r *. r
    Square(s) -> s *. s
  }
}

fn describe(item: Labelled) -> String {
  let size = area(item.shape)
  item.label <> ": " <> float.to_string(size)
}

pub fn main() {
  let shapes = [Circle(1.0), Square(2.0)]
  shapes
  |> map(area)
  |> list.each(io.debug)
  Labelled("unit", Square(1.0))
  |> describe
  |> io.println
}
