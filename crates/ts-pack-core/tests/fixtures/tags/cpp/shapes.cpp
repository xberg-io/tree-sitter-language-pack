#include <cmath>
#include <string>
#include <vector>

#define GEO_VERSION 3
#define SQUARE(x) ((x) * (x))

namespace geo {

constexpr double kPi = 3.14159;
const int kMaxShapes = 64;
int shape_count = 0;
extern int external_count;

typedef unsigned int shape_id;
using Name = std::string;

enum class Color { Red, Green = 2, Blue };

union Value {
  int i;
  double d;
};

struct Point {
  double x;
  double y;

  Point(double x, double y) : x(x), y(y) {}
  Point() : Point(0, 0) {}
  Point scaled(double k) const { return Point(x * k, y * k); }
  Point scaled(int k) const { return scaled(static_cast<double>(k)); }
  double norm() const;
};

class Shape {
 public:
  virtual ~Shape() = default;
  virtual double area() const = 0;
  const Name& name() const { return label; }
  Name label;
};

class Circle : public Shape {
 public:
  explicit Circle(double r);
  double area() const override { return kPi * SQUARE(radius_); }

 private:
  double radius_;
};

template <typename T>
class Registry : public std::vector<T> {
 public:
  void add(const T& item) { this->push_back(item); }
};

template <typename T>
T clamp(T value, T low, T high) {
  return std::min(std::max(value, low), high);
}

namespace detail::math {
double hypot2(double a, double b) { return a * a + b * b; }
}  // namespace detail::math

double Point::norm() const {
  double sum = detail::math::hypot2(x, y);
  return std::sqrt(sum);
}

Circle::Circle(double r) : radius_(r) {}

Shape* make_circle(double r) {
  return new Circle(r);
}

double total(const Shape& shape) {
  return shape.area();
}

double total(const std::vector<Shape*>& shapes) {
  double sum = 0;
  for (const Shape* shape : shapes) {
    sum += shape->area();
  }
  return sum;
}

}  // namespace geo

int main() {
  geo::Circle circle(2.0);
  int count = geo::clamp<int>(3, 0, 10);
  delete geo::make_circle(1.0);
  return static_cast<int>(circle.area()) + count;
}
