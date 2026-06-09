```swift title="Swift"
import TreeSitterLanguagePack
import RustBridge

// Pre-download specific languages.
let names = RustVec<String>()
names.push(value: "python")
names.push(value: "javascript")
names.push(value: "rust")
let installed = try download(names)
print("Downloaded \(installed) parsers")

// Or initialize with config (cache_dir + languages).
let packConfig = try packConfigFromJson(
    #"{"cache_dir":"/tmp/parsers","languages":["python","go"]}"#
)
try init(packConfig)

// Inspect downloaded state.
let cached = downloadedLanguages().map { $0.as_str().toString() }
let manifest = try manifestLanguages().map { $0.as_str().toString() }
print("Cached: \(cached)")
print("Manifest sample: \(manifest.prefix(5))")
```
