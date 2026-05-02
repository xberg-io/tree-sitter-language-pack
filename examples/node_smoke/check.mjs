import {
	availableLanguages,
	hasLanguage,
} from "@kreuzberg/tree-sitter-language-pack";

const langs = availableLanguages();
console.log(`Available languages: ${langs.length}`);

if (langs.length === 0) throw new Error("no languages available");
if (!hasLanguage("javascript")) throw new Error("javascript not found");

console.log("Node smoke test passed");
