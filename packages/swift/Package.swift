// swift-tools-version: 6.0
import PackageDescription

// NOTE: Run `cargo build -p tree-sitter-language-pack-swift` before `swift build`.
// The build step generates Swift + C bridge sources; copy them into Sources/RustBridge
// and Sources/RustBridgeC before building. See README.md for the full workflow.
let package = Package(
  name: "TreeSitterLanguagePack",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
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
    // linkerSettings wire the Rust staticlib (libtree_sitter_language_pack_swift.a) produced by
    // `cargo build -p tree-sitter-language-pack-swift` so `swift build` / `swift test` can resolve
    // the `__swift_bridge__$*` C symbols. Both target/release and target/debug are
    // searched so either cargo profile works.
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
        .linkedFramework("Security", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("CoreFoundation", .when(platforms: [.macOS, .iOS])),
        .linkedFramework("SystemConfiguration", .when(platforms: [.macOS])),
      ]
    ),
    .target(
      name: "TreeSitterLanguagePack", dependencies: ["RustBridge"],
      path: "Sources/TreeSitterLanguagePack"),
    .testTarget(
      name: "TreeSitterLanguagePackTests", dependencies: ["TreeSitterLanguagePack"],
      path: "Tests/TreeSitterLanguagePackTests"),
  ]
)
