$registry = {}

module Geometry
  VERSION = "1.0"
  Point = Struct.new(:x, :y)

  class Shape
    include Comparable

    attr_accessor :name
    attr_reader "sides"

    def area
      raise NotImplementedError
    end

    def <=>(other)
      area <=> other.area
    end
  end

  class Circle < Shape
    DEFAULT_RADIUS = 1

    def initialize(r = DEFAULT_RADIUS)
      @r = r
    end

    def area
      local = @r**2
      Math::PI * local
    end

    def self.unit
      new(1)
    end

    class << self
      def build(r)
        new(r)
      end
    end
  end
end

module Geometry::Solids
  class Sphere < Geometry::Circle
  end
end

Geometry::LIMIT = 10
puts Geometry::Circle.unit.area
