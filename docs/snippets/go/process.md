```go title="Go"
package main

import (
	"fmt"
	"log"

	tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func main() {
	config := tslp.ProcessConfig{Language: "go"}
	result, err := tslp.Process(
		"package main\nimport \"fmt\"\nfunc hello() { fmt.Println(\"hi\") }",
		config,
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
