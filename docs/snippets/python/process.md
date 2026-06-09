```python title="Python"
import tree_sitter_language_pack as tslp

config = tslp.ProcessConfig(
    language="python",
    structure=True,
    imports=True,
    comments=True,
    chunk_max_size=1000,
)

result = tslp.process('''
import os
from pathlib import Path

def read_file(path: str) -> str:
    """Read a file and return its contents."""
    return Path(path).read_text()

class FileReader:
    def __init__(self, base_dir: str):
        self.base_dir = base_dir
''', config)

for item in result.structure:
    print(f"{item.kind}: {item.name}")

for imp in result.imports:
    print(f"import: {imp.source}")
```
