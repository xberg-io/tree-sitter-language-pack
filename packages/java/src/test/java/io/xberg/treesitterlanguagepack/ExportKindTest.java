package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class ExportKindTest {

    @Test
    void shouldExposeThreeVariants() {
        assertEquals(3, ExportKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Named", ExportKind.NAMED.getValue());
        assertEquals("Default", ExportKind.DEFAULT.getValue());
        assertEquals("ReExport", ExportKind.RE_EXPORT.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(ExportKind.RE_EXPORT, ExportKind.fromValue("reexport"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> ExportKind.fromValue("Star"));
    }
}
