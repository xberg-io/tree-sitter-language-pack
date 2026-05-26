import { describe, expect, it } from "vitest";
import Parser from "tree-sitter";
import { getLanguage } from "@kreuzberg/tree-sitter-language-pack";

describe("Language capsule passthrough", () => {
	it("returns a tree-sitter Language usable by Parser.setLanguage", () => {
		const language = getLanguage("python");
		const parser = new Parser();
		parser.setLanguage(language);
		const tree = parser.parse("def foo(): pass");
		expect(tree.rootNode.type).toBe("module");
	});

	it("parses python source into the expected node kinds", () => {
		const language = getLanguage("python");
		const parser = new Parser();
		parser.setLanguage(language);
		const tree = parser.parse("x = 1\ny = 2");
		const root = tree.rootNode;
		expect(root.type).toBe("module");
		expect(root.childCount).toBe(2);
		expect(root.child(0)?.type).toBe("assignment");
	});
});
