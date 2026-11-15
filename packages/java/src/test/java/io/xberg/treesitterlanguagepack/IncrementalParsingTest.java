package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.nio.charset.StandardCharsets;
import org.junit.jupiter.api.Test;

class IncrementalParsingTest {

    @Test
    void reusesEditedTreeAndPreservesOriginalCoordinates() throws Exception {
        String before = "x = 'é'\ny = 1\n";
        String after = "x = '🙂'\ny = 1\n";
        InputEdit edit = new InputEdit(5, 7, 9, new Point(0, 5), new Point(0, 7), new Point(0, 9));
        try (Parser parser = TreeSitterLanguagePack.getParser("python");
             Tree original = parser.parse(before).orElseThrow();
             Node originalRoot = original.rootNode(); Tree edited = original.edited(edit);
             Tree incremental = parser.parseIncremental(after, edited).orElseThrow();
             Node incrementalRoot = incremental.rootNode()) {
            assertEquals(before.getBytes(StandardCharsets.UTF_8).length, originalRoot.endByte());
            assertEquals(after.getBytes(StandardCharsets.UTF_8).length, incrementalRoot.endByte());
        }
    }
}
