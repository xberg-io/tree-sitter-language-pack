```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/xberg-io/tree-sitter-language-pack/packages/go"
)

func main() {
	config := tspack.NewProcessConfig(
		tspack.WithProcessConfigLanguage("go"),
		tspack.WithProcessConfigStructure(true),
		tspack.WithProcessConfigImports(true),
	)
	result, err := tspack.Process(
		"package main\nimport \"fmt\"\nfunc hello() { fmt.Println(\"hi\") }",
		*config,
	)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println("Language:", result.Language)
	for _, item := range result.Structure {
		fmt.Printf("%s: %s\n", item.Kind, item.Name)
	}
	for _, imp := range result.Imports {
		fmt.Println("import:", imp.Source)
	}
}
```
