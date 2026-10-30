const std = @import("std");
const tslp = @import("tree_sitter_language_pack");

// These tests require the native `ts_pack_core_ffi` library to have been built
// with at least `python` statically compiled in (TSLP_LANGUAGES must include
// "python"). The CI workflow (.github/workflows/ci-zig.yaml) builds with
// TSLP_LANGUAGES=python,rust,javascript,typescript,go,html,css,json,mojo,nim,norg,
// so "python" is guaranteed present without any network access. Do not add
// assertions here that depend on languages outside that set, or on
// auto-download (`get_language`/`has_language` for genuinely unknown names
// resolve to `LanguageNotFound` from the manifest lookup itself, before any
// network call is attempted). ~keep

test "should_return_module_root_kind_when_parsing_valid_python_source" {
    const source = "x = 1\n";

    var parser = try tslp.get_parser("python");
    defer parser.free();

    // ~keep A null tree means no grammar was linked in. That is a failure, not a
    // skip: `zig build test` exits 0 on skipped tests, so skipping here would
    // restore the green-on-nothing suite this file exists to replace.
    var tree = (try parser.parse(source)) orelse
        return error.ParserReturnedNoTree;
    defer tree.free();

    var root = try tree.root_node();
    defer root.free();

    const kind = try root.kind();
    defer std.heap.c_allocator.free(kind);
    try std.testing.expectEqualStrings("module", kind);

    try std.testing.expect(try root.is_named());
    try std.testing.expect(!(try root.has_error()));
    try std.testing.expectEqual(@as(u64, 0), try root.start_byte());
    try std.testing.expectEqual(@as(u64, source.len), try root.end_byte());
    try std.testing.expect((try root.child_count()) > 0);
}

test "should_report_error_nodes_when_parsing_syntactically_invalid_python_source" {
    // Unterminated function signature: tree-sitter must recover with a
    // partial tree carrying ERROR/MISSING nodes, not fail parsing outright.
    const source = "def broken(\n";

    var parser = try tslp.get_parser("python");
    defer parser.free();

    var tree = (try parser.parse(source)) orelse
        return error.ParserReturnedNoTree;
    defer tree.free();

    var root = try tree.root_node();
    defer root.free();

    try std.testing.expect(try root.has_error());
}

test "should_map_py_extension_to_python_when_detecting_language_from_extension" {
    const got = try tslp.detect_language_from_extension("py");
    try std.testing.expect(got != null);
    defer std.heap.c_allocator.free(got.?);
    try std.testing.expectEqualStrings("python", got.?);
}

test "should_report_local_python_parser_availability_without_network_when_queried_via_registry" {
    var registry = try tslp.new_language_registry();
    defer registry.free();

    // `has_parser` is documented to never perform network I/O, so this is a
    // deterministic, offline assertion that the statically compiled "python"
    // grammar this test suite depends on is actually linked in.
    try std.testing.expect(try registry.has_parser("python"));
}

test "should_report_parser_unavailable_when_registry_queried_for_unknown_language" {
    var registry = try tslp.new_language_registry();
    defer registry.free();

    try std.testing.expect(!(try registry.has_parser("this-language-definitely-does-not-exist-xyz")));
}

test "should_not_return_a_language_when_getting_unknown_language" {
    // ~keep `get_language` documents two failure modes for a name it cannot
    // supply: `LanguageNotFound` when the manifest is readable and does not
    // list it, and `Download` when the manifest itself cannot be fetched --
    // which is what CI hits, because the release artifacts for the version
    // under test are not published while that version is being tested. Pinning
    // either one makes the assertion depend on network reachability, so this
    // asserts the only thing that is invariant: the call does not hand back a
    // language. A `null` payload is the third shape of the same outcome.
    if (tslp.get_language("this-language-definitely-does-not-exist-xyz")) |maybe_language| {
        try std.testing.expect(maybe_language == null);
    } else |err| switch (err) {
        tslp.Error.LanguageNotFound, tslp.Error.Download => {},
        else => return err,
    }
}

test "should_report_nonzero_language_count_when_global_registry_queried" {
    try std.testing.expect((tslp.language_count()) > 0);
    try std.testing.expect(try tslp.has_language("python"));
}
