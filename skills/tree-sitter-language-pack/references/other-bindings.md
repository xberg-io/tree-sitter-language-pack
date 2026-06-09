# Other Language Bindings Quick Reference

## Go

### Installation

```bash
go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go
```

### Quick Start

```go
package main

import (
    "fmt"
    tspack "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func main() {
    reg, _ := tspack.NewRegistry()
    defer reg.Close()

    tree, _ := reg.ParseString("python", "def hello(): pass")
    defer tree.Close()

    fmt.Println(tree.RootNodeType())          // "module"
    fmt.Println(tree.ContainsNodeType("function_definition"))  // true

    config := tspack.NewProcessConfig("python")
    result, _ := reg.Process("def hello(): pass", config)
    fmt.Printf("Functions: %d\n", len(result.Metadata.Structure))
}
```

### Key Differences

- Registry is thread-safe, must be closed: `defer reg.Close()`
- Trees must be closed: `defer tree.Close()`
- Config is `ProcessConfig` struct, not JSON
- Errors are returned as second value
- All functions are synchronous

---

## Java

### Installation

Requires JDK 25+ (Panama FFI). Set `TSPACK_LIB_PATH` to native library path.

Maven:

```xml
<dependency>
    <groupId>dev.kreuzberg.treesitterlanguagepack</groupId>
    <artifactId>tree-sitter-language-pack</artifactId>
    <version>1.8.0</version>
</dependency>
```

### Quick Start

```java
import dev.kreuzberg.treesitterlanguagepack.*;

public class Main {
    public static void main(String[] args) {
        try (var registry = new TsPackRegistry()) {
            var languages = registry.availableLanguages();
            System.out.printf("%d languages available%n", languages.size());

            try (var tree = registry.parseString("python", "def hello(): pass")) {
                System.out.println(tree.rootNodeType());     // "module"
                System.out.println(tree.rootChildCount());   // 1
                System.out.println(tree.containsNodeType("function_definition")); // true
            }

            String configJson = "{\"language\":\"python\",\"structure\":true}";
            String resultJson = registry.process("def hello(): pass", configJson);
            System.out.println(resultJson);
        }
    }
}
```

### Key Differences

