package main

import "core:fmt"

LIMIT :: 8

counter := 0

@(private)
SCALE : f32 : 2.0

total: int

Point :: struct {
	x, y: int,
	label: string,
}

Color :: enum {
	Red,
	Green,
}

Shape :: union {
	Point,
	int,
}

Flags :: bit_field u8 {
	visible: bool | 1,
	dirty:   bool | 1,
}

Score :: distinct int

add :: proc(a, b: int) -> int {
	sum := a + b
	helper :: proc() {}
	return sum
}

area_int :: proc(x: int) -> int { return x * x }
area_f32 :: proc(x: f32) -> f32 { return x * x }
area :: proc{area_int, area_f32}

main :: proc() {
	fmt.println(add(1, 2), area(3))
}
