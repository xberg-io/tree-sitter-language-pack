// Hand-written capsule-passthrough test (not fixture-generated); preserved across regen.

package io.xberg.treesitterlanguagepack.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import io.xberg.treesitterlanguagepack.TreeSitterLanguagePack;
import io.github.treesitter.jtreesitter.Language;
import io.github.treesitter.jtreesitter.Parser;
import io.github.treesitter.jtreesitter.InputEncoding;

/**
 * Capsule passthrough tests verify that TreeSitterLanguagePack.getLanguage()
 * returns a host-native Language that works with upstream jtreesitter Parser.
 * This confirms #143: language capsule handoff between bindings.
 */
public class CapsulePassthroughTest {

    @Test
    void testPythonLanguagePassthrough() throws Exception {
        // Verify python language is usable with upstream jtreesitter Parser
        Language pythonLang = TreeSitterLanguagePack.getLanguage("python");
        assertNotNull(pythonLang, "getLanguage(\"python\") must not return null");

        try (var parser = new Parser()) {
            parser.setLanguage(pythonLang);
            var tree = parser.parse("def greet(name):\n    return name\n", InputEncoding.UTF_8);
            assertNotNull(tree, "Parser.parse() must not return null");

            var rootNode = tree.get().getRootNode();
            assertNotNull(rootNode, "getRootNode() must not return null");

            assertEquals("module", rootNode.getType(),
                "Python root node type must be 'module'");
        }
    }

    @Test
    void testJavaScriptLanguagePassthrough() throws Exception {
        // Verify javascript language is usable with upstream jtreesitter Parser
        Language jsLang = TreeSitterLanguagePack.getLanguage("javascript");
        assertNotNull(jsLang, "getLanguage(\"javascript\") must not return null");

        try (var parser = new Parser()) {
            parser.setLanguage(jsLang);
            var tree = parser.parse("const x = 1;\n", InputEncoding.UTF_8);
            assertNotNull(tree, "Parser.parse() must not return null");

            var rootNode = tree.get().getRootNode();
            assertNotNull(rootNode, "getRootNode() must not return null");

            assertEquals("program", rootNode.getType(),
                "JavaScript root node type must be 'program'");
        }
    }
}
