// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "E2eSwift",
    platforms: [
        .macOS(.v13),
        .iOS(.v14),
    ],
    dependencies: [
        .package(url: "https://github.com/kreuzberg-dev/tree-sitter-language-pack.git", from: "1.9.0-rc.23"),
    ],
    targets: [
        .testTarget(
            name: "TreeSitterLanguagePackE2ETests",
            dependencies: [.product(name: "TreeSitterLanguagePack", package: "tree-sitter-language-pack")]
        ),
    ]
)
