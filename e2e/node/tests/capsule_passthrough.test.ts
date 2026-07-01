// Hand-written capsule-passthrough test (not fixture-generated).
// Verifies that `getLanguage` returns a value the upstream `tree-sitter` npm
// package accepts directly — i.e. host-native Language passthrough (#143).
import Parser from "tree-sitter";
import { describe, expect, it } from "vitest";
import { getLanguage } from "@xberg-io/tree-sitter-language-pack";

describe("capsule Language passthrough", () => {
  it("returns a Language usable by upstream tree-sitter Parser.setLanguage", () => {
    const language = getLanguage("python");
    expect(language).toBeDefined();

    const parser = new Parser();
    // Throws if `language` is not a valid tree-sitter Language handle.
    parser.setLanguage(language);

    const tree = parser.parse("def greet(name):\n    return name\n");
    expect(tree.rootNode.type).toBe("module");
    expect(tree.rootNode.namedChildCount).toBeGreaterThan(0);
    expect(tree.rootNode.firstNamedChild?.type).toBe("function_definition");
  });

  it("returns distinct languages for distinct names", () => {
    const python = getLanguage("python");
    const javascript = getLanguage("javascript");

    const parser = new Parser();
    parser.setLanguage(javascript);
    const tree = parser.parse("const x = 1;\n");
    expect(tree.rootNode.type).toBe("program");
    expect(python).not.toBe(javascript);
  });
});
