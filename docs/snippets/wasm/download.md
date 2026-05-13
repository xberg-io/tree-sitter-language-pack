```javascript title="WebAssembly"
// Note: the WASM build ships with statically compiled parsers — no download step needed.
import {
  availableLanguages,
  hasLanguage,
  languageCount,
} from "@kreuzberg/tree-sitter-language-pack-wasm";

console.log(`Has Python: ${hasLanguage("python")}`);
console.log(`Has Rust: ${hasLanguage("rust")}`);
console.log(`Total bundled languages: ${languageCount()}`);
console.log(`Sample: ${availableLanguages().slice(0, 10).join(", ")}`);
```
