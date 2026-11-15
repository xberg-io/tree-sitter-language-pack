import { getParser } from "@xberg-io/tree-sitter-language-pack";
import { expect, it } from "vitest";

it("reuses an edited tree with UTF-8 coordinates and preserves the original", () => {
  const before = "x = 'é'\ny = 1\n";
  const after = "x = '🙂'\ny = 1\n";
  const parser = getParser("python");
  const original = parser.parse(before);
  if (!original) throw new Error("Initial parse failed");
  const originalRoot = original.rootNode();

  const edited = original.edited({
    startByte: 5,
    oldEndByte: 7,
    newEndByte: 9,
    startPosition: { row: 0, column: 5 },
    oldEndPosition: { row: 0, column: 7 },
    newEndPosition: { row: 0, column: 9 },
  });

  const incremental = parser.parseIncremental(after, edited);
  const fresh = parser.parse(after);
  if (!incremental || !fresh) throw new Error("Reparse failed");
  expect(incremental.rootNode().toSexp()).toBe(fresh.rootNode().toSexp());
  expect(incremental.rootNode().endByte()).toBe(Buffer.byteLength(after));
  expect(originalRoot.endByte()).toBe(Buffer.byteLength(before));
});
