{% if language == "rust" %}

```rust
use tree_sitter_language_pack::{get_language, get_parser};

let mut parser = get_parser("python").expect("language available");
let tree = parser.parse("def hello(): pass", None).unwrap();
println!("{}", tree.root_node().to_sexp());
```

{% elif language == "python" %}

```python
from tree_sitter_language_pack import get_parser

parser = get_parser("python")
tree = parser.parse(b"def hello(): pass")
print(tree.root_node.sexp())
```

{% elif language in ["typescript", "node"] %}

```typescript
import { getParser } from "@xberg-io/tree-sitter-language-pack";

const parser = getParser("python");
const tree = parser.parse("def hello(): pass");
console.log(tree.rootNode.toString());
```

{% elif language == "wasm" %}

```typescript
import init, { getParser } from "@xberg-io/tree-sitter-language-pack-wasm";

await init();
const parser = getParser("python");
const tree = parser.parse("def hello(): pass");
```

{% else %}
See the [language guide](https://docs.tree-sitter-language-pack.xberg.io) for `{{ language }}`-specific usage.
{% endif %}
