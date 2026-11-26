#import "Basic";

LIMIT :: 8;

counter := 0;

Vector2 :: struct {
    x: float;
    y: float;
}

Color :: enum {
    RED;
    GREEN;
}

add :: (a: int, b: int) -> int {
    sum := a + b;
    return sum;
}

main :: () {
    print("%\n", add(1, 2));
}
