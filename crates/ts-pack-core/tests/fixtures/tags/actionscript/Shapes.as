package geometry {
    import flash.display.Sprite;

    public interface IShape {
        function area():Number;
    }

    public class Circle extends Sprite implements IShape {
        public static const PI_APPROX:Number = 3.14;
        private var radius:Number;

        public function Circle(r:Number) {
            radius = r;
        }

        public function area():Number {
            return PI_APPROX * square(radius);
        }

        public function scaled(factor:Number):Circle {
            var copy:Circle = new Circle(radius * factor);
            trace(copy.area());
            return copy;
        }

        private function square(x:Number):Number {
            return x * x;
        }
    }
}
