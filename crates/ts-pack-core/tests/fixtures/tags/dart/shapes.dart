import 'dart:math' as math;

const defaultRadius = 1.0;
const List<String> unitNames = ['m', 'in'];
final registry = <String, Shape>{};
var shapeCount = 0;

typedef AreaCallback = void Function(double area);
typedef Mapper<T> = T Function(T value);
typedef int Compare(Shape a, Shape b);

enum Unit { metric, imperial }

mixin Describable on Shape {
  String describe() => 'shape with area ${area()}';
}

abstract class Shape {
  static const sides = 0;

  double area();

  double get perimeter;
}

class Circle extends Shape with Describable implements Comparable<Circle> {
  final double radius;
  late Unit unit;

  Circle(this.radius);

  Circle.unit() : this(1.0);

  const Circle.zero() : radius = 0;

  factory Circle.fromDiameter(double diameter) => Circle(diameter / 2);

  factory Circle.parse(String text) = Circle.fromText;

  @override
  double area() => math.pi * square(radius);

  @override
  double get perimeter => 2 * math.pi * radius;

  set scale(double factor) {}

  double square(double value) => value * value;

  int compareTo(Circle other) => area().compareTo(other.area());
}

extension CircleMath on Circle {
  double get diameter => radius * 2;
}

double get totalArea => 0;

set lastShape(Shape shape) {}

double sumAreas(List<Shape> shapes) {
  var sum = 0.0;
  for (final shape in shapes) {
    sum += shape.area();
  }
  return sum;
}

void main() {
  final circle = Circle(2.0);
  final shapes = <Shape>[]..add(circle);
  print(sumAreas(shapes));
  print(circle?.area());
}
