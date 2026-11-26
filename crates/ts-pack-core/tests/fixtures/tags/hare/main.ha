use fmt;

export def LIMIT: int = 8;

let counter: int = 0;

const greeting: str = "hi";

export type point = struct {
	x: int,
	y: int,
};

type color = enum {
	RED,
	GREEN = 2,
};

type number = union {
	i: int,
	f: f64,
};

type score = int;

fn add(a: int, b: int) int = {
	let sum = a + b;
	const twice = sum * 2;
	return twice;
};

export fn main() void = {
	fmt::println(add(1, 2))!;
};
