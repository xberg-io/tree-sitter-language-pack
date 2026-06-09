```go title="Go"
package main

import (
	"fmt"
	"log"

	"github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func main() {
	config := tspack.PackConfig{
		Languages: []string{"go", "python"},
	}
	if err := tspack.Init(config); err != nil {
		log.Fatal(err)
	}

	count, err := tspack.Download([]string{"rust", "javascript"})
	if err != nil {
		log.Fatal(err)
	}
	if count != nil {
		fmt.Printf("Ensured %d languages\n", *count)
	}

	for _, name := range tspack.DownloadedLanguages() {
		fmt.Println("cached:", name)
	}
}
```
