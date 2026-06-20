// Hand-written capsule-passthrough test (not fixture-generated); preserved across regen.

using System;
using Xunit;
using TreeSitterLanguagePack;

namespace TreeSitterLanguagePack
{
    /// <summary>
    /// Verify that TreeSitterLanguagePackConverter.GetLanguage returns a host-native
    /// TreeSitter.Language usable with the upstream TreeSitter.DotNet parser.
    ///
    /// This tests the capsule passthrough feature (#143): the host-native Language
    /// wrapper allows direct interop with third-party tree-sitter consumers.
    /// </summary>
    public class CapsulePassthroughTests
    {
        [Fact]
        public void Test_ParsePythonWithHostLanguage()
        {
            // Get host-native Language via TreeSitterLanguagePackConverter
            var language = TreeSitterLanguagePackConverter.GetLanguage("python");
            Assert.NotNull(language);

            // Create upstream TreeSitter.DotNet parser
            using var parser = new TreeSitter.Parser();

            // Set the host-native language on the parser
            parser.Language = language;

            // Parse Python source
            var source = "def greet(name):\n    return name\n";
            using var tree = parser.Parse(source);

            var rootNode = tree.RootNode;
            Assert.NotNull(rootNode);

            // Verify the root node type is "module" (Python AST root)
            Assert.Equal("module", rootNode.Type);
        }

        [Fact]
        public void Test_ParseJavascriptWithHostLanguage()
        {
            // Get host-native Language via TreeSitterLanguagePackConverter
            var language = TreeSitterLanguagePackConverter.GetLanguage("javascript");
            Assert.NotNull(language);

            // Create upstream TreeSitter.DotNet parser
            using var parser = new TreeSitter.Parser();

            // Set the host-native language on the parser
            parser.Language = language;

            // Parse JavaScript source
            var source = "const x = 1;\n";
            using var tree = parser.Parse(source);

            var rootNode = tree.RootNode;
            Assert.NotNull(rootNode);

            // Verify the root node type is "program" (JavaScript AST root)
            Assert.Equal("program", rootNode.Type);
        }
    }
}
