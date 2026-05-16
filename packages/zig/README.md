# tree-sitter-language-pack - Zig Bindings

Pre-compiled tree-sitter grammars for 305 programming languages

## Installation

Add to `build.zig.zon`:

```zig
.dependencies = .{
    .tree_sitter_language_pack = .{ .url = "<tarball-url>" },
};
```

## Quick Start

```zig
const tree_sitter_language_pack = @import("tree_sitter_language_pack");

// Call generated wrapper functions; strings allocated by the FFI must
// be released with `tree_sitter_language_pack._free_string`.
```

## Documentation

For full documentation, see the [tree-sitter-language-pack repository](https://github.com/kreuzberg-dev/tree-sitter-language-pack).

## License

See the [LICENSE](https://github.com/kreuzberg-dev/tree-sitter-language-pack/blob/main/LICENSE) file in the root repository.
