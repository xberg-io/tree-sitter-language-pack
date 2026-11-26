package main

import "core:fmt"

LIMIT :: 8

counter := 0

@(private)
SCALE : f32 : 2.0

total: int

Point :: struct {
	x, y: int,
}

Color :: enum {
	Red,
	Green,
}

Shape :: union {
	Point,
	int,
}

Score :: distinct int

add :: proc(a, b: int) -> int {
	sum := a + b
	return sum
}

main :: proc() {
	fmt.println(add(1, 2))
}
