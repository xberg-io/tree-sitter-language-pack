```javascript title="WebAssembly"
import { process } from "@xberg-io/tree-sitter-language-pack-wasm";

const result = process("function add(a, b) { return a + b; }", {
  language: "javascript",
  structure: true,
  imports: true,
  exports: true,
  comments: false,
  docstrings: false,
  symbols: false,
  diagnostics: false,
});

console.log(`Language: ${result.language}`);
for (const item of result.structure) {
  console.log(`${item.kind}: ${item.name ?? "(anonymous)"}`);
}
```
