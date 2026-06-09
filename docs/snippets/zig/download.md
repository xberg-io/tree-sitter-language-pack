```zig title="Zig"
const std = @import("std");
const tslp = @import("tree_sitter_language_pack");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    _ = gpa.allocator();

    // Pre-download specific languages (names passed as JSON array).
    const count = try tslp.download("[\"python\", \"javascript\", \"rust\"]");
    std.debug.print("languages available after download: {d}\n", .{count});

    // Inspect what is cached locally — returned as a JSON array string.
    const installed = try tslp.downloaded_languages();
    defer std.heap.c_allocator.free(installed);
    std.debug.print("downloaded: {s}\n", .{installed});

    // Report the effective cache directory.
    const dir = try tslp.cache_dir();
    defer std.heap.c_allocator.free(dir);
    std.debug.print("cache dir: {s}\n", .{dir});
}
```
