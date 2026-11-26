use fmt;

export def LIMIT: int = 8;

let counter: int = 0;

type point = struct {
	x: int,
	y: int,
};

type color = enum {
	RED,
	GREEN,
};

fn add(a: int, b: int) int = {
	let sum = a + b;
	return sum;
};

export fn main() void = {
	fmt::println(add(1, 2))!;
};
