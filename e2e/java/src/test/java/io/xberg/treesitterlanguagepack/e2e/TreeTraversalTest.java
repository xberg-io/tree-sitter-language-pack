// Hand-written regression test for issue #146 (not fixture-generated; preserved across regen).
//
// Exercises the opaque-handle traversal API end-to-end: getParser -> parse ->
// rootNode / walk / cursor.node / child / parent. Before the alef fix, these
// accessors freed the returned native handle immediately after wrapping it, so
// every returned Tree/Node/TreeCursor wrapped an already-freed pointer and the
// next native call crashed the JVM with EXCEPTION_ACCESS_VIOLATION.
//
// Also covers kind() and toSexp() string-return correctness: alef <= 0.24.x
// serialized these through serde_json::to_string, producing a JSON-quoted
// string ("\"module\"" instead of "module"). The JNI shim now passes the raw
// &str / String directly, so these tests would fail with the old code.

package io.xberg.treesitterlanguagepack.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import io.xberg.treesitterlanguagepack.TreeSitterLanguagePack;
import io.xberg.treesitterlanguagepack.Parser;
import io.xberg.treesitterlanguagepack.Tree;
import io.xberg.treesitterlanguagepack.TreeCursor;
import io.xberg.treesitterlanguagepack.Node;

public class TreeTraversalTest {

    @Test
    void testWalkCursorDoesNotCrash() throws Exception {
        // The exact reproduction from issue #146.
        Parser parser = TreeSitterLanguagePack.getParser("python");
        assertNotNull(parser, "getParser(\"python\") must not return null");

        try (Tree tree = parser.parse("myvar = 10").orElseThrow();
             TreeCursor cursor = tree.walk()) {
            assertNotNull(cursor, "tree.walk() must not return null");

            try (Node cursorNode = cursor.node()) {
                assertEquals("module", cursorNode.kind(),
                    "cursor node at root must be the Python 'module'");
            }

            assertTrue(cursor.gotoFirstChild(), "module must have a first child");
            try (Node child = cursor.node()) {
                assertEquals("assignment", child.kind(),
                    "first child of 'myvar = 10' is the assignment statement");
            }
        }
    }

    @Test
    void testRootNodeAndChildTraversalDoesNotCrash() throws Exception {
        Parser parser = TreeSitterLanguagePack.getParser("python");

        try (Tree tree = parser.parse("myvar = 10").orElseThrow();
             Node root = tree.rootNode()) {
            assertEquals("module", root.kind());
            assertTrue(root.childCount() >= 1, "module must have children");

            // child() returns an opaque Node handle — the #146 UAF path.
            try (Node first = root.child(0).orElseThrow()) {
                assertEquals("assignment", first.kind());

                // parent() walks back up via another opaque handle.
                try (Node parent = first.parent().orElseThrow()) {
                    assertEquals("module", parent.kind(),
                        "parent of the assignment must be the module");
                }
            }
        }
    }

    @Test
    void testToSexpReturnsPlainSexpNotJson() throws Exception {
        // Regression: alef <= 0.24.x routed toSexp() through serde_json::to_string,
        // so the return value was a JSON-quoted string: "\"(module ...)\"" instead of
        // "(module ...)". Verify the raw S-expression is returned.
        Parser parser = TreeSitterLanguagePack.getParser("python");

        try (Tree tree = parser.parse("x = 1").orElseThrow();
             Node root = tree.rootNode()) {
            String sexp = root.toSexp();
            assertNotNull(sexp, "toSexp() must not return null");
            assertTrue(sexp.startsWith("("),
                "S-expression must start with '(' — got: " + sexp);
            assertFalse(sexp.startsWith("\""),
                "toSexp() must not be JSON-quoted — got: " + sexp);
            assertTrue(sexp.contains("module"),
                "Python root sexp must contain 'module' — got: " + sexp);
        }
    }

    @Test
    void testKindReturnsPlainStringNotJson() throws Exception {
        // Regression: alef <= 0.24.x routed kind() through serde_json::to_string,
        // returning "\"module\"" instead of "module". This test would pass only with
        // the raw-string JNI path introduced in alef 0.25.47+.
        Parser parser = TreeSitterLanguagePack.getParser("python");

        try (Tree tree = parser.parse("x = 1").orElseThrow();
             Node root = tree.rootNode()) {
            String kind = root.kind();
            assertNotNull(kind, "kind() must not return null");
            assertFalse(kind.startsWith("\""),
                "kind() must not be JSON-quoted — got: " + kind);
            assertEquals("module", kind,
                "Python root node kind must be 'module'");
        }
    }
}
