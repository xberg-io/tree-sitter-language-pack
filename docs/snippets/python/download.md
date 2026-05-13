```python title="Python"
import tree_sitter_language_pack as tslp

# Pre-download specific languages
tslp.download(["python", "javascript", "rust"])

# Or initialize with config
tslp.init(tslp.PackConfig(languages=["python", "go"], cache_dir="/tmp/parsers"))

# Check what's cached
print(tslp.downloaded_languages())
print(tslp.manifest_languages()[:5])
```
