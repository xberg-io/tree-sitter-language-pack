```javascript title="WebAssembly"
import {
  availableLanguages,
  getParser,
  hasLanguage,
  languageCount,
} from "@xberg-io/tree-sitter-language-pack-wasm";

console.log(`${languageCount()} languages available`);
console.log(`Python available: ${hasLanguage("python")}`);
console.log(`First 5: ${availableLanguages().slice(0, 5).join(", ")}`);

const parser = getParser("python");
try {
  const tree = parser.parse("def hello(): pass");
  try {
    console.log(`Root: ${tree.rootNode().kind()}`);
  } finally {
    tree.free();
  }
} finally {
  parser.free();
}
```
