module github.com/xberg-io/tree-sitter-language-pack/packages/go/e2e

go 1.26

require (
	github.com/stretchr/testify v1.12.1
	github.com/tree-sitter/go-tree-sitter v0.25.0
	github.com/xberg-io/tree-sitter-language-pack/packages/go v1.21.2
)

require (
	github.com/mattn/go-pointer v0.0.1 // indirect
	github.com/stretchr/objx v0.5.3 // indirect
	go.yaml.in/yaml/v3 v3.0.5 // indirect
)

replace github.com/xberg-io/tree-sitter-language-pack/packages/go => ../../packages/go
