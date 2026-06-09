```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
import 'package:tree_sitter_language_pack/src/tree_sitter_language_pack_bridge_generated/frb_generated.dart'
    show RustLib;

void main() async {
  await RustLib.init();

  const config = ProcessConfig(
    language: 'python',
    structure: true,
    imports: true,
    exports: false,
    comments: false,
    docstrings: false,
    symbols: false,
    diagnostics: false,
  );

  const source = '''
import os
from pathlib import Path

def read_file(path: str) -> str:
    return Path(path).read_text()

class FileReader:
    def __init__(self, base_dir: str):
        self.base_dir = base_dir
''';

  final result = await TreeSitterLanguagePackBridge.process(source, config);

  for (final item in result.structure) {
    print('${item.kind}: ${item.name ?? "<anonymous>"}');
  }
}
```
