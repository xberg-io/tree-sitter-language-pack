require "json"

MAX_SIDES = 8

module Geometry
  abstract class Shape
    abstract def area : Float64
  end

  struct Point
    getter x : Int32
    getter y : Int32

    def initialize(@x, @y)
    end
  end

  class Square < Shape
    def initialize(@side : Float64)
    end

    def area : Float64
      result = @side * @side
      result
    end
  end

  enum Color
    Red
    Green
  end

  alias Coord = Tuple(Int32, Int32)
end

class Box(T)
  def self.wrap(value : T) : Box(T)
    new(value)
  end

  def initialize(@value : T)
  end
end

def describe(shape)
  puts shape.area
end
