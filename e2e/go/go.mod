module e2e_go

go 1.26

require (
	github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go v1.9.0-rc.15
	github.com/stretchr/testify v1.11.1
)

replace github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go => ../../packages/go
