```zig title="Zig"
const std = @import("std");
const tslp = @import("tree_sitter_language_pack");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const source = "def hello():\n    pass\n\nimport os\n";
    const config_json =
        \\{"language":"python","structure":true,"imports":true,"exports":false,
        \\"comments":false,"docstrings":false,"symbols":false,"diagnostics":false,
        \\"chunk_max_size":null}
    ;

    const result_json = try tslp.process(source, config_json);
    defer std.heap.c_allocator.free(result_json);

    var parsed = try std.json.parseFromSlice(std.json.Value, allocator, result_json, .{});
    defer parsed.deinit();

    const structure = parsed.value.object.get("structure").?.array;
    for (structure.items) |item| {
        const kind_value = item.object.get("kind").?;
        const kind_name = switch (kind_value) {
            .string => |s| s,
            .object => |obj| obj.keys()[0],
            else => "unknown",
        };
        const name_value = item.object.get("name") orelse std.json.Value{ .null = {} };
        const name_str = if (name_value == .string) name_value.string else "<anonymous>";
        std.debug.print("{s}: {s}\n", .{ kind_name, name_str });
    }
}
```
