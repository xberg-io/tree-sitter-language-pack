---
title: Host-Native Language Passthrough
description: "How get_language() integrates with native tree-sitter packages across ecosystems."
---

Tree-sitter-language-pack's `get_language()` function returns the **native tree-sitter `Language` type** for your ecosystem—not a wrapper or opaque handle. This means you can pass the result directly to your parser without translation overhead or intermediate APIs.

---

## Passthrough vs. Opaque Bindings

### Passthrough Bindings

These 9 bindings return the real `Language` type from the host ecosystem's tree-sitter package:

| Binding     | Returns                                  | Host Package                          | Min Version |
| ----------- | ---------------------------------------- | ------------------------------------- | ----------- |
| **Python**  | `tree_sitter.Language` (PyCapsule)       | `tree-sitter`                         | ≥0.23       |
| **Node.js** | `tree-sitter` npm package `Language`     | `tree-sitter`                         | latest      |
| **Go**      | `*tree_sitter.Language`                  | `go-tree-sitter`                      | v0.24.0+    |
| **Java**    | `io.github.treesitter.jtreesitter.Language` | `io.github.tree-sitter:jtreesitter` | 0.26.0+     |
| **C#**      | `TreeSitter.Language`                    | `TreeSitter.DotNet`                   | 1.3.0+      |
| **Kotlin**  | `io.github.treesitter.ktreesitter.Language` | `io.github.tree-sitter:ktreesitter` | 0.25.0+     |
| **Swift**   | `SwiftTreeSitter.Language`                | `SwiftTreeSitter` (SwiftPM)           | 0.25.0+     |
| **Zig**     | `?*const tree_sitter.Language`            | `zig-tree-sitter`                     | v0.26.0+    |
| **C FFI**   | `const TSLanguage *`                      | `tree-sitter` (libtree-sitter)        | ABI 14+     |

The C FFI surface is the canonical passthrough: `ts_pack_get_language()` returns the bare
`const TSLanguage *` that every other C-ABI binding wraps in its host `Language` type. Pass it
straight to `ts_parser_set_language()`. The returned pointer is **borrowed** — it points at a
static, library-owned grammar, so do not `free` it (there is no `ts_pack_language_free` for it).

With a passthrough binding, you use the ecosystem's native parser without any wrapper:

=== "Python"

    ```python
    import tree_sitter
    import tree_sitter_language_pack

    # get_language returns tree_sitter.Language directly
    python_lang = tree_sitter_language_pack.get_language("python")

    # Use it with the native tree-sitter parser
    parser = tree_sitter.Parser(python_lang)

    tree = parser.parse(b"def foo(): pass")
    ```

=== "Node.js"

    ```typescript
    import TreeSitter from "tree-sitter";
    import * as tslp from "@kreuzberg/tree-sitter-language-pack";

    // getLanguage returns tree-sitter npm package Language
    const pythonLang = tslp.getLanguage("python");

    // Use it with the native tree-sitter parser
    const parser = new TreeSitter();
    parser.setLanguage(pythonLang);

    const tree = parser.parse("def foo(): pass");
    ```

=== "Go"

    ```go
    import (
        tree_sitter "github.com/tree-sitter/go-tree-sitter"
        tspack "github.com/xberg-io/tree-sitter-language-pack/packages/go"
    )

    // GetLanguage returns *tree_sitter.Language directly
    pythonLang, err := tspack.GetLanguage("python")
    if err != nil {
        // get_language can fail (unknown language)
        return err
    }

    // Use it with the native tree-sitter parser
    parser := tree_sitter.NewParser()
    parser.SetLanguage(pythonLang)

    tree := parser.Parse(code, nil)
    ```

=== "Java"

    ```java
    import io.github.treesitter.jtreesitter.Language;
    import io.github.treesitter.jtreesitter.Parser;
    import dev.kreuzberg.treesitterlanguagepack.TreeSitterLanguagePack;

    // getLanguage returns jtreesitter Language directly
    Language pythonLang = TreeSitterLanguagePack.getLanguage("python");

    // Use it with the native jtreesitter parser
    Parser parser = new Parser();
    parser.setLanguage(pythonLang);

    var tree = parser.parse(source);
    ```

=== "C"

    ```c
    #include <tree_sitter/api.h>
    #include "ts_pack.h"

    // ts_pack_get_language returns a borrowed const TSLanguage * (NULL on error)
    const TSLanguage *python_lang = ts_pack_get_language("python");
    if (python_lang == NULL) { /* unknown language */ }

    // Use it directly with the native tree-sitter C API — do NOT free it
    TSParser *parser = ts_parser_new();
    ts_parser_set_language(parser, python_lang);

    TSTree *tree = ts_parser_parse_string(parser, NULL, "def foo(): pass", 15);
    ```

