package io.xberg.treesitterlanguagepack;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import org.junit.jupiter.api.Test;

class SymbolKindTest {

    private static final ObjectMapper MAPPER = new ObjectMapper();

    @Test
    void shouldExposeNineVariants() {
        assertEquals(9, SymbolKind.class.getPermittedSubclasses().length);
    }

    @Test
    void shouldReturnWireFormatValueFromSerialization() throws Exception {
        assertEquals("\"Variable\"", MAPPER.writeValueAsString(new SymbolKind.Variable()));
        assertEquals("\"Constant\"", MAPPER.writeValueAsString(new SymbolKind.Constant()));
        assertEquals("\"Function\"", MAPPER.writeValueAsString(new SymbolKind.Function()));
        assertEquals("\"Class\"", MAPPER.writeValueAsString(new SymbolKind.Class()));
        assertEquals("\"Type\"", MAPPER.writeValueAsString(new SymbolKind.Type()));
        assertEquals("\"Interface\"", MAPPER.writeValueAsString(new SymbolKind.Interface()));
        assertEquals("\"Enum\"", MAPPER.writeValueAsString(new SymbolKind.Enum()));
        assertEquals("\"Module\"", MAPPER.writeValueAsString(new SymbolKind.Module()));
        assertEquals("{\"Other\":\"label\"}", MAPPER.writeValueAsString(new SymbolKind.Other("label")));
    }

    @Test
    void shouldResolveVariantFromWireValue() throws Exception {
        assertEquals(new SymbolKind.Variable(), MAPPER.readValue("\"Variable\"", SymbolKind.class));
        assertEquals(new SymbolKind.Constant(), MAPPER.readValue("\"Constant\"", SymbolKind.class));
        assertEquals(new SymbolKind.Function(), MAPPER.readValue("\"Function\"", SymbolKind.class));
        assertEquals(new SymbolKind.Class(), MAPPER.readValue("\"Class\"", SymbolKind.class));
        assertEquals(new SymbolKind.Type(), MAPPER.readValue("\"Type\"", SymbolKind.class));
        assertEquals(new SymbolKind.Interface(), MAPPER.readValue("\"Interface\"", SymbolKind.class));
        assertEquals(new SymbolKind.Enum(), MAPPER.readValue("\"Enum\"", SymbolKind.class));
        assertEquals(new SymbolKind.Module(), MAPPER.readValue("\"Module\"", SymbolKind.class));
        assertEquals(new SymbolKind.Other("label"), MAPPER.readValue("{\"Other\":\"label\"}", SymbolKind.class));
    }

    @Test
    void shouldThrowJsonProcessingExceptionForUnknownValue() {
        assertThrows(JsonProcessingException.class, () -> MAPPER.readValue("\"Unknown\"", SymbolKind.class));
    }
}