- Implements `AutoCloseable`, use try-with-resources: `try (var registry = new TsPackRegistry())`
- Config and results are JSON strings, decode/encode manually
- Static methods for download/init (don't require registry instance)
- Specific exception: `LanguageNotFoundException`

---

## C# / .NET

### Installation

Requires .NET 10+.

```bash
dotnet add package TreeSitterLanguagePack
```

### Quick Start

```csharp
using TreeSitterLanguagePack;

// List languages
string[] langs = TsPackClient.AvailableLanguages();
Console.WriteLine($"{langs.Length} languages available");

// Parse code
using var tree = TsPackClient.Parse("python", "def hello(): pass");
Console.WriteLine(tree.RootNodeType());  // "module"

// Process code
var config = new ProcessConfig { Language = "python", Structure = true };
var result = TsPackClient.Process("def hello(): pass", config);
Console.WriteLine($"Functions: {result.Structure.Count}");
```

### Key Differences

- Static client class `TsPackClient` (not instance-based)
- Config is typed class `ProcessConfig`, results are deserialized objects
- Tree implements `IDisposable`, use `using` statement
- Exceptions: `TsPackException` (inherits Exception)
- Thread-safe with lazy initialization

---

## Ruby

### Installation

```bash
gem install tree_sitter_language_pack
# Or in Gemfile:
gem "tree_sitter_language_pack"
```

### Quick Start

```ruby
require "tree_sitter_language_pack"

# List languages
langs = TreeSitterLanguagePack.available_languages
puts "#{langs.length} languages available"

# Parse code
tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass")
puts tree.root_node_type         # "module"
puts tree.has_error_nodes        # false
puts tree.contains_node_type("function_definition") # true

# Process code (returns JSON string)
config = { language: "python", structure: true }.to_json
result_json = TreeSitterLanguagePack.process("def hello(): pass", config)
result = JSON.parse(result_json)
puts "Functions: #{result['structure'].length}"
```

### Key Differences

- All module-level functions (no classes)
- Parse returns opaque Tree reference (not dereferenceable)
- Config and results are JSON strings, convert with `.to_json` and `JSON.parse()`
- Errors raised as `RuntimeError`
- Pattern extraction supported: `extract()`, `validate_extraction()`

---

## Elixir

### Installation

Mix:

```elixir
def deps do
  [{:tree_sitter_language_pack, "~> 1.0"}]
end
```

### Quick Start

```elixir
# List languages
langs = TreeSitterLanguagePack.available_languages()
IO.puts("#{length(langs)} languages available")

# Parse code
tree = TreeSitterLanguagePack.parse_string("python", "def hello(): pass")
IO.puts(TreeSitterLanguagePack.tree_root_node_type(tree))  # "module"
IO.puts(TreeSitterLanguagePack.tree_has_error_nodes(tree))  # false

# Process code (returns map)
config = Jason.encode!(%{"language" => "python", "structure" => true})
result = TreeSitterLanguagePack.process("def hello(): pass", config)
IO.puts("Functions: #{length(result["structure"])}")
```

### Key Differences

- All module-level functions
- Config is JSON string, results are maps (not JSON strings)
- Parse returns opaque tree reference
- I/O functions run on DirtyIo scheduler (non-blocking)
- Errors raised as Erlang errors
- Pattern extraction supported: `extract()`, `validate_extraction()`

---

## PHP

### Installation

Composer:

```bash
composer require kreuzberg-dev/tree-sitter-language-pack
```

PHP 8.2+, requires native Rust extension (ext-php-rs).

### Quick Start

```php
<?php
declare(strict_types=1);

use TreeSitterLanguagePack\TreeSitterLanguagePack;
use TreeSitterLanguagePack\ProcessConfig;

// List languages
$langs = TreeSitterLanguagePack::availableLanguages();
echo count($langs) . " languages available\n";

// Parse code (returns S-expression string)
$sexp = TreeSitterLanguagePack::parseString("python", "def hello(): pass");
echo "Tree: $sexp\n";

// Process code
$config = new ProcessConfig("python", structure: true, imports: true);
$result = TreeSitterLanguagePack::process("def hello(): pass", $config);
echo count($result['structure']) . " structure items\n";
```

### Key Differences

- OOP wrapper class around procedural extension functions
- Parse returns S-expression string (not tree object)
- Config is typed `ProcessConfig` class (PHP 8.2 constructor promotion)
- Results are associative arrays (decoded from JSON)
- Use procedural `ts_pack_*` functions directly if preferred
- Exceptions: `Exception` base class

---

## WebAssembly

### Installation

npm:

```bash
npm install @kreuzberg/tree-sitter-language-pack-wasm
```

Browser (ES module):

```html
<script type="module">
  import * as tsp from "https://cdn.jsdelivr.net/npm/@kreuzberg/tree-sitter-language-pack-wasm";
</script>
```

### Quick Start

```javascript
import * as tsp from "@kreuzberg/tree-sitter-language-pack-wasm";

// List languages
const langs = tsp.availableLanguages();
console.log(`${langs.length} languages available`);

// Parse code
const tree = tsp.parseString("python", "def hello(): pass");
console.log(tsp.treeRootNodeType(tree)); // "module"
console.log(tsp.treeHasErrorNodes(tree)); // false
tsp.freeTree(tree);

// Process code (config is JS object)
const result = tsp.process("def hello(): pass", { language: "python" });
console.log(`Functions: ${result.structure.length}`);
```

### Key Differences

- Curated subset of languages (not all 306) optimized for browser/edge
- No download API (stubs that throw)
- Config is JS object, results are JS objects (no JSON conversion needed)
- Single-threaded (use Web Workers for large files)
- Manual memory management: `freeTree()` releases memory promptly
- Pattern extraction supported: `extract()`, `validateExtraction()`

---

## C / FFI

### Installation

Header file: `crates/ts-pack-ffi/include/ts_pack.h`

Link against compiled library:

```bash
gcc -o program program.c -L. -lts_pack_ffi
```

### Quick Start

```c
#include "ts_pack.h"
#include <stdio.h>
#include <string.h>

int main(void) {
    TsPackRegistry* reg = ts_pack_registry_new();
    if (!reg) {
        fprintf(stderr, "Error: %s\n", ts_pack_last_error());
        return 1;
    }

    // Check for Python
    if (!ts_pack_has_language(reg, "python")) {
        fprintf(stderr, "Python not available\n");
        ts_pack_registry_free(reg);
        return 1;
    }

    // Parse code
    const char* code = "def hello(): pass";
    TsPackTree* tree = ts_pack_parse_string(reg, "python", code, strlen(code));
    if (!tree) {
        fprintf(stderr, "Parse error: %s\n", ts_pack_last_error());
        ts_pack_registry_free(reg);
        return 1;
    }

    // Inspect tree
    char* root_type = ts_pack_tree_root_node_type(tree);
    printf("Root: %s\n", root_type);
    ts_pack_free_string(root_type);

    // Cleanup
    ts_pack_tree_free(tree);
    ts_pack_registry_free(reg);
    return 0;
}
```

### Key Differences

- Opaque handles: `TsPackRegistry*`, `TsPackTree*`
- Error handling via thread-local string: `ts_pack_last_error()`
- Manual memory management: `ts_pack_free_string()` for allocated strings
- All results are newly-allocated (caller owns memory)
- #[no_mangle] extern "C" functions for C interop
- Use with cgo (Go), Panama FFI (Java), P/Invoke (C#)

---

## Summary Table

| Language    | Package   | Installation | Config      | Results     | Memory            | Pattern Extraction |
| ----------- | --------- | ------------ | ----------- | ----------- | ----------------- | ------------------ |
| Python      | PyPI      | pip          | Python dict | Python dict | Auto              | Yes                |
| Node.js     | npm       | npm          | JS object   | JS object   | Auto              | Yes                |
| Rust        | crates.io | Cargo        | Struct      | Struct      | Auto              | Yes (compiled)     |
| Go          | go.pkg    | go get       | Struct      | Struct      | Manual (Close)    | No                 |
| Java        | Maven     | mvn          | JSON string | JSON string | Manual (close)    | Yes                |
| C#          | NuGet     | dotnet       | Class       | Classes     | Auto (using)      | No                 |
| Ruby        | RubyGems  | gem          | JSON string | Map         | Auto              | Yes                |
| Elixir      | Hex       | Mix          | JSON string | Map         | Auto              | Yes                |
| PHP         | Packagist | composer     | Class       | Array       | Auto              | Yes                |
| WebAssembly | npm       | npm          | JS object   | JS object   | Manual (freeTree) | Yes                |
| C FFI       | Native    | Link         | N/A         | Strings     | Manual (free)     | Yes                |
