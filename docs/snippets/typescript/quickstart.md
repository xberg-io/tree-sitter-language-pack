```typescript title="Node.js"
import { process } from "@xberg-io/tree-sitter-language-pack";

const result = process("function hello() { console.log('world'); }", {
  language: "javascript",
  structure: true,
  imports: true,
});

console.log(`Language: ${result.language}`);
console.log(`Functions: ${result.structure?.length ?? 0}`);
```
