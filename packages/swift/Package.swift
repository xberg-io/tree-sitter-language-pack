// swift-tools-version: 6.0
import PackageDescription

// NOTE: Run `cargo build -p tree-sitter-language-pack-swift` and then rerun `alef generate`
// before `swift build`. Alef materializes the swift-bridge Swift/C outputs into
// Sources/RustBridge and Sources/RustBridgeC when the Cargo build output exists.
// See README.md for the full workflow.
let package = Package(
  name: "TreeSitterLanguagePack",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  dependencies: [
    .package(url: "https://github.com/tree-sitter/swift-tree-sitter", from: "0.25.0"),
  ],
  products: [
    .library(name: "TreeSitterLanguagePack", targets: ["TreeSitterLanguagePack"])
  ],
  targets: [
    // RustBridgeC: pure C/headers target. Swift files in RustBridge import this
    // to access C types (RustStr, etc.) produced by swift-bridge.
    // publicHeadersPath: "." exposes RustBridgeC.h to dependents.
    .target(
      name: "RustBridgeC",
      path: "Sources/RustBridgeC",
      publicHeadersPath: "."
    ),
    // RustBridge: Swift wrapper around the Rust static library.
    // Depends on RustBridgeC so the generated Swift files can use the C types.
    // linkerSettings wire the Rust staticlibs (libtree_sitter_language_pack_swift.a and libts_pack_core_ffi.a)
    // produced by `cargo build -p tree-sitter-language-pack-swift` and the FFI crate so
    // `swift build` / `swift test` can resolve the `__swift_bridge__$*` and FFI C symbols.
    // Both target/release and target/debug are searched so either cargo profile works.
    // The FFI library is needed because the generated Swift service API code (App.swift)
    // calls FFI functions directly via @_silgen_name declarations.
    .target(
      name: "RustBridge",
      dependencies: ["RustBridgeC"],
      path: "Sources/RustBridge",
      linkerSettings: [
        .unsafeFlags([
          "-L../../target/release",
          "-L../../target/debug",
        ]),
        .linkedLibrary("tree_sitter_language_pack_swift"),
        .linkedLibrary("ts_pack_core_ffi"),
        .linkedFramework("Security", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("CoreFoundation", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("SystemConfiguration", .when(platforms: [.macOS])),
      ]
    ),
    .target(
      name: "TreeSitterLanguagePack", dependencies: ["RustBridge", "SwiftTreeSitter"],
      path: "Sources/TreeSitterLanguagePack",
      exclude: ["LICENSE"]),
    .testTarget(
      name: "TreeSitterLanguagePackTests", dependencies: ["TreeSitterLanguagePack"],
      path: "Tests/TreeSitterLanguagePackTests"),
  ]
)