### Opaque-Handle Bindings

These 5 bindings return an opaque handle specific to this package and do **not** expose a
host-native `Language`. Use the package's own `Parser` / `process()` API instead.

| Binding    | Why no passthrough                                                                 | Recommendation                      |
| ---------- | ---------------------------------------------------------------------------------- | ----------------------------------- |
| **Ruby**   | No maintained Ruby tree-sitter gem that constructs a `Language` from a raw pointer  | Use this package's `Parser` wrapper |
| **PHP**    | No maintained PHP tree-sitter extension exposing a `Language` from a raw pointer    | Use this package's extension API    |
| **Elixir** | No maintained Elixir tree-sitter library that ingests a raw `TSLanguage *`          | Use this package's NIF wrapper      |
| **WASM**   | The web-tree-sitter `Language` is loaded from `.wasm` bytes, not a native pointer   | Use this package's JS wrapper       |
| **Dart**   | Built with `flutter_rust_bridge`, which marshals Rust types as Arc-counted opaque proxies and never hands back the raw `const TSLanguage *` the capsule mechanism needs — and there is no maintained Dart tree-sitter package to construct from it | Use the generated `Parser` wrapper  |

!!! Note "Why Dart is different"

    Python and Node are not C-ABI bindings either, yet they *do* support passthrough — PyO3 and
    napi-rs let the binding return an arbitrary host object (the upstream `tree_sitter.Language` /
    the npm `Language`). `flutter_rust_bridge` has no equivalent escape hatch: it auto-wraps every
    Rust return value in its own `RustOpaque` proxy with an `Arc` lifecycle, so there is no point
    in the call where a bare `const TSLanguage *` could be handed to a host `Language(pointer)`
    constructor. Dart passthrough would require both an FRB raw-pointer path and a maintained Dart
    tree-sitter package — a meaningfully larger effort than the C-ABI bindings.

For opaque-handle bindings, use the high-level `process()` function or the package's `getParser()` method instead:

=== "Ruby"

    ```ruby
    require "tree_sitter_language_pack"

    # Use getParser() to get a pre-configured parser
    parser = TreeSitterLanguagePack.get_parser("python")

    # Or use process() for structured analysis
    result = TreeSitterLanguagePack.process(source, { language: "python" })
    ```

=== "PHP"

    ```php
    use TreeSitterLanguagePack\TreeSitterLanguagePack;

    // Use getParser() for a pre-configured parser
    $parser = TreeSitterLanguagePack::getParser("python");

    // Or use process() for structured analysis
    $result = TreeSitterLanguagePack::process($source, ["language" => "python"]);
    ```

---

## Error Handling

All passthrough bindings require you to handle the possibility that `get_language()` fails (e.g., unknown language name):

=== "Python"

    ```python
    from tree_sitter_language_pack import LanguageNotFoundError

    try:
        lang = get_language("unknown_lang")
    except LanguageNotFoundError as e:
        print(f"Language not found: {e}")
    ```

=== "Node.js"

    ```typescript
    try {
        const lang = tslp.getLanguage("unknown_lang");
    } catch (err) {
        console.error(`Language not found: ${(err as Error).message}`);
    }
    ```

=== "Go"

    ```go
    lang, err := tspack.GetLanguage("unknown_lang")
    if err != nil {
        // Check if it's a language-not-found error
        return fmt.Errorf("getting language: %w", err)
    }
    ```

=== "Java"

    ```java
    try {
        Language lang = TreeSitterLanguagePack.getLanguage("unknown_lang");
    } catch (TreeSitterLanguagePackRsException e) {
        System.err.println("Language not found: " + e.getMessage());
    }
    ```

---

## Why Passthrough?

A passthrough binding returns the ecosystem's standard tree-sitter `Language`, so:

1. The result works with the existing tree-sitter tooling in that ecosystem — query APIs, tree cursors, and editor integrations — with no wrapper in between.
2. You choose when to upgrade your host tree-sitter package; the grammars are compiled at a backwards-compatible ABI (see below).
3. You can use these 306 grammars alongside custom grammars or other tree-sitter packages in the same parser instance.

---

## ABI Compatibility Window

The language pack is compiled at tree-sitter **ABI 14**, which is stable across:

- Tree-sitter runtime versions 0.21 through 0.26
- All host package versions listed in the table above

This wide compatibility ensures you can use any reasonably recent version of your ecosystem's tree-sitter package without pinning or rebuilding the language pack.
