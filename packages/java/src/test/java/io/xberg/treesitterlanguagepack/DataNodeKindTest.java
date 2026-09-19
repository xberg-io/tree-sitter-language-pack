package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class DataNodeKindTest {

    @Test
    void shouldExposeThreeVariants() {
        assertEquals(3, DataNodeKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("KeyValue", DataNodeKind.KEY_VALUE.getValue());
        assertEquals("Element", DataNodeKind.ELEMENT.getValue());
        assertEquals("Sequence", DataNodeKind.SEQUENCE.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(DataNodeKind.SEQUENCE, DataNodeKind.fromValue("sequence"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> DataNodeKind.fromValue("Bogus"));
    }
}
