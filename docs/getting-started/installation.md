---
title: Installation
description: "Install tree-sitter-language-pack in Python, Node.js, Rust, Go, Java, C#, Ruby, Elixir, PHP, Dart, Kotlin Android, Swift, Zig, C FFI, WebAssembly, or via the CLI."
---

## Install using the CLI

The `ts-pack` CLI allows you to manage parsers and run code analysis directly from your terminal.
You can use it in CI pipelines, automation scripts, or to explore and experiment with **306 supported languages** <span class="version-badge">Available by v1.9</span>.

<div class="install-cli-hero" markdown>

## :material-console-line: CLI <span class="version-badge">Homebrew bundles available by v1.9</span>

=== ":fontawesome-brands-apple: Homebrew"

    ```bash
    brew tap xberg-io/homebrew-tap
    brew install xberg-io/homebrew-tap/ts-pack
    ```

=== ":material-package-variant-closed: Cargo"

    ```bash
    cargo install ts-pack-cli
    ```

```bash title="Verify"
ts-pack --version
ts-pack list | wc -l   # → 306
```

</div>

### Run the MCP Server

The CLI bundles an MCP server for AI agents:

```bash
ts-pack mcp
```

See the [MCP Server guide](../guides/mcp-server.md) for integration with Claude Code, Cursor, VS Code, and other tools.

---

## Language Bindings

Tree-sitter-language-pack is available for every major ecosystem. All packages share the same version and API surface.

