// Setup file to make process function available globally for e2e tests
import { process } from "@kreuzberg/tree-sitter-language-pack-wasm";

// Make process globally available in test files
(globalThis as any).process = process;
