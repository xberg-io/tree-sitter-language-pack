module main

import os

const max_sides = 8

__global (
	calls int
)

__global counter = 0

struct Point {
	x int
	y int
}

enum Color {
	red
	green
}

interface Shape {
	area() f64
}

type Score = int

fn (p Point) sum() int {
	return p.x + p.y
}

fn add(a int, b int) int {
	total := a + b
	return total
}

fn main() {
	p := Point{1, 2}
	println(add(p.sum(), 1))
	os.exit(0)
}
