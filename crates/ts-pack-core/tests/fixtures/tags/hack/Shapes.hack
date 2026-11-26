namespace Geometry;

const float PI_APPROX = 3.14;

enum Color: string {
  RED = 'red';
  GREEN = 'green';
}

interface Shape {
  public function area(): float;
}

trait Describable {
  public function describe(): string {
    return \get_class($this);
  }
}

type Point = shape('x' => int, 'y' => int);

final class Circle implements Shape {
  use Describable;

  const int SIDES = 0;
  private float $radius;

  public function __construct(float $radius) {
    $this->radius = $radius;
  }

  public function area(): float {
    return PI_APPROX * square($this->radius);
  }

  public static function unit(): Circle {
    return new Circle(1.0);
  }
}

function square(float $x): float {
  return $x * $x;
}

function total_area(vec<Shape> $shapes): float {
  $sum = 0.0;
  foreach ($shapes as $shape) {
    $sum += $shape->area();
  }
  return $sum + Circle::unit()->area();
}
