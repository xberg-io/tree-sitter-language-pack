---
title: Host-Native Language Passthrough
description: "How get_language() integrates with native tree-sitter packages across ecosystems."
---

Tree-sitter-language-pack's `get_language()` function returns the **native tree-sitter `Language` type** for your ecosystem—not a wrapper or opaque handle. This means you can pass the result directly to your parser without translation overhead or intermediate APIs.

---

## Passthrough vs. Opaque Bindings

### Passthrough Bindings

These 8 bindings return the real `Language` type from the host ecosystem's tree-sitter package:

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
        tspack "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
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

### Opaque-Handle Bindings

These 7 bindings return an opaque handle specific to this package (they lack a native tree-sitter implementation that accepts raw pointers):

| Binding    | Recommendation                         |
| ---------- | -------------------------------------- |
| **Ruby**   | Use this package's `Parser` wrapper    |
| **PHP**    | Use this package's extension API       |
| **Elixir** | Use this package's NIF wrapper         |
| **Dart**   | Use the generated `Parser` wrapper     |
| **WASM**   | Use this package's JS wrapper          |
| **C FFI**  | Use the exported FFI functions         |

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

- tree-sitter runtime versions 0.21 through 0.26
- All host package versions listed in the table above

This wide compatibility ensures you can use any reasonably recent version of your ecosystem's tree-sitter package without pinning or rebuilding the language pack.
