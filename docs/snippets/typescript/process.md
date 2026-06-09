```typescript title="Node.js"
import { process } from "@kreuzberg/tree-sitter-language-pack";

const result = process(
  `
import { readFile } from 'fs/promises';

export async function loadConfig(path: string): Promise<Config> {
  const data = await readFile(path, 'utf-8');
  return JSON.parse(data);
}

export class ConfigManager {
  constructor(private basePath: string) {}
}
`,
  { language: "typescript", structure: true, imports: true, exports: true, comments: true },
);

if (result.structure) {
  for (const item of result.structure) {
    console.log(`${item.kind}: ${item.name}`);
  }
}
```
