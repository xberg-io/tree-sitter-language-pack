package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class CommentKindTest {

    @Test
    void shouldExposeThreeVariants() {
        assertEquals(3, CommentKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Line", CommentKind.LINE.getValue());
        assertEquals("Block", CommentKind.BLOCK.getValue());
        assertEquals("Doc", CommentKind.DOC.getValue());
    }

    @Test
    void shouldReturnWireFormatValueFromToString() {
        assertEquals("Line", CommentKind.LINE.toString());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(CommentKind.BLOCK, CommentKind.fromValue("block"));
        assertEquals(CommentKind.BLOCK, CommentKind.fromValue("BLOCK"));
        assertEquals(CommentKind.BLOCK, CommentKind.fromValue("Block"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        IllegalArgumentException exception = assertThrows(
            IllegalArgumentException.class, () -> CommentKind.fromValue("NotAKind")
        );

        assertEquals("Unknown CommentKind value: NotAKind", exception.getMessage());
    }
}
