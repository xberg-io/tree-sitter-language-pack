// Hand-written capsule-passthrough test (not fixture-generated); preserved across regen.
package e2e_test

import (
	"testing"

	"github.com/stretchr/testify/require"
	tree_sitter "github.com/tree-sitter/go-tree-sitter"

	tspack "github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go"
)

// TestCapsulePassthroughParsesPython verifies that GetLanguage returns a *tree_sitter.Language
// usable directly with upstream go-tree-sitter parser. Verifies host-native Language passthrough (#143).
func TestCapsulePassthroughParsesPython(t *testing.T) {
	// Get the Language from tspack
	lang, err := tspack.GetLanguage("python")
	require.NoError(t, err, "GetLanguage('python') should not error")
	require.NotNil(t, lang, "GetLanguage('python') should return non-nil Language")

	// Feed the Language into upstream tree_sitter parser
	parser := tree_sitter.NewParser()
	require.NotNil(t, parser, "NewParser() should succeed")
	defer parser.Close()

	parser.SetLanguage(lang)

	// Parse Python code
	source := "def greet(name):\n    return name\n"
	tree := parser.Parse([]byte(source), nil)
	require.NotNil(t, tree, "Parse() should return non-nil tree")
	defer tree.Close()

	// Verify the root node kind is "module"
	root := tree.RootNode()
	require.NotNil(t, root, "RootNode() should return non-nil node")
	require.Equal(t, "module", root.Kind(), "root node kind should be 'module' for Python source")
}

// TestCapsulePassthroughDistinctLanguages verifies that different languages work correctly
// when passed through GetLanguage to the upstream parser.
func TestCapsulePassthroughDistinctLanguages(t *testing.T) {
	// Get the JavaScript Language
	lang, err := tspack.GetLanguage("javascript")
	require.NoError(t, err, "GetLanguage('javascript') should not error")
	require.NotNil(t, lang, "GetLanguage('javascript') should return non-nil Language")

	// Feed to upstream parser
	parser := tree_sitter.NewParser()
	require.NotNil(t, parser, "NewParser() should succeed")
	defer parser.Close()

	parser.SetLanguage(lang)

	// Parse JavaScript code
	source := "const x = 1;\n"
	tree := parser.Parse([]byte(source), nil)
	require.NotNil(t, tree, "Parse() should return non-nil tree")
	defer tree.Close()

	// Verify the root node kind is "program"
	root := tree.RootNode()
	require.NotNil(t, root, "RootNode() should return non-nil node")
	require.Equal(t, "program", root.Kind(), "root node kind should be 'program' for JavaScript source")
}
