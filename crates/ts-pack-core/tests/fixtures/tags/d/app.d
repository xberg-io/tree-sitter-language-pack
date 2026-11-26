module geo.shapes;

import std.math : sqrt;
import std.stdio;

enum maxShapes = 64;
immutable double unit = 1.0;
int shapeCount = 0;

alias Points = Point[];

enum Color { red, green = 2 }

enum { north, south }

union Value {
    int i;
    double d;
}

struct Point {
    double x;
    double y;

    this(double x, double y) {
        this.x = x;
        this.y = y;
    }

    double norm() const {
        return sqrt(x * x + y * y);
    }
}

interface Drawable {
    void draw();
}

abstract class Shape : Drawable {
    protected string label;

    this(string label) {
        this.label = label;
    }

    this() {
        this("shape");
    }

    ~this() {}

    abstract double area();

    void draw() {
        writeln(label, area());
    }
}

class Circle : Shape {
    private double r;

    this(double r) {
        super("circle");
        this.r = r;
    }

    override double area() {
        double squared = r * r;
        return 3.14 * squared;
    }
}

template Pair(T) {
    alias Pair = T[2];
}

mixin template Counted() {
    int count;
}

T clamp(T)(T value, T low, T high) {
    return value < low ? low : (value > high ? high : value);
}

int add(int a, int b) {
    return a + b;
}

int add(int a, int b, int c) {
    return add(add(a, b), c);
}

void main() {
    auto c = new Circle(2.0);
    c.draw();
    int total = add(clamp(3, 0, 10), maxShapes);
}
