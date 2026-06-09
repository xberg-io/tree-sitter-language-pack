```zig title="Zig"
const std = @import("std");
const tslp = @import("tree_sitter_language_pack");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    _ = gpa.allocator();

    var parser = try tslp.get_parser("rust");
    defer parser.free();

    const source = "fn main() { println!(\"hello\"); }";
    var tree = (try parser.parse(source)) orelse return error.ParseFailed;
    defer tree.free();

    var root = tree.root_node();
    defer root.free();

    const kind = try root.kind();
    defer std.heap.c_allocator.free(kind);
    std.debug.print("root kind: {s}\n", .{kind});
}
```
