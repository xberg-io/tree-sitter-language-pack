```python title="Python"
import tree_sitter_language_pack as tslp

# Parsers download automatically on first use
result = tslp.process(
    "def hello():\n    print('world')\n",
    tslp.ProcessConfig(language="python", structure=True, imports=True),
)

print(f"Language: {result.language}")
print(f"Functions: {len(result.structure)}")
```
