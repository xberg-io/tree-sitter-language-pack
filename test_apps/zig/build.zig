const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");

    // Select the platform-specific dependency based on build host.
    const pkg_name = if (builtin.target.os.tag == .linux) (
        if (builtin.target.cpu.arch == .x86_64) "tree_sitter_language_pack_linux_x86_64" else "tree_sitter_language_pack_linux_aarch64")
    else if (builtin.target.os.tag == .macos) (
        if (builtin.target.cpu.arch == .x86_64) "tree_sitter_language_pack_macos_amd64" else "tree_sitter_language_pack_macos_arm64")
    else if (builtin.target.os.tag == .windows) "tree_sitter_language_pack_windows_x64" else @compileError("unsupported platform for this Zig package");

    const tree_sitter_language_pack_module = b.dependency(pkg_name, .{
        .target = target,
        .optimize = optimize,
    }).module("tree_sitter_language_pack");

    const download_module = b.createModule(.{
        .root_source_file = b.path("src/download_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    download_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const download_tests = b.addTest(.{
        .name = "download_test",
        .root_module = download_module,
        .use_llvm = true,
    });
    const download_run = b.addRunArtifact(download_tests);
    test_step.dependOn(&download_run.step);

    const error_handling_module = b.createModule(.{
        .root_source_file = b.path("src/error_handling_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    error_handling_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const error_handling_tests = b.addTest(.{
        .name = "error_handling_test",
        .root_module = error_handling_module,
        .use_llvm = true,
    });
    const error_handling_run = b.addRunArtifact(error_handling_tests);
    test_step.dependOn(&error_handling_run.step);

    const language_detection_module = b.createModule(.{
        .root_source_file = b.path("src/language_detection_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    language_detection_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const language_detection_tests = b.addTest(.{
        .name = "language_detection_test",
        .root_module = language_detection_module,
        .use_llvm = true,
    });
    const language_detection_run = b.addRunArtifact(language_detection_tests);
    test_step.dependOn(&language_detection_run.step);

    const parsing_module = b.createModule(.{
        .root_source_file = b.path("src/parsing_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    parsing_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const parsing_tests = b.addTest(.{
        .name = "parsing_test",
        .root_module = parsing_module,
        .use_llvm = true,
    });
    const parsing_run = b.addRunArtifact(parsing_tests);
    test_step.dependOn(&parsing_run.step);

    const process_module = b.createModule(.{
        .root_source_file = b.path("src/process_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    process_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const process_tests = b.addTest(.{
        .name = "process_test",
        .root_module = process_module,
        .use_llvm = true,
    });
    const process_run = b.addRunArtifact(process_tests);
    test_step.dependOn(&process_run.step);

    const query_module = b.createModule(.{
        .root_source_file = b.path("src/query_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    query_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const query_tests = b.addTest(.{
        .name = "query_test",
        .root_module = query_module,
        .use_llvm = true,
    });
    const query_run = b.addRunArtifact(query_tests);
    test_step.dependOn(&query_run.step);

    const registry_module = b.createModule(.{
        .root_source_file = b.path("src/registry_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    registry_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const registry_tests = b.addTest(.{
        .name = "registry_test",
        .root_module = registry_module,
        .use_llvm = true,
    });
    const registry_run = b.addRunArtifact(registry_tests);
    test_step.dependOn(&registry_run.step);

    const smoke_module = b.createModule(.{
        .root_source_file = b.path("src/smoke_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    smoke_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const smoke_tests = b.addTest(.{
        .name = "smoke_test",
        .root_module = smoke_module,
        .use_llvm = true,
    });
    const smoke_run = b.addRunArtifact(smoke_tests);
    test_step.dependOn(&smoke_run.step);

}
