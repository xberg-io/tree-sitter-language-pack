{% if language == "rust" %}

```bash
cargo add tree-sitter-language-pack
```

{% elif language == "python" %}

```bash
pip install tree-sitter-language-pack
```

{% elif language in ["typescript", "node"] %}

```bash
npm install @xberg-io/tree-sitter-language-pack
```

{% elif language == "wasm" %}

```bash
npm install @xberg-io/tree-sitter-language-pack-wasm
```

{% elif language == "ruby" %}

```bash
gem install tree_sitter_language_pack
```

{% elif language == "php" %}

```bash
composer require xberg-io/tree-sitter-language-pack
```

{% elif language == "go" %}

```bash
go get {{ package_name }}
```

{% elif language == "java" %}

```xml
<dependency>
  <groupId>io.xberg.treesitterlanguagepack</groupId>
  <artifactId>tree-sitter-language-pack</artifactId>
  <version>{{ version }}</version>
</dependency>
```

{% elif language == "csharp" %}

```bash
dotnet add package XbergIo.TreeSitterLanguagePack
```

{% elif language == "elixir" %}
Add to `mix.exs`:

```elixir
defp deps do
  [
    {:tree_sitter_language_pack, "~> {{ version }}"}
  ]
end
```

{% elif language == "ffi" %}
Download the prebuilt static/dynamic library from the [GitHub releases page](https://github.com/xberg-io/tree-sitter-language-pack/releases) or build from source:

```bash
git clone https://github.com/xberg-io/tree-sitter-language-pack
cargo build --release -p tree-sitter-language-pack-ffi
```

{% elif language == "dart" %}

```bash
dart pub add tree_sitter_language_pack
```

Flutter projects use `flutter pub add tree_sitter_language_pack` instead.

{% elif language == "kotlin_android" %}
Add to your module's `build.gradle.kts`:

```kotlin
dependencies {
    implementation("io.xberg.tslp.android:tree-sitter-language-pack-android:{{ version }}")
}
```

{% elif language == "swift" %}
Requires Swift 6.0+. Add to your `Package.swift`:

```swift
.package(
    url: "https://github.com/xberg-io/tree-sitter-language-pack",
    exact: "{{ version }}"
)
```

{% elif language == "zig" %}
Requires Zig 0.16+. Fetch the `{{ version }}` package tarball from the [GitHub releases page](https://github.com/xberg-io/tree-sitter-language-pack/releases):

```bash
zig fetch --save <release-tarball-url>
```

Then add the dependency to your `build.zig`:

```zig
const tslp = b.dependency("tree_sitter_language_pack", .{
    .target = target,
    .optimize = optimize,
});
exe.root_module.addImport("tree_sitter_language_pack", tslp.module("tree_sitter_language_pack"));
```

{% endif %}
