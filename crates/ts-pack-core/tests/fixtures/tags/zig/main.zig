const std = @import("std");

pub const max_sides: u32 = 8;

var counter: u32 = 0;

const Point = struct {
    x: i32,
    y: i32,

    pub const origin = Point{ .x = 0, .y = 0 };

    pub fn sum(self: Point) i32 {
        return self.x + self.y;
    }
};

const Color = enum {
    red,
    green,

    pub fn isRed(self: Color) bool {
        return self == .red;
    }
};

const Shape = union(enum) {
    point: Point,
    radius: f32,
};

const Handle = opaque {
    pub fn close(handle: *Handle) void {
        _ = handle;
    }
};

const FileError = error{ NotFound, AccessDenied };

fn add(a: i32, b: i32) i32 {
    const total = a + b;
    return total;
}

pub fn main() void {
    const p = Point{ .x = 1, .y = 2 };
    std.debug.print("{d}\n", .{add(p.sum(), 1)});
}

test "addition" {
    try std.testing.expect(add(1, 2) == 3);
}
