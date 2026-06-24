// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v16),
    ],
    dependencies: [
        .package(url: "https://github.com/xberg-io/tree-sitter-language-pack", branch: "release/swift/1.10.9"),
    ],
    targets: [
        .testTarget(
            name: "TreeSitterLanguagePackE2ETests",
            dependencies: [.product(name: "TreeSitterLanguagePack", package: "tree-sitter-language-pack")]
        ),
    ]
)
