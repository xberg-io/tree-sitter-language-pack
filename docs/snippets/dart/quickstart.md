```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
import 'package:tree_sitter_language_pack/src/tree_sitter_language_pack_bridge_generated/frb_generated.dart'
    show RustLib;

void main() async {
  await RustLib.init();

  final parser = await TreeSitterLanguagePackBridge.getParser('python');
  final tree = await parser.parse(source: "def hello():\n    print('world')\n");
  final root = await tree!.rootNode();

  print('Root kind: ${await root.kind()}');
}
```
