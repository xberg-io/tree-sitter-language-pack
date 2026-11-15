import { getParser, WasmInputEdit, WasmPoint } from "@xberg-io/tree-sitter-language-pack-wasm";
import { expect, it } from "vitest";

it("can reuse the same edited tree for more than one parse", () => {
  const parser = getParser("python");
  const original = parser.parse("x = 'é'\n");
  if (!original) throw new Error("Initial parse failed");
  const edit = new WasmInputEdit(5, 7, 9, new WasmPoint(0, 5), new WasmPoint(0, 7), new WasmPoint(0, 9));
  const edited = original.edited(edit);
  const source = "x = '🙂'\n";
  const fresh = parser.parse(source);
  if (!fresh) throw new Error("Fresh parse failed");
  const expected = fresh.rootNode().toSexp();
  for (let run = 0; run < 2; run++) {
    const incremental = parser.parseIncremental(source, edited);
    expect(incremental?.rootNode().toSexp()).toBe(expected);
  }
});
