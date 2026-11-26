const std = @import("std");

pub const max_sides: u32 = 8;

var counter: u32 = 0;

const Point = struct {
    x: i32,
    y: i32,

    pub fn sum(self: Point) i32 {
        return self.x + self.y;
    }
};

const Color = enum { red, green };

const Shape = union(enum) {
    point: Point,
    radius: f32,
};

fn add(a: i32, b: i32) i32 {
    const total = a + b;
    return total;
}

pub fn main() void {
    const p = Point{ .x = 1, .y = 2 };
    std.debug.print("{d}\n", .{add(p.sum(), 1)});
}

test "add" {
    try std.testing.expect(add(1, 2) == 3);
}
