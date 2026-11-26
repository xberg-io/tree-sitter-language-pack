package geometry;

import haxe.ds.StringMap;

var defaultRadius:Float = 1.0;

function describe(shape:Shape):String {
    return Std.string(shape.area());
}

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

interface Scalable extends Shape {
    function scale(factor:Float):Void;
}

enum abstract Unit(String) {
    var Meters = "m";
    var Feet = "ft";
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

class UnitCircle extends Circle {
    public function new() {
        super(1.0);
    }
}

class Main {
    static function main() {
        var circle = new Circle(2.0);
        trace(circle.area());
    }
}
