import XCTest
import Foundation

import TreeSitterLanguagePack
import SwiftTreeSitter

// Requires the native `tree-sitter-language-pack-swift` + `ts-pack-core-ffi` Rust crates to
// be built and wired via `scripts/setup-swift-bridge.sh` (see `task swift:build` /
// `task swift:test`), compiled with TSLP_LINK_MODE=static and TSLP_LANGUAGES=mojo,nim,norg
// (see .task/swift.yml). Every test below that needs a real grammar (parsing, root-node
// kind, `process()`) uses "nim", one of those three statically-compiled languages, so this
// suite needs no network access and no warm download cache. "python"/"rust"/"markdown"
// appear only as literal data values in the pure language-detection tests below, which
// consult a static extension/shebang lookup table and never touch a parser.

/// Pure functions that need no grammar at all: extension/path/shebang lookup tables.
final class LanguageDetectionTests: XCTestCase {
    func testDetectLanguageFromExtensionMapsPyToPython() {
        XCTAssertEqual(
            TreeSitterLanguagePack.detectLanguageFromExtension(ext: "py"),
            "python",
            "\"py\" is a well-known extension and must resolve to \"python\""
        )
    }

    func testDetectLanguageFromExtensionIsCaseInsensitive() {
        XCTAssertEqual(
            TreeSitterLanguagePack.detectLanguageFromExtension(ext: "RS"),
            "rust",
            "extension matching must be case-insensitive per documented behavior"
        )
    }

    func testDetectLanguageFromExtensionReturnsNilForUnknownExtension() {
        XCTAssertNil(TreeSitterLanguagePack.detectLanguageFromExtension(ext: "this-extension-does-not-exist"))
    }

    func testDetectLanguageFromPathMatchesRustFile() {
        XCTAssertEqual(TreeSitterLanguagePack.detectLanguageFromPath(path: "src/main.rs"), "rust")
    }

    func testDetectLanguageFromPathReturnsNilWithoutExtension() {
        XCTAssertNil(
            TreeSitterLanguagePack.detectLanguageFromPath(path: "Makefile"),
            "a path with no extension has nothing to detect from"
        )
    }

    func testDetectLanguageFromContentMatchesPythonShebang() {
        XCTAssertEqual(
            TreeSitterLanguagePack.detectLanguageFromContent(content: "#!/usr/bin/env python3\npass"),
            "python"
        )
    }

    func testDetectLanguageFromContentReturnsNilWithoutShebang() {
        XCTAssertNil(TreeSitterLanguagePack.detectLanguageFromContent(content: "no shebang here"))
    }

    func testDetectLanguageAliasResolvesPathExtension() {
        XCTAssertEqual(
            TreeSitterLanguagePack.detectLanguage(path: "README.md"),
            "markdown",
            "detectLanguage is documented as a path/extension detection alias"
        )
    }
}

/// Bundled query lookup: also pure, no grammar required.
final class BundledQueryTests: XCTestCase {
    func testGetHighlightsQueryReturnsNilForUnknownLanguage() {
        XCTAssertNil(TreeSitterLanguagePack.getHighlightsQuery(language: "this-language-does-not-exist"))
    }
}

/// Registry checks against "nim", which `task swift:test` compiles in statically
/// (TSLP_LANGUAGES=mojo,nim,norg), so these never touch the network.
final class RegistryTests: XCTestCase {
    func testHasLanguageIsTrueForStaticallyCompiledLanguage() {
        XCTAssertTrue(
            TreeSitterLanguagePack.hasLanguage(name: "nim"),
            "nim is compiled in by the swift build task (TSLP_LANGUAGES=mojo,nim,norg)"
        )
    }

    func testHasLanguageIsFalseForUnknownLanguage() {
        XCTAssertFalse(TreeSitterLanguagePack.hasLanguage(name: "totally-bogus-language-name"))
    }

    func testAvailableLanguagesContainsStaticallyCompiledLanguage() {
        XCTAssertTrue(TreeSitterLanguagePack.availableLanguages().contains("nim"))
    }

    func testLanguageCountMatchesAvailableLanguagesCount() {
        let count = TreeSitterLanguagePack.languageCount()
        let names = TreeSitterLanguagePack.availableLanguages()
        XCTAssertEqual(
            count,
            UInt(names.count),
            "languageCount() must always equal availableLanguages().count; a mismatch means one of the two "
                + "accessors is stale relative to the other"
        )
    }
}

/// Real parsing through the upstream SwiftTreeSitter.Parser, mirroring
/// CapsulePassthroughTests.swift's hand-written pattern: `getLanguage()` hands back a real
/// `SwiftTreeSitter.Language` capsule usable directly with `SwiftTreeSitter.Parser`.
///
/// Uses "nim" (statically compiled, see the file-header comment) rather than "python": the
/// upstream e2e CapsulePassthroughTests.swift precedent parses python, but that grammar is
/// not one of TSLP_LANGUAGES=mojo,nim,norg baked into this package's build, so it would make
/// this suite network-dependent.
final class ParsingTests: XCTestCase {
    // parsers/nim/src/grammar.json names its start rule "module", and node-types.json
    // confirms "module" is a named node type — verified directly against the grammar
    // sources vendored in this repo, not assumed from another language's doc example.
    func testGetLanguageProducesAParserUsableNimLanguage() throws {
        let nimLanguage = try TreeSitterLanguagePack.getLanguage(name: "nim")

        var parser = Parser()
        try parser.setLanguage(nimLanguage)

        let tree = try XCTUnwrap(parser.parse("echo \"hello\""), "parsing valid nim source must produce a tree")
        let root = try XCTUnwrap(tree.rootNode)

        XCTAssertEqual(root.nodeType, "module", "nim's tree-sitter grammar names its root node \"module\"")
    }

    // fixtures/smoke/nim.json asserts `not_error` for this exact source, so it is
    // known-valid nim, not a guess.
    func testProcessEchoesBackTheRequestedLanguageForValidNim() throws {
        let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"nim\"}")
        let result = try TreeSitterLanguagePack.process(source: "echo \"hello\"", config: configObj)

        XCTAssertEqual(result.language().toString(), "nim")
    }

    // No structure-extraction test: crates/ts-pack-core/src/intel/intelligence.rs
    // `structure_kind_at()` matches an exact, hardcoded set of tree-sitter node kind names
    // (`function_definition`, `function_item`, `struct_item`, ...), and nim's grammar
    // (parsers/nim/src/node-types.json) uses none of them — its declarations are named
    // `declaration` / `declColonEquals`. Structure extraction is therefore unimplemented for
    // nim and would always report an empty list; asserting that would be vacuous, so the
    // test is omitted rather than weakened to a truthiness check.
}

/// Error paths: unknown languages and invalid configuration must fail loudly, never
/// silently succeed with garbage output.
final class ErrorHandlingTests: XCTestCase {
    func testGetLanguageThrowsForUnknownLanguage() {
        XCTAssertThrowsError(try TreeSitterLanguagePack.getLanguage(name: "this-language-does-not-exist-anywhere"))
    }

    func testProcessThrowsForEmptyLanguageName() throws {
        let configObj = try TreeSitterLanguagePack.processConfigFromJson("{\"language\":\"\"}")
        XCTAssertThrowsError(try TreeSitterLanguagePack.process(source: "hello", config: configObj))
    }
}
