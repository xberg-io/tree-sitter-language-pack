```go title="Go"
package main

import (
	"fmt"
	"log"

	tslp "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

func main() {
	config := tslp.PackConfig{
		Languages: []string{"go", "python"},
	}
	if err := tslp.Init(config); err != nil {
		log.Fatal(err)
	}

	count, err := tslp.Download([]string{"rust", "javascript"})
	if err != nil {
		log.Fatal(err)
	}
	if count != nil {
		fmt.Printf("Ensured %d languages\n", *count)
	}

	for _, name := range tslp.DownloadedLanguages() {
		fmt.Println("cached:", name)
	}
}
```
