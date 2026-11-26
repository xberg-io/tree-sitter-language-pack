package geometry;

import haxe.ds.StringMap;

typedef Point = {
    var x:Float;
    var y:Float;
}

enum Color {
    Red;
    Green;
}

interface Shape {
    function area():Float;
}

class Circle implements Shape {
    static inline var PI_APPROX:Float = 3.14;
    var radius:Float;

    public function new(radius:Float) {
        this.radius = radius;
    }

    public function area():Float {
        var squared = square(radius);
        return PI_APPROX * squared;
    }

    static function square(x:Float):Float {
        return x * x;
    }
}

class Main {
    static function main() {
        var circle = new Circle(2.0);
        trace(circle.area());
    }
}
