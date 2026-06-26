// Hand-written regression test for issue #146 (not fixture-generated; preserved across regen).
//
// Exercises the opaque-handle traversal API end-to-end: getParser -> parse ->
// rootNode / walk / cursor.node / child / parent. Before the alef fix, these
// accessors freed the returned native handle immediately after wrapping it, so
// every returned Tree/Node/TreeCursor wrapped an already-freed pointer and the
// next native call crashed the JVM with EXCEPTION_ACCESS_VIOLATION.

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
}
