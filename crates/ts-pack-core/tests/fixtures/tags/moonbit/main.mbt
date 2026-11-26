///| A point in the plane.
pub(all) struct Point {
  x : Int
  y : Int
  fn new(x~ : Int, y~ : Int) -> Point
}

pub(all) enum Color {
  Red
  Green
  Rgb(Int, Int, Int)
}

type UserId = Int

suberror ParseError String

typealias Int as Score

pub trait Shape {
  area(Self) -> Double
}

const MAX_SIZE : Int = 100

let origin : Point = { x: 0, y: 0 }

fn Point::norm(self : Point) -> Int {
  let sq = self.x * self.x
  sq + self.y * self.y
}

impl Shape for Point with area(self) {
  self.norm().to_double()
}

fn add(a : Int, b : Int) -> Score {
  a + b
}

fn main {
  let p = Point::new(x=1, y=2)
  println(p.norm())
  println(add(MAX_SIZE, origin.x))
}
