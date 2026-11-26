struct Point {
  x : Int
  y : Int
}

trait Shape {
  area(Self) -> Double
}

fn add(a : Int, b : Int) -> Int {
  a + b
}

fn main {
  println(add(1, 2))
}
