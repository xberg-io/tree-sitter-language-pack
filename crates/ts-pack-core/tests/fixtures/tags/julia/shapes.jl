module Shapes

export Circle, area

const PI_APPROX = 3.14

abstract type Shape end

primitive type Word24 <: Unsigned 24 end

struct Circle <: Shape
    radius::Float64
end

mutable struct Counter
    count::Int
end

macro twice(expr)
    return :($expr; $expr)
end

function area(c::Circle)
    return PI_APPROX * square(c.radius)
end

square(x) = x * x

function bump!(counter::Counter)
    counter.count += 1
    @twice counter.count += 1
    Base.println(counter.count)
end

end
