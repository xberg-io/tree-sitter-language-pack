```swift title="Swift"
import TreeSitterLanguagePack
import RustBridge

// Parsers download automatically on first use.
let config = try processConfigFromJson(#"{"language":"swift","structure":true}"#)
let result = try process("func greet() { print(\"hello\") }", config)

print("Language: \(result.language().toString())")
print("Functions: \(result.structure().count)")
print("Total lines: \(result.metrics().total_lines())")
```
