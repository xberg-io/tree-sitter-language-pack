```swift title="Swift"
import TreeSitterLanguagePack
import RustBridge

let config = try processConfigFromJson(#"""
{
  "language": "python",
  "structure": true,
  "imports": true
}
"""#)

let source = """
import os
from pathlib import Path

def read_file(path: str) -> str:
    return Path(path).read_text()

class FileReader:
    def __init__(self, base_dir: str):
        self.base_dir = base_dir
"""

let result = try process(source, config)

for item in result.structure() {
    let kind = item.kind().toString()
    let name = item.name()?.toString() ?? "<anonymous>"
    print("\(kind): \(name)")
}

for imp in result.imports() {
    print("import: \(imp.source().toString())")
}
```
