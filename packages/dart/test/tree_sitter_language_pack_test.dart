import 'package:test/test.dart';
import 'package:tree_sitter_language_pack/tree_sitter_language_pack.dart';
import 'package:tree_sitter_language_pack/src/tree_sitter_language_pack_bridge_generated/frb_generated.dart'
    show RustLib;

// Requires the native `tree-sitter-language-pack-dart` Rust crate to be built
// via `task dart:build`, which compiles with TSLP_LINK_MODE=static and
// TSLP_LANGUAGES=mojo,nim,norg (see .task/dart.yml). Every test that needs a
// real grammar (parsing, root-node kind, `process()`) uses "nim", one of
// those three statically-compiled languages, so this suite needs no network
// access and no warm download cache. "python"/"rust"/"markdown" appear only
// as literal data values in the pure language-detection tests below, which
// consult a static extension/shebang lookup table and never touch a parser.

void main() {
  var rustLibInitialized = false;

  setUpAll(() async {
    await RustLib.init();
    rustLibInitialized = true;
  });

  tearDownAll(() async {
    if (rustLibInitialized) {
      RustLib.dispose();
    }
  });

  group('language detection (pure, no grammar required)', () {
    test('should_map_py_extension_to_python', () async {
      final result =
          await TreeSitterLanguagePackBridge.detectLanguageFromExtension('py');
      expect(
        result,
        equals('python'),
        reason: '"py" is a well-known extension and must resolve to "python"',
      );
    });

    test('should_match_extensions_case_insensitively', () async {
      final result =
          await TreeSitterLanguagePackBridge.detectLanguageFromExtension('RS');
      expect(
        result,
        equals('rust'),
        reason:
            'extension matching must be case-insensitive per documented behavior',
      );
    });

    test('should_return_null_for_unrecognized_extension', () async {
      final result =
          await TreeSitterLanguagePackBridge.detectLanguageFromExtension(
            'this-extension-does-not-exist-anywhere',
          );
      expect(
        result,
        isNull,
        reason: 'unrecognized extensions must not resolve to any language',
      );
    });

    test('should_detect_rust_from_file_path', () async {
      final result = await TreeSitterLanguagePackBridge.detectLanguageFromPath(
        'src/main.rs',
      );
      expect(result, equals('rust'));
    });

    test('should_return_null_for_path_without_extension', () async {
      final result = await TreeSitterLanguagePackBridge.detectLanguageFromPath(
        'Makefile',
      );
      expect(
        result,
        isNull,
        reason: 'a path with no extension has nothing to detect from',
      );
    });

    test('should_detect_python_from_env_shebang', () async {
      final result =
          await TreeSitterLanguagePackBridge.detectLanguageFromContent(
            '#!/usr/bin/env python3\npass',
          );
      expect(result, equals('python'));
    });

    test('should_return_null_when_content_has_no_shebang', () async {
      final result =
          await TreeSitterLanguagePackBridge.detectLanguageFromContent(
            'no shebang here',
          );
      expect(result, isNull);
    });

    test('should_alias_detect_language_to_path_extension_lookup', () async {
      final result = await TreeSitterLanguagePackBridge.detectLanguage(
        'README.md',
      );
      expect(
        result,
        equals('markdown'),
        reason:
            'detectLanguage is documented as a path/extension detection alias',
      );
    });
  });

  group('bundled queries (pure, no grammar required)', () {
    test('should_return_null_highlights_query_for_unknown_language', () async {
      final result = await TreeSitterLanguagePackBridge.getHighlightsQuery(
        'this-language-does-not-exist',
      );
      expect(result, isNull);
    });
  });

  group('registry (statically compiled "nim", no network required)', () {
    test('should_report_statically_compiled_language_as_available', () async {
      final result = await TreeSitterLanguagePackBridge.hasLanguage('nim');
      expect(
        result,
        isTrue,
        reason:
            'nim is compiled in by task dart:build (TSLP_LANGUAGES=mojo,nim,norg)',
      );
    });

    test('should_report_unknown_language_as_unavailable', () async {
      final result = await TreeSitterLanguagePackBridge.hasLanguage(
        'totally-bogus-language-name',
      );
      expect(result, isFalse);
    });

    test(
      'should_list_statically_compiled_language_in_available_languages',
      () async {
        final result = await TreeSitterLanguagePackBridge.availableLanguages();
        expect(result, contains('nim'));
      },
    );

    test(
      'should_keep_language_count_consistent_with_available_languages_list',
      () async {
        final count = await TreeSitterLanguagePackBridge.languageCount();
        final names = await TreeSitterLanguagePackBridge.availableLanguages();
        expect(
          count,
          equals(names.length),
          reason:
              'languageCount() must always equal availableLanguages().length; a mismatch means one of the two '
              'accessors is stale relative to the other',
        );
      },
    );
  });

  group('parsing (statically compiled "nim", no network required)', () {
    // parsers/nim/src/grammar.json names its start rule "module", and
    // node-types.json confirms "module" is a named node type — verified
    // directly against the grammar sources vendored in this repo, not
    // assumed from another language's doc example.
    test('should_parse_nim_source_to_a_module_root_node', () async {
      final parser = await TreeSitterLanguagePackBridge.getParser('nim');
      final tree = await parser.parse(source: 'echo "hello"');
      expect(
        tree,
        isNotNull,
        reason: 'parsing valid nim source must produce a tree',
      );
      final root = await tree!.rootNode();
      final kind = await root.kind();
      expect(
        kind,
        equals('module'),
        reason: 'nim\'s tree-sitter grammar names its root node "module"',
      );
    });

    // fixtures/smoke/nim.json asserts `not_error` for this exact source, so
    // it is known-valid nim, not a guess.
    test('should_report_zero_errors_for_syntactically_valid_nim', () async {
      final config = await createProcessConfigFromJson(
        json: '{"language":"nim"}',
      );
      final result = await TreeSitterLanguagePackBridge.process(
        'echo "hello"',
        config: config,
      );
      expect(result.language, equals('nim'));
      expect(result.metrics.errorCount, equals(0));
    });

    // No structure-extraction test: crates/ts-pack-core/src/intel/intelligence.rs
    // `structure_kind_at()` matches an exact, hardcoded set of tree-sitter node
    // kind names (`function_definition`, `function_item`, `struct_item`, ...),
    // and nim's grammar (parsers/nim/src/node-types.json) uses none of them —
    // its declarations are named `declaration` / `declColonEquals`. Structure
    // extraction is therefore unimplemented for nim and would always report an
    // empty list; asserting that would be vacuous, so the test is omitted
    // rather than weakened to a truthiness check.
  });

  group('error paths', () {
    test('should_throw_when_getting_an_unknown_language', () async {
      await expectLater(
        TreeSitterLanguagePackBridge.getLanguage(
          'this-language-does-not-exist-anywhere',
        ),
        throwsA(anything),
      );
    });

    test(
      'should_throw_when_getting_a_parser_for_an_unknown_language',
      () async {
        await expectLater(
          TreeSitterLanguagePackBridge.getParser(
            'this-language-does-not-exist-anywhere',
          ),
          throwsA(anything),
        );
      },
    );

    test('should_throw_when_processing_with_an_empty_language_name', () async {
      await expectLater(() async {
        final config = await createProcessConfigFromJson(
          json: '{"language":""}',
        );
        return TreeSitterLanguagePackBridge.process('hello', config: config);
      }(), throwsA(anything));
    });
  });
}
