package geo;

interface Shape {
    double area();
}

class Circle implements Shape {
    private final double r;

    Circle(double r) {
        this.r = r;
    }

    public double area() {
        return Math.pow(r, 2) * Math.PI;
    }
}
