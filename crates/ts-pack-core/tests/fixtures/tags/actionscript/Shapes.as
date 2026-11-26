package geometry {
    import flash.display.Sprite;

    public const VERSION:String = "1.0";
    public var instances:int = 0;

    public namespace internal_api = "http://example.com/geometry/internal";

    public interface IShape {
        function area():Number;
    }

    public interface IScalable extends IShape {
        function scaled(factor:Number):IShape;
    }

    public class Circle extends Sprite implements IScalable {
        public static const PI_APPROX:Number = 3.14;
        private var radius:Number;

        public function Circle(r:Number) {
            radius = r;
            instances++;
        }

        public function area():Number {
            return PI_APPROX * square(radius);
        }

        public function scaled(factor:Number):IShape {
            var copy:Circle = new Circle(radius * factor);
            trace(copy.area());
            return copy;
        }

        private function square(x:Number):Number {
            return x * x;
        }
    }
}

const DEFAULT_RADIUS:Number = 1;
var registry:Array = [];

function describe(shape:Object):String {
    return String(shape);
}
