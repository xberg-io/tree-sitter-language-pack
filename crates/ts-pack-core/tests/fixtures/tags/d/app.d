module app;

struct Point { int x; int y; }

class Shape {
    double area() { return 0; }
}

interface Drawable { void draw(); }

enum Color { red, green }

int add(int a, int b) { return a + b; }

void main() { add(1, 2); }
