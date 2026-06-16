const std = @import("std");
const builtin = @import("builtin");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});
    const test_step = b.step("test", "Run tests");

    // Fetch the published Zig package from the registry (multi-target lazy dependency).
    // Select the appropriate platform variant based on the target triple.
    const target_os = target.result.os.tag;
    const target_arch = target.result.cpu.arch;

    const tree_sitter_language_pack_dep_name = if (target_os == .linux and target_arch == .x86_64)
        "tree_sitter_language_pack_x86_64_unknown_linux_gnu"
    else if (target_os == .linux and target_arch == .aarch64)
        "tree_sitter_language_pack_aarch64_unknown_linux_gnu"
    else if (target_os == .macos and target_arch == .aarch64)
        "tree_sitter_language_pack_aarch64_apple_darwin"
    else if (target_os == .macos and target_arch == .x86_64)
        "tree_sitter_language_pack_x86_64_apple_darwin"
    else if (target_os == .windows and target_arch == .x86_64)
        "tree_sitter_language_pack_x86_64_pc_windows_msvc"
    else
        @panic("unsupported target — supported: linux-{x86_64,aarch64}, macos-{arm64,x86_64}, windows-x86_64");

    const tree_sitter_language_pack_dep = b.lazyDependency(tree_sitter_language_pack_dep_name, .{
        .target = target,
        .optimize = optimize,
    }) orelse return;
    const tree_sitter_language_pack_module = tree_sitter_language_pack_dep.module("tree_sitter_language_pack");
    const tree_sitter_language_pack_lib_path = tree_sitter_language_pack_dep.path("lib");
    const tree_sitter_language_pack_include_path = tree_sitter_language_pack_dep.path("include");
    tree_sitter_language_pack_module.addLibraryPath(tree_sitter_language_pack_lib_path);
    tree_sitter_language_pack_module.addIncludePath(tree_sitter_language_pack_include_path);
    tree_sitter_language_pack_module.linkSystemLibrary("ts_pack_core_ffi", .{});

    const data_extraction_module = b.createModule(.{
        .root_source_file = b.path("src/data_extraction_test.zig"),
        .target = target,
        .optimize = optimize,
        .link_libc = true,
    });
    data_extraction_module.addImport("tree_sitter_language_pack", tree_sitter_language_pack_module);
    const data_extraction_tests = b.addTest(.{
        .name = "data_extraction_test",
        .root_module = data_extraction_module,
        .use_llvm = true,
    });
    const data_extraction_run = b.addRunArtifact(data_extraction_tests);
    test_step.dependOn(&data_extraction_run.step);

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
    download_run.step.dependOn(&data_extraction_run.step);
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
    error_handling_run.step.dependOn(&download_run.step);
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
    language_detection_run.step.dependOn(&error_handling_run.step);
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
    parsing_run.step.dependOn(&language_detection_run.step);
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
    process_run.step.dependOn(&parsing_run.step);
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
    query_run.step.dependOn(&process_run.step);
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
    registry_run.step.dependOn(&query_run.step);
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
    smoke_run.step.dependOn(&registry_run.step);
    test_step.dependOn(&smoke_run.step);

}
