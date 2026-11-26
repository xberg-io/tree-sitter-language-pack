package com.example.geo;

import java.util.ArrayList;
import java.util.List;

@interface Audited {
    String owner();
    int version() default 1;
}

interface Shape extends Comparable<Shape> {
    double UNIT = 1.0;

    double area();

    default String describe() {
        return String.format("%s(%f)", getClass().getSimpleName(), area());
    }
}

enum Color {
    RED("#f00"),
    GREEN("#0f0");

    private final String hex;

    Color(String hex) {
        this.hex = hex;
    }

    String hex() {
        return hex;
    }
}

record Point(double x, double y) {
    Point {
        if (Double.isNaN(x)) {
            throw new IllegalArgumentException("x");
        }
    }

    double norm() {
        return Math.sqrt(x * x + y * y);
    }
}

@Audited(owner = "geo")
public abstract class Base<T extends Shape> implements Shape {
    public static final int MAX_SHAPES = 64;
    protected final List<T> children = new ArrayList<>();
    private int count;

    protected Base() {
        this.count = 0;
    }

    @Override
    public int compareTo(Shape other) {
        return Double.compare(area(), other.area());
    }

    static class Registry {
        private final List<Shape> shapes = new ArrayList<>();

        void register(Shape shape) {
            shapes.add(shape);
        }
    }

    interface Visitor {
        void visit(Shape shape);
    }
}

class Circle extends Base<Circle> {
    private final double r;

    Circle(double r) {
        super();
        this.r = r;
    }

    Circle() {
        this(1.0);
    }

    Circle scale(double factor) {
        return new Circle(r * factor);
    }

    Circle scale(int factor) {
        return scale((double) factor);
    }

    @Override
    public double area() {
        double squared = Math.pow(r, 2);
        return squared * Math.PI;
    }

    public static <S extends Shape> S largest(List<S> shapes) {
        S best = null;
        for (S shape : shapes) {
            if (best == null || shape.compareTo(best) > 0) {
                best = shape;
            }
        }
        return best;
    }
}
