# TypeScript/Node.js API Quick Reference

## Installation

```bash
npm install @kreuzberg/tree-sitter-language-pack
pnpm add @kreuzberg/tree-sitter-language-pack
yarn add @kreuzberg/tree-sitter-language-pack
```

All functions are synchronous. Pre-built binaries for macOS (arm64), Linux (x64, arm64), Windows (x64). Requires Node.js >= 16.

## Language Discovery

```typescript
availableLanguages(): string[]
hasLanguage(name: string): boolean
languageCount(): number
detectLanguage(path: string): string | null
detectLanguageFromContent(content: string): string | null
detectLanguageFromExtension(ext: string): string | null
detectLanguageFromPath(path: string): string | null
extensionAmbiguity(ext: string): AmbiguityResult | null
// AmbiguityResult: {assigned: string, alternatives: string[]}
```

## Parsing

```typescript
parseString(language: string, source: string): ExternalObject<Tree>
// Returns opaque tree handle

treeRootNodeType(tree: Tree): string
treeRootChildCount(tree: Tree): number
treeContainsNodeType(tree: Tree, nodeType: string): boolean
treeHasErrorNodes(tree: Tree): boolean
```

Example:

```typescript
const tree = parseString("python", "x = 1");
console.log(treeRootNodeType(tree)); // "module"
console.log(treeHasErrorNodes(tree)); // false
```

## Code Intelligence Processing

```typescript
process(source: string, config: JsProcessConfig): ProcessResult

interface JsProcessConfig {
  language: string;
  structure?: boolean;      // default: true
  imports?: boolean;        // default: true
  exports?: boolean;        // default: true
  comments?: boolean;       // default: false
  docstrings?: boolean;     // default: false
  symbols?: boolean;        // default: false
  diagnostics?: boolean;    // default: false
  chunkMaxSize?: number;    // optional, in bytes
  extractions?: Record<string, PatternConfig>;
}

interface ProcessResult {
  language: string;
  metrics: FileMetrics;
  structure: StructureItem[];
  imports: ImportInfo[];
  exports: ExportInfo[];
  comments: CommentInfo[];
  docstrings: DocstringInfo[];
  symbols: SymbolInfo[];
  diagnostics: Diagnostic[];
  chunks: CodeChunk[];
}
```

Example:

```typescript
const result = process("def hello(): pass", { language: "python" });
console.log(result.structure); // Functions, classes
console.log(result.imports); // Import statements
console.log(result.chunks); // Code chunks for LLMs
```

Supporting types:

```typescript
interface FileMetrics {
  totalLines: number;
  totalBytes: number;
  blankLines: number;
  commentLines: number;
  codeLines: number;
  errorCount: number;
}

interface Span {
  startByte: number;
  endByte: number;
  startRow: number;
  startCol: number;
  endRow: number;
  endCol: number;
}

interface StructureItem {
  kind: string; // "function" | "class" | "method" | ...
  name: string;
  span: Span;
  parent: string | null;
}

interface ImportInfo {
  module: string;
  names: string[];
  span: Span;
}

interface ExportInfo {
  name: string;
  kind: string;
  span: Span;
}

interface CommentInfo {
  text: string;
  kind: string;
  span: Span;
  associatedNode: string | null;
}

interface DocstringInfo {
  text: string;
  format: string;
  span: Span;
  associatedItem: string | null;
  sections: Array<Record<string, string>>;
}

interface SymbolInfo {
  name: string;
  kind: string;
  span: Span;
  typeAnnotation: string | null;
}

interface Diagnostic {
  message: string;
  severity: string;
  span: Span;
}

interface CodeChunk {
  content: string;
  startByte: number;
  endByte: number;
  metadata: ChunkContext;
}

interface ChunkContext {
  language: string;
  chunkIndex: number;
  totalChunks: number;
  startLine: number;
  endLine: number;
  nodeTypes: string[];
  symbolsDefined: string[];
  comments: string[];
  docstrings: string[];
  hasErrorNodes: boolean;
  contextPath: string[];
}
```

## Extraction Queries

```typescript
extract(source: string, config: object): ExtractionResult
validateExtraction(config: object): ValidationResult

interface ExtractionConfig {
  language: string;
  patterns: Record<string, PatternConfig>;
}

interface PatternConfig {
  query: string;
  captureOutput?: "Text" | "Node" | "Full";  // default: "Full"
  childFields?: string[];
  maxResults?: number;
  byteRange?: [number, number];
}

interface ExtractionResult {
  language: string;
  results: Record<string, PatternResult>;
}

interface PatternResult {
  matches: MatchResult[];
  totalCount: number;
}

interface MatchResult {
  patternIndex: number;
  captures: CaptureResult[];
}

interface CaptureResult {
  name: string;
  node: NodeInfo | null;
  text: string | null;
  childFields: Record<string, string | null>;
  startByte: number;
}

interface ValidationResult {
  valid: boolean;
  patterns: Record<string, PatternValidation>;
}

interface PatternValidation {
  valid: boolean;
  captureNames: string[];
  patternCount: number;
  warnings: string[];
  errors: string[];
}
```

Example:

```typescript
const result = extract("def hello(): pass", {
  language: "python",
  patterns: {
    functions: {
      query: "(function_definition name: (identifier) @fn_name)",
      captureOutput: "Text",
    },
  },
});

for (const match of result.results.functions.matches) {
  for (const capture of match.captures) {
    console.log(capture.text); // "hello"
  }
}
```

## Bundled Queries

```typescript
getHighlightsQuery(language: string): string | null
getInjectionsQuery(language: string): string | null
getLocalsQuery(language: string): string | null
```

## Download & Configuration

```typescript
init(config?: JsPackConfig): void
configure(config: JsPackConfig): void
download(names: string[]): number
downloadAll(): number
manifestLanguages(): string[]
downloadedLanguages(): string[]
cleanCache(): void
cacheDir(): string

interface JsPackConfig {
  cacheDir?: string;
  languages?: string[];
  groups?: string[];
}
```

Example:

```typescript
import { init, download, cacheDir } from "@kreuzberg/tree-sitter-language-pack";

// Pre-download languages
init({ languages: ["python", "javascript", "rust"] });

// Download on-demand
download(["python", "typescript"]);

// Check cache
console.log(cacheDir());
```

## Low-Level Interop

```typescript
getLanguagePtr(name: string): number
// Returns raw TSLanguage pointer for use with node-tree-sitter
```

## Error Handling

All errors throw standard JavaScript Error objects with descriptive messages. No custom exception types.

```typescript
try {
  parseString("nonexistent_language", "code");
} catch (error) {
  console.error(error.message);
}
```

## Common Patterns

### Detect and Analyze

```typescript
import { detectLanguage, process } from "@kreuzberg/tree-sitter-language-pack";
import { readFileSync } from "fs";

const lang = detectLanguage("src/main.py");
if (lang) {
  const source = readFileSync("src/main.py", "utf-8");
  const result = process(source, { language: lang });
  console.log(`Functions: ${result.structure.length}`);
}
```

### Batch Processing

```typescript
import { detectLanguage, process } from "@kreuzberg/tree-sitter-language-pack";
import { readdirSync, readFileSync } from "fs";

const files = ["app.py", "lib.rs", "index.ts"];
for (const file of files) {
  const lang = detectLanguage(file);
  if (!lang) continue;

  const source = readFileSync(file, "utf-8");
  const result = process(source, { language: lang });
  console.log(`${file}: ${result.structure.length} items`);
}
```
