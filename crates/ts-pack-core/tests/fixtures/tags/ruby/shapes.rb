module Geometry
  class Shape
    def area
      raise NotImplementedError
    end
  end

  class Circle < Shape
    def initialize(r)
      @r = r
    end

    def area
      Math::PI * @r**2
    end

    def self.unit
      new(1)
    end
  end
end

puts Geometry::Circle.unit.area
