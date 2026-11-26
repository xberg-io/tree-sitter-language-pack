abstract class Shape {
  double area();
}

class Circle extends Shape implements Comparable<Circle> {
  final double radius;

  Circle(this.radius);

  @override
  double area() => 3.14 * square(radius);

  double square(double value) => value * value;

  int compareTo(Circle other) => area().compareTo(other.area());
}

double totalArea(List<Shape> shapes) {
  var sum = 0.0;
  for (final shape in shapes) {
    sum += shape.area();
  }
  return sum;
}

void main() {
  final circle = Circle(2.0);
  final shapes = <Shape>[]..add(circle);
  print(totalArea(shapes));
  print(circle?.area());
}
