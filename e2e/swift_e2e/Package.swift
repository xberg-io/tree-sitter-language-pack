// swift-tools-version: 6.0
import PackageDescription

let package = Package(
  name: "E2eSwift",
  platforms: [
    .macOS(.v13),
    .iOS(.v16),
  ],
  dependencies: [
    .package(path: "../../packages/swift"),
    .package(url: "https://github.com/tree-sitter/swift-tree-sitter", from: "0.25.0"),
  ],
  targets: [
    .testTarget(
      name: "TreeSitterLanguagePackE2ETests",
      dependencies: [.product(name: "TreeSitterLanguagePack", package: "swift"), .product(name: "SwiftTreeSitter", package: "swift-tree-sitter")]
    ),
  ]
)
