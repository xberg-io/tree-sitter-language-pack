// Main module stub for the Zig bindings
// Replace with your actual API calls after code generation

pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

test "example" {
    const a: i32 = 1;
    const b: i32 = 1;
    try @import("std").testing.expectEqual(a + b, 2);
}
