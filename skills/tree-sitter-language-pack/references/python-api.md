# Python API Quick Reference

## Installation

```bash
pip install tree-sitter-language-pack
```

## Language Discovery

```python
available_languages() -> list[str]
has_language(name: str) -> bool
language_count() -> int
detect_language(path: str) -> str | None
detect_language_from_content(content: str) -> str | None
detect_language_from_extension(ext: str) -> str | None
detect_language_from_path(path: str) -> str | None
extension_ambiguity(ext: str) -> tuple[str, list[str]] | None
```

## Parsing

```python
parse_string(language: str, source: str) -> TreeHandle
# Returns opaque tree handle

tree.root_node_type() -> str
tree.root_child_count() -> int
tree.contains_node_type(node_type: str) -> bool
tree.has_error_nodes() -> bool
tree.error_count() -> int
tree.to_sexp() -> str
tree.root_node_info() -> dict  # {kind, is_named, start_byte, end_byte, ...}
tree.find_nodes_by_type(node_type: str) -> list[dict]
tree.named_children_info() -> list[dict]
tree.extract_text(start_byte: int, end_byte: int) -> str
tree.run_query(language: str, query_source: str) -> list[dict]
```

Example:

```python
tree = parse_string("python", "def hello(): pass")
print(tree.root_node_type())          # "module"
print(tree.root_child_count())         # 1
print(tree.contains_node_type("function_definition"))  # True
print(tree.to_sexp())  # S-expression
```

## Code Intelligence Processing

```python
process(source: str, config: ProcessConfig) -> dict
# Returns {language, metrics, structure, imports, exports, comments,
#          docstrings, symbols, diagnostics, chunks}
```

## ProcessConfig

```python
ProcessConfig(
    language: str,
    structure: bool = True,
    imports: bool = True,
    exports: bool = True,
    comments: bool = False,
    docstrings: bool = False,
    symbols: bool = False,
    diagnostics: bool = False,
    chunk_max_size: int | None = None,
    extractions: dict | None = None,
)

# Static constructors:
ProcessConfig.all(language: str)      # All features enabled
ProcessConfig.minimal(language: str)  # All features disabled
```

Example:

```python
config = ProcessConfig(
    "python",
    structure=True,
    imports=True,
    comments=True,
    chunk_max_size=1000,
)
result = process("import os\ndef foo(): pass", config)
print(result["structure"])    # List of functions/classes
print(result["imports"])       # List of imports
print(result["chunks"])        # Code chunks for LLMs
```

ProcessResult fields:

- `language` (str): Language used
- `metrics` (dict): {total_lines, code_lines, comment_lines, blank_lines, total_bytes, error_count}
- `structure` (list): Functions, classes, methods
- `imports` (list): Import statements
- `exports` (list): Exported symbols
- `comments` (list): Inline and block comments
- `docstrings` (list): Docstrings with parsed sections
- `symbols` (list): All identifiers
- `diagnostics` (list): Syntax errors, {message, severity, span}
- `chunks` (list): Code chunks, {content, start_line, end_line, metadata}

## Extraction Queries

```python
extract(source: str, config: dict) -> dict
# Returns {language, results: {pattern_name: {matches, total_count}}}

validate_extraction(config: dict) -> dict
# Returns {valid, patterns: {name: {valid, capture_names, pattern_count, errors, warnings}}}
```

Pattern config fields:

- `query` (str): Tree-sitter S-expression query
- `capture_output` (str): "Text", "Node", or "Full" (default)
- `child_fields` (list): Field names to extract
- `max_results` (int | None): Max matches to return
- `byte_range` ([int, int] | None): Restrict to byte range

Example:

```python
result = extract("def hello(): pass\ndef world(): pass", {
    "language": "python",
    "patterns": {
        "functions": {
            "query": "(function_definition name: (identifier) @fn_name)",
            "capture_output": "Text",
        },
    },
})

for match in result["results"]["functions"]["matches"]:
    for capture in match["captures"]:
        print(capture["text"])  # "hello", "world"
```

## Bundled Queries

```python
get_highlights_query(language: str) -> str | None
get_injections_query(language: str) -> str | None
get_locals_query(language: str) -> str | None
```

## Download & Configuration

```python
init(config: dict) -> None
# {cache_dir?: str, languages?: list[str], groups?: list[str]}

configure(cache_dir: str | None = None) -> None
download(names: list[str]) -> int  # Returns count of newly downloaded
download_all() -> int
manifest_languages() -> list[str]
downloaded_languages() -> list[str]
clean_cache() -> None
cache_dir() -> str
```

Example:

```python
# Pre-download languages
init({"languages": ["python", "javascript", "rust"]})

# Set custom cache directory
configure(cache_dir="/opt/parsers")

# Download on-demand
download(["python", "typescript"])

# Check what's cached
print(downloaded_languages())
```

## tree-sitter Interop

```python
get_binding(name: str) -> PyCapsule  # Raw TSLanguage pointer
get_language(name: str) -> tree_sitter.Language
get_parser(name: str) -> tree_sitter.Parser
```

## Exceptions

- `LanguageNotFoundError`: Language not available
- `ParseError`: Parse or tree operation failed
- `QueryError`: Tree-sitter query syntax error
- `DownloadError`: Download or cache operation failed

Example:

```python
from tree_sitter_language_pack import parse_string, ParseError

try:
    tree = parse_string("nonexistent_lang", "code")
except ParseError as e:
    print(f"Error: {e}")
```
