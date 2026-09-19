package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class StructureKindTest {

    @Test
    void shouldExposeElevenVariants() {
        assertEquals(11, StructureKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Function", StructureKind.FUNCTION.getValue());
        assertEquals("Method", StructureKind.METHOD.getValue());
        assertEquals("Class", StructureKind.CLASS.getValue());
        assertEquals("Struct", StructureKind.STRUCT.getValue());
        assertEquals("Interface", StructureKind.INTERFACE.getValue());
        assertEquals("Enum", StructureKind.ENUM.getValue());
        assertEquals("Module", StructureKind.MODULE.getValue());
        assertEquals("Trait", StructureKind.TRAIT.getValue());
        assertEquals("Impl", StructureKind.IMPL.getValue());
        assertEquals("Namespace", StructureKind.NAMESPACE.getValue());
        assertEquals("Other", StructureKind.OTHER.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(StructureKind.NAMESPACE, StructureKind.fromValue("namespace"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> StructureKind.fromValue("Macro"));
    }
}
