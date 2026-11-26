#include <cmath>

namespace geo {

class Shape {
 public:
  virtual double area() const = 0;
};

class Circle : public Shape {
 public:
  double area() const override { return 3.14 * r * r; }
  double r = 1.0;
};

double total(const Shape& s) {
  return s.area();
}

}  // namespace geo

int main() {
  geo::Circle c;
  return static_cast<int>(geo::total(c));
}
