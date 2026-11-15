import 'package:test/test.dart';
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';

void main() {
  setUpAll(RustLib.init);
  tearDownAll(RustLib.dispose);

  test('reuses an edited tree across parses', () async {
    final parser = await TreeSitterLanguagePackBridge.getParser('python');
    final original = (await parser.parse(source: "x = 'é'\n"))!;
    final edited = await original.edited(
      edit: const InputEdit(
        startByte: 5,
        oldEndByte: 7,
        newEndByte: 9,
        startPosition: Point(row: 0, column: 5),
        oldEndPosition: Point(row: 0, column: 7),
        newEndPosition: Point(row: 0, column: 9),
      ),
    );
    const source = "x = '🙂'\n";
    final fresh = (await parser.parse(source: source))!;
    final expected = await (await fresh.rootNode()).toSexp();
    for (var run = 0; run < 2; run++) {
      final tree = await parser.parseIncremental(
        source: source,
        oldTree: edited,
      );
      expect(await (await tree!.rootNode()).toSexp(), expected);
    }
  });
}