<div class="lang-picker" markdown>
[Python](#python){ .lang-pill }
[Node.js](#nodejs){ .lang-pill }
[Rust](#rust){ .lang-pill }
[Go](#go){ .lang-pill }
[Java](#java){ .lang-pill }
[C#](#c){ .lang-pill }
[Ruby](#ruby){ .lang-pill }
[Elixir](#elixir){ .lang-pill }
[PHP](#php){ .lang-pill }
[Dart](#dart){ .lang-pill }
[Kotlin Android](#kotlin-android){ .lang-pill }
[Swift](#swift){ .lang-pill }
[Zig](#zig){ .lang-pill }
[C FFI](#c-ffi){ .lang-pill }
[WebAssembly](#webassembly){ .lang-pill }
</div>

---

### Python

Requires Python 3.10+.

=== "pip"

    ```bash
    pip install tree-sitter-language-pack
    ```

=== "uv"

    ```bash
    uv add tree-sitter-language-pack
    ```

=== "poetry"

    ```bash
    poetry add tree-sitter-language-pack
    ```

Verify:

```python
import tree_sitter_language_pack as tslp
print(tslp.language_count())  # 306
```

---

### Node.js

Requires Node.js 18+. Pre-built binaries for Linux (x64, arm64), macOS (x64, arm64), and Windows (x64).

=== "npm"

    ```bash
    npm install @xberg-io/tree-sitter-language-pack
    ```

=== "pnpm"

    ```bash
    pnpm add @xberg-io/tree-sitter-language-pack
    ```

=== "Yarn"

    ```bash
    yarn add @xberg-io/tree-sitter-language-pack
    ```

Verify:

```javascript
const tslp = require("@xberg-io/tree-sitter-language-pack");
console.log(tslp.languageCount()); // 306
```

---

### Rust

Requires Rust 1.85+.

=== "Cargo CLI"

    ```bash
    cargo add tree-sitter-language-pack
    ```

=== "Cargo.toml"

    ```toml
    [dependencies]
    tree-sitter-language-pack = "1"
    ```

Verify:

```rust
fn main() {
    println!("{}", tree_sitter_language_pack::language_count()); // 306
}
```

---

### Go

Requires Go 1.26+. The binding uses cgo and links against the pre-compiled C FFI library.

```bash
go get github.com/xberg-io/tree-sitter-language-pack/packages/go
```

```go
import tslp "github.com/xberg-io/tree-sitter-language-pack/packages/go"

func main() {
    fmt.Println(tslp.LanguageCount()) // 306
}
```

---

### Java

Requires JDK 25+ (uses Panama FFM API).

=== "Maven"

    ```xml
    <dependency>
        <groupId>io.xberg.treesitterlanguagepack</groupId>
        <artifactId>tree-sitter-language-pack</artifactId>
        <version>1.9.0-rc.49</version>
    </dependency>
    ```

=== "Gradle (Kotlin)"

    ```kotlin
    dependencies {
        implementation("io.xberg.treesitterlanguagepack:tree-sitter-language-pack:1.9.0-rc.49")
    }
    ```

=== "Gradle (Groovy)"

    ```groovy
    dependencies {
        implementation 'io.xberg.treesitterlanguagepack:tree-sitter-language-pack:1.9.0-rc.49'
    }
    ```

Verify:

```java
import io.xberg.treesitterlanguagepack.TreeSitterLanguagePack;

public class Main {
    public static void main(String[] args) {
        System.out.println(TreeSitterLanguagePack.languageCount()); // 306
    }
}
```

---

### C# <span class="version-badge">Available by v1.9</span> {#c}

Requires .NET 10.

```bash
dotnet add package TreeSitterLanguagePack --version 1.9.0-rc.49
```

Verify:

```csharp
using TreeSitterLanguagePack;

Console.WriteLine(TreeSitterLanguagePackConverter.LanguageCount()); // 306
```

The NuGet package includes native runtime assets for Windows, Linux, and macOS.

---

### Ruby

Requires Ruby 3.2+.

=== "gem"

    ```bash
    gem install tree_sitter_language_pack
    ```

=== "Gemfile"

    ```ruby
    gem "tree_sitter_language_pack", "~> 1.9"
    ```

    ```bash
    bundle install
    ```

Verify:

```ruby
require "tree_sitter_language_pack"

puts TreeSitterLanguagePack.language_count # 306
```

---

### Elixir

Requires Elixir 1.14+ and OTP 25+.

=== "mix.exs"

    ```elixir
    defp deps do
      [
        {:tree_sitter_language_pack, "~> 1.9"}
      ]
    end
    ```

    ```bash
    mix deps.get
    ```

Verify:

```elixir
IO.puts TreeSitterLanguagePack.language_count() # 306
```

---

### PHP

Requires PHP 8.2+. This package is a native PHP extension (`type: php-ext`). Install
[`mlocati/php-extension-installer`](https://github.com/mlocati/php-extension-installer) first
so Composer can download and register the compiled `.so`/`.dll`:

```bash
composer require mlocati/php-extension-installer
```

=== "Composer"

    ```bash
    composer require xberg-io/tree-sitter-language-pack
    ```

=== "composer.json"

    ```json
    {
        "require": {
            "mlocati/php-extension-installer": "^2.0",
            "xberg-io/tree-sitter-language-pack": "^1.9"
        }
    }
    ```

Verify:

```php
<?php
echo \ts_pack_language_count(); // 306
```

---

### Dart

Requires Dart 3.11+.

```bash
dart pub add tree_sitter_language_pack
```

---

### Kotlin Android <span class="version-badge">Available by v1.9</span> {#kotlin-android}

Requires Android minSdk 21 and Java 17 bytecode.

```kotlin
implementation("io.xberg.tslp.android:tree-sitter-language-pack-android:1.9.0-rc.49")
```

This is an Android AAR. Kotlin/JVM users should use the Java artifact.

---

### Swift

Requires Swift 6.0+.

```swift
.package(
    url: "https://github.com/xberg-io/tree-sitter-language-pack",
    exact: "1.9.0-rc.49"
)
```

---

### Zig

Requires Zig 0.16+.

```bash
zig fetch --save <release tarball url>
```

The package name is `tree_sitter_language_pack`.

---

### C FFI

Download the shared library and generated C header from GitHub Releases. The C FFI is the stable native contract used by the generated native packages.

---

### WebAssembly

Use from any JavaScript environment — browsers, Deno, and Cloudflare Workers.

=== "npm"

    ```bash
    npm install @xberg-io/tree-sitter-language-pack-wasm
    ```

=== "CDN (browser)"

    ```html
    <script type="module">
      import { availableLanguages, parseString } from "https://cdn.jsdelivr.net/npm/@xberg-io/tree-sitter-language-pack-wasm/+esm";
      console.log(availableLanguages());
    </script>
    ```

=== "Deno"

    ```typescript
    import { availableLanguages, parseString } from "npm:@xberg-io/tree-sitter-language-pack-wasm";
    console.log(availableLanguages());
    ```

The WASM package is a static curated subset of parsers compiled into the module. It does not expose the native download/cache helpers.

---

## Next Steps

- [:material-arrow-right: Quick Start](quickstart.md) — parse your first file in 5 minutes
- [:material-arrow-right: Download parsers](quickstart.md#2-download-parsers) — pre-download grammars for production, CI, or offline use
- [:material-arrow-right: Download model](../concepts/download-model.md) — understand how parser caching works
- [:material-arrow-right: Languages](../languages.md) — full list of 306 supported languages
