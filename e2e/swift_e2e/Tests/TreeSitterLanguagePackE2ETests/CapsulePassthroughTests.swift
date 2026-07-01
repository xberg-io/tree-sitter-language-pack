// Hand-written capsule-passthrough test (not fixture-generated); preserved across regen.

import XCTest
import Foundation
#if canImport(FoundationNetworking)
import FoundationNetworking
#endif
import TreeSitterLanguagePack
import SwiftTreeSitter

/// Hand-written e2e test for host-native Language passthrough.
/// Verifies that TreeSitterLanguagePack.getLanguage() returns a SwiftTreeSitter.Language
/// that is usable directly with the upstream SwiftTreeSitter parser.
final class CapsulePassthroughTests: XCTestCase {

  func testPassthroughPython() throws {
    // Parse Python code using capsule Language with upstream SwiftTreeSitter.Parser
    let pythonLang = try TreeSitterLanguagePack.getLanguage(name: "python")

    var parser = Parser()
    try parser.setLanguage(pythonLang)

    let source = "def greet(name):\n    return name\n"
    let tree = try parser.parse(source)

    let root = try XCTUnwrap(tree.rootNode)
    XCTAssertEqual(root.nodeType, "module")
  }

  func testPassthroughJavascript() throws {
    // Parse JavaScript code using capsule Language with upstream SwiftTreeSitter.Parser
    let jsLang = try TreeSitterLanguagePack.getLanguage(name: "javascript")

    var parser = Parser()
    try parser.setLanguage(jsLang)

    let source = "const x = 1;\n"
    let tree = try parser.parse(source)

    let root = try XCTUnwrap(tree.rootNode)
    XCTAssertEqual(root.nodeType, "program")
  }
}
