// swift-tools-version: 6.2
import PackageDescription

let package = Package(
  name: "E2eSwift",
  platforms: [
    .macOS(.v15),
    .iOS(.v14),
  ],
  dependencies: [
    .package(url: "https://github.com/xberg-io/tree-sitter-language-pack.git", from: "1.25.0"),
  ],
  targets: [
    .testTarget(
      name: "TreeSitterLanguagePackE2ETests",
      dependencies: [.product(name: "TreeSitterLanguagePack", package: "tree-sitter-language-pack")]
    ),
  ]
)
