---
title: Installation
description: "Install tree-sitter-language-pack in Python, Node.js, Rust, Go, Java, Ruby, Elixir, PHP, WebAssembly, or via the CLI."
---

## Install using the CLI

The `ts-pack` CLI allows you to manage parsers and run code analysis directly from your terminal.
You can use it in CI pipelines, automation scripts, or to explore and experiment with **300+ supported languages**.

<div class="install-cli-hero" markdown>

## :material-console-line: CLI

=== ":fontawesome-brands-apple: Homebrew"

    ```bash
    brew install kreuzberg-dev/tap/ts-pack
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

---

## Language Bindings

Tree-sitter-language-pack is available for every major ecosystem. All packages share the same version and API surface.

<div class="lang-picker" markdown>
[Python](#python){ .lang-pill }
[Node.js](#nodejs){ .lang-pill }
[Rust](#rust){ .lang-pill }
[Go](#go){ .lang-pill }
[Java](#java){ .lang-pill }
[Ruby](#ruby){ .lang-pill }
[Elixir](#elixir){ .lang-pill }
[PHP](#php){ .lang-pill }
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
    npm install @kreuzberg/tree-sitter-language-pack
    ```

=== "pnpm"

    ```bash
    pnpm add @kreuzberg/tree-sitter-language-pack
    ```

=== "Yarn"

    ```bash
    yarn add @kreuzberg/tree-sitter-language-pack
    ```

Verify:

```javascript
const tslp = require("@kreuzberg/tree-sitter-language-pack");
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
go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go
```

```go
import tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"

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
        <groupId>dev.kreuzberg.treesitterlanguagepack</groupId>
        <artifactId>tree-sitter-language-pack</artifactId>
        <version>1.8.1</version>
    </dependency>
    ```

=== "Gradle (Kotlin)"

    ```kotlin
    dependencies {
        implementation("dev.kreuzberg.treesitterlanguagepack:tree-sitter-language-pack:1.8.1")
    }
    ```

=== "Gradle (Groovy)"

    ```groovy
    dependencies {
        implementation 'dev.kreuzberg.treesitterlanguagepack:tree-sitter-language-pack:1.8.1'
    }
    ```

Verify:

```java
import dev.kreuzberg.treesitterlanguagepack.TreeSitterLanguagePack;

public class Main {
    public static void main(String[] args) {
        System.out.println(TreeSitterLanguagePack.languageCount()); // 306
    }
}
```

---

### Ruby

Requires Ruby 3.2+.

=== "gem"

    ```bash
    gem install tree_sitter_language_pack
    ```

=== "Gemfile"

    ```ruby
    gem "tree_sitter_language_pack", "~> 1.8"
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
        {:tree_sitter_language_pack, "~> 1.8"}
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
    composer require kreuzberg-dev/tree-sitter-language-pack
    ```

=== "composer.json"

    ```json
    {
        "require": {
            "mlocati/php-extension-installer": "^2.0",
            "kreuzberg-dev/tree-sitter-language-pack": "^1.0"
        }
    }
    ```

Verify:

```php
<?php
echo \ts_pack_language_count(); // 306
```

---

### WebAssembly

Use from any JavaScript environment — browsers, Deno, and Cloudflare Workers.

=== "npm"

    ```bash
    npm install @kreuzberg/tree-sitter-language-pack-wasm
    ```

=== "CDN (browser)"

    ```html
    <script type="module">
      import { availableLanguages, parseString } from "https://cdn.jsdelivr.net/npm/@kreuzberg/tree-sitter-language-pack-wasm/+esm";
      console.log(availableLanguages());
    </script>
    ```

=== "Deno"

    ```typescript
    import { availableLanguages, parseString } from "npm:@kreuzberg/tree-sitter-language-pack-wasm";
    console.log(availableLanguages());
    ```

---

## Next Steps

- [:material-arrow-right: Quick Start](quickstart.md) — parse your first file in 5 minutes
- [:material-arrow-right: Download parsers](quickstart.md#2-download-parsers) — pre-download grammars for production, CI, or offline use
- [:material-arrow-right: Download model](../concepts/download-model.md) — understand how parser caching works
- [:material-arrow-right: Languages](../languages.md) — full list of 306 supported languages
