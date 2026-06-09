```dart title="Dart"
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
import 'package:tree_sitter_language_pack/src/tree_sitter_language_pack_bridge_generated/frb_generated.dart'
    show RustLib;

void main() async {
  await RustLib.init();

  // Pre-download specific languages.
  final count = await TreeSitterLanguagePackBridge.download(
    ['python', 'javascript', 'rust'],
  );
  print('Downloaded $count languages');

  // Or initialize with config.
  await TreeSitterLanguagePackBridge.init(
    const PackConfig(languages: ['python', 'go'], cacheDir: '/tmp/parsers'),
  );

  // Inspect cache state.
  print(await TreeSitterLanguagePackBridge.downloadedLanguages());
  final manifest = await TreeSitterLanguagePackBridge.manifestLanguages();
  print(manifest.take(5).toList());
}
```
