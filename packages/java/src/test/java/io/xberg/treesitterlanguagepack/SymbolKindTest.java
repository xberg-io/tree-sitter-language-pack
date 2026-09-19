package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.junit.jupiter.api.Test;

class SymbolKindTest {

    @Test
    void shouldExposeNineVariants() {
        assertEquals(9, SymbolKind.values().length);
    }

    @Test
    void shouldReturnWireFormatValueFromGetValue() {
        assertEquals("Variable", SymbolKind.VARIABLE.getValue());
        assertEquals("Constant", SymbolKind.CONSTANT.getValue());
        assertEquals("Function", SymbolKind.FUNCTION.getValue());
        assertEquals("Class", SymbolKind.CLASS.getValue());
        assertEquals("Type", SymbolKind.TYPE.getValue());
        assertEquals("Interface", SymbolKind.INTERFACE.getValue());
        assertEquals("Enum", SymbolKind.ENUM.getValue());
        assertEquals("Module", SymbolKind.MODULE.getValue());
        assertEquals("Other", SymbolKind.OTHER.getValue());
    }

    @Test
    void shouldResolveFromValueCaseInsensitively() {
        assertEquals(SymbolKind.FUNCTION, SymbolKind.fromValue("FUNCTION"));
    }

    @Test
    void shouldThrowIllegalArgumentExceptionForUnknownValue() {
        assertThrows(IllegalArgumentException.class, () -> SymbolKind.fromValue("Unknown"));
    }
}
